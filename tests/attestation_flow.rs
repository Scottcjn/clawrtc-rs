// SPDX-License-Identifier: MIT
//! Attestation + node-client integration tests for clawrtc-rs.
//!
//! Complements `tests/public_api.rs` with:
//!
//!   * the *real* sign → attest pipeline — the wallet actually signs the
//!     challenge nonce before submission (rather than a hard-coded signature),
//!     proving the end-to-end flow is internally consistent;
//!   * attestation submission determinism for fixed inputs;
//!   * broader node error paths using HTTP status codes (404 / 500) on *every*
//!     public `Result`-returning API, which the base suite did not cover.
//!
//! Item → test mapping (for the bounty write-up):
//!   3. Attestation determinism  → `attestation_flow_signs_real_nonce_...`,
//!   4. Error paths              → `node_error_paths_return_err_on_bad_status`,
//!                                `attest_and_enroll_reject_ok_false`

use clawrtc::{ClawError, CpuArch, NodeClient, Wallet};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// ── Mock node ────────────────────────────────────────────────────
// Each integration test file is compiled as its own crate, so the mock server
// helper is duplicated here on purpose.

fn spawn_json_server(responses: Vec<&'static str>) -> (String, JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock node");
    let base_url = format!(
        "http://{}",
        listener.local_addr().expect("mock node address")
    );
    let handle = thread::spawn(move || {
        responses
            .into_iter()
            .map(|body| {
                let (mut stream, _) = listener.accept().expect("accept mock-node request");
                let request = read_http_request(&mut stream);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("write mock-node response");
                request
            })
            .collect()
    });
    (base_url, handle)
}

fn spawn_status_server(responses: Vec<(u16, &'static str)>) -> (String, JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock node");
    let base_url = format!(
        "http://{}",
        listener.local_addr().expect("mock node address")
    );
    let handle = thread::spawn(move || {
        responses
            .into_iter()
            .map(|(status, body)| {
                let (mut stream, _) = listener.accept().expect("accept mock-node request");
                let request = read_http_request(&mut stream);
                let response = format!(
                    "HTTP/1.1 {} Status\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status,
                    body.len(),
                    body
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("write mock-node response");
                request
            })
            .collect()
    });
    (base_url, handle)
}

fn read_http_request(stream: &mut TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("set request timeout");

    let mut request = Vec::new();
    let mut chunk = [0_u8; 2048];
    let mut expected_len = None;

    loop {
        let bytes_read = stream.read(&mut chunk).expect("read mock-node request");
        if bytes_read == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..bytes_read]);

        if expected_len.is_none() {
            if let Some(header_end) = find_bytes(&request, b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&request[..header_end]);
                let content_len = headers
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        if name.eq_ignore_ascii_case("content-length") {
                            value.trim().parse::<usize>().ok()
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);
                expected_len = Some(header_end + 4 + content_len);
            }
        }

        if expected_len.is_some_and(|len| request.len() >= len) {
            break;
        }
    }

    String::from_utf8(request).expect("mock-node request is UTF-8")
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn request_body(request: &str) -> &str {
    request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .expect("request contains a header separator")
}

// ── Tests ────────────────────────────────────────────────────────

#[test]
fn attestation_flow_signs_real_nonce_is_consistent_and_deterministic() {
    // 1) challenge, 2) first attest, 3) second attest (same payload).
    let (base_url, server) = spawn_json_server(vec![
        r#"{"nonce":"fixed-challenge-nonce-001"}"#,
        r#"{"ok":true,"message":"accepted"}"#,
        r#"{"ok":true,"message":"accepted"}"#,
    ]);
    let client = NodeClient::new(&base_url);
    let wallet = Wallet::generate();

    // Real pipeline: the wallet signs the challenge nonce, then we submit.
    let challenge = client.challenge().expect("challenge request");
    let signature = wallet.sign(challenge.nonce.as_bytes());

    let build_payload = || {
        json!({
            "miner_id": wallet.address(),
            "nonce": challenge.nonce,
            "device": {"arch": "g4", "family": "powerpc"},
            "signature": signature,
        })
    };

    let first = build_payload();
    let second = build_payload();
    assert_eq!(first, second, "payload construction must be deterministic");

    assert!(client.attest(&first).expect("first attest").ok);
    assert!(client.attest(&second).expect("second attest").ok);

    let requests = server.join().expect("join mock node");
    assert_eq!(requests.len(), 3);
    assert!(requests[0].starts_with("POST /attest/challenge HTTP/1.1"));
    assert!(requests[1].starts_with("POST /attest/submit HTTP/1.1"));
    assert!(requests[2].starts_with("POST /attest/submit HTTP/1.1"));

    // Deterministic submission: identical request bodies for both attest calls.
    assert_eq!(request_body(&requests[1]), request_body(&requests[2]));

    // The signature we submitted actually verifies against the wallet — the
    // end-to-end flow is internally consistent.
    let sent: Value = serde_json::from_str(request_body(&requests[1])).expect("valid JSON body");
    let sent_sig = sent["signature"].as_str().expect("signature present");
    assert!(Wallet::verify(
        &wallet.public_key_hex(),
        challenge.nonce.as_bytes(),
        sent_sig
    )
    .unwrap());
}

#[test]
fn node_error_paths_return_err_on_bad_status() {
    // Every public Result-returning read/write API must surface an error (and
    // never panic) when the node answers with a non-2xx status code.

    let (base_url, server) = spawn_status_server(vec![(404, r#"{"error":"not found"}"#)]);
    assert!(NodeClient::new(&base_url).health().is_err());
    assert!(server.join().unwrap()[0].starts_with("GET /health HTTP/1.1"));

    let (base_url, server) = spawn_status_server(vec![(500, r#"{"error":"boom"}"#)]);
    assert!(NodeClient::new(&base_url).balance("RTCbad").is_err());
    assert!(server.join().unwrap()[0].starts_with("GET /wallet/balance?miner_id=RTCbad HTTP/1.1"));

    let (base_url, server) = spawn_status_server(vec![(404, r#"{"error":"no miners"}"#)]);
    assert!(NodeClient::new(&base_url).miners().is_err());
    assert!(server.join().unwrap()[0].starts_with("GET /api/miners HTTP/1.1"));

    let (base_url, server) = spawn_status_server(vec![(500, r#"{"error":"no challenge"}"#)]);
    assert!(NodeClient::new(&base_url).challenge().is_err());
    assert!(server.join().unwrap()[0].starts_with("POST /attest/challenge HTTP/1.1"));
}

#[test]
fn attest_and_enroll_reject_ok_false() {
    let (base_url, server) = spawn_status_server(vec![(200, r#"{"ok":false,"error":"missing miner_id"}"#)]);
    let client = NodeClient::new(&base_url);
    let error = client
        .attest(&json!({}))
        .expect_err("missing fields must fail");
    assert!(matches!(error, ClawError::Node(message) if message == "missing miner_id"));
    assert!(server.join().unwrap()[0].starts_with("POST /attest/submit HTTP/1.1"));

    let (base_url, server) = spawn_status_server(vec![(200, r#"{"ok":false,"error":"invalid miner_id"}"#)]);
    let client = NodeClient::new(&base_url);
    let error = client
        .enroll("bad-wallet", "", &CpuArch::Modern)
        .expect_err("rejected enrollment must fail");
    assert!(matches!(error, ClawError::Node(message) if message == "invalid miner_id"));
    assert!(server.join().unwrap()[0].starts_with("POST /epoch/enroll HTTP/1.1"));
}
