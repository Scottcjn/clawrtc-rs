use clawrtc::{ClawError, CpuArch, NodeClient, Wallet};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::Duration;

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
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
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

#[test]
fn wallet_roundtrip_sign_verify_and_rejection_paths() {
    let original = Wallet::generate();
    let restored = Wallet::from_hex(&original.private_key_hex()).expect("restore generated wallet");

    assert_eq!(restored.address(), original.address());
    assert_eq!(restored.public_key_hex(), original.public_key_hex());

    let message = b"fixed integration-test message";
    let signature = original.sign(message);
    assert!(Wallet::verify(&original.public_key_hex(), message, &signature).unwrap());
    assert!(!Wallet::verify(
        &original.public_key_hex(),
        b"tampered integration-test message",
        &signature
    )
    .unwrap());

    let mut tampered_signature = signature.into_bytes();
    tampered_signature[0] = if tampered_signature[0] == b'0' {
        b'1'
    } else {
        b'0'
    };
    let tampered_signature = String::from_utf8(tampered_signature).unwrap();
    assert!(!Wallet::verify(&original.public_key_hex(), message, &tampered_signature).unwrap());

    let wrong_wallet = Wallet::generate();
    assert!(!Wallet::verify(
        &wrong_wallet.public_key_hex(),
        message,
        &original.sign(message)
    )
    .unwrap());
}

#[test]
fn rtc_address_vectors_are_stable() {
    // Ed25519 keypairs are the published RFC 8032 test vectors. RTC addresses
    // are SHA-256(public key), prefixed with RTC and truncated to 40 hex chars.
    let vectors = [
        (
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
            "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            "RTC21fe31dfa154a261626bf854046fd2271b7bed4b",
        ),
        (
            "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
            "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
            "RTC39f713d0a644253f04529421b9f51b9b08979d08",
        ),
        (
            "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
            "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
            "RTCdac073e0123bdea59dd9b3bda9cf6037f63aca82",
        ),
    ];

    for (private_key, public_key, address) in vectors {
        let wallet = Wallet::from_hex(private_key).expect("valid RFC 8032 private key");
        assert_eq!(wallet.public_key_hex(), public_key);
        assert_eq!(wallet.address(), address);
    }
}

#[test]
fn wallet_result_apis_reject_invalid_inputs() {
    assert!(matches!(
        Wallet::from_hex("not hex"),
        Err(ClawError::Wallet(_))
    ));
    assert!(matches!(Wallet::from_hex("00"), Err(ClawError::Wallet(_))));
    assert!(matches!(
        Wallet::verify("not hex", b"message", &"00".repeat(64)),
        Err(ClawError::Signing(_))
    ));
    assert!(matches!(
        Wallet::verify(&"00".repeat(32), b"message", "too short"),
        Err(ClawError::Signing(_))
    ));
}

#[test]
fn attestation_is_deterministic_for_fixed_inputs() {
    let (base_url, server) = spawn_json_server(vec![
        r#"{"ok":true,"message":"accepted"}"#,
        r#"{"ok":true,"message":"accepted"}"#,
    ]);
    let client = NodeClient::new(&base_url);
    let fixed_payload = || {
        json!({
            "miner_id": "RTC21fe31dfa154a261626bf854046fd2271b7bed4b",
            "nonce": "fixed-nonce-001",
            "device": {"arch": "g4", "family": "powerpc"},
            "signature": "ab".repeat(64),
        })
    };

    let first_payload = fixed_payload();
    let second_payload = fixed_payload();
    assert_eq!(first_payload, second_payload);
    assert!(client.attest(&first_payload).unwrap().ok);
    assert!(client.attest(&second_payload).unwrap().ok);

    let requests = server.join().expect("join mock node");
    assert_eq!(requests.len(), 2);
    assert!(requests[0].starts_with("POST /attest/submit HTTP/1.1"));
    assert_eq!(request_body(&requests[0]), request_body(&requests[1]));
    assert_eq!(
        serde_json::from_str::<Value>(request_body(&requests[0])).unwrap(),
        first_payload
    );
}

#[test]
fn malformed_attestation_returns_error_without_panicking() {
    let (base_url, server) = spawn_json_server(vec![r#"{"ok":false,"error":"missing miner_id"}"#]);
    let client = NodeClient::new(&base_url);

    let error = client
        .attest(&json!({}))
        .expect_err("missing fields must fail");
    assert!(matches!(error, ClawError::Node(message) if message == "missing miner_id"));

    let requests = server.join().expect("join mock node");
    assert_eq!(request_body(&requests[0]), "{}");
}

#[test]
fn node_read_result_apis_reject_invalid_json() {
    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    assert!(NodeClient::new(&base_url).health().is_err());
    assert!(server.join().unwrap()[0].starts_with("GET /health HTTP/1.1"));

    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    assert!(NodeClient::new(&base_url).balance("RTCbad").is_err());
    assert!(server.join().unwrap()[0].starts_with("GET /wallet/balance?miner_id=RTCbad HTTP/1.1"));

    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    assert!(NodeClient::new(&base_url).miners().is_err());
    assert!(server.join().unwrap()[0].starts_with("GET /api/miners HTTP/1.1"));

    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    assert!(NodeClient::new(&base_url).challenge().is_err());
    assert!(server.join().unwrap()[0].starts_with("POST /attest/challenge HTTP/1.1"));
}

#[test]
fn enroll_returns_node_error_for_rejected_input() {
    let (base_url, server) = spawn_json_server(vec![r#"{"ok":false,"error":"invalid miner_id"}"#]);
    let client = NodeClient::new(&base_url);

    let error = client
        .enroll("bad-wallet", "", &CpuArch::Modern)
        .expect_err("rejected enrollment must fail");
    assert!(matches!(error, ClawError::Node(message) if message == "invalid miner_id"));

    let requests = server.join().expect("join mock node");
    assert!(requests[0].starts_with("POST /epoch/enroll HTTP/1.1"));
}
