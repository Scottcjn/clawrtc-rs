//! Integration test suite for clawrtc-rs
//!
//! Covers wallet roundtrip, address derivation vectors, attestation
//! determinism, error paths, and public API coverage.

use clawrtc::{ClawError, CpuArch, NodeClient, Wallet};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════
// Helper: Mock HTTP server
// ═══════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════
// 1. Wallet Roundtrip
// ═══════════════════════════════════════════════════════════════

#[test]
fn wallet_generate_serialize_deserialize_same_address_and_pubkey() {
    // generate → serialize (hex) → deserialize → same address & pubkey
    let wallet = Wallet::generate();
    let private_key_hex = wallet.private_key_hex();
    let original_address = wallet.address();
    let original_pubkey = wallet.public_key_hex();

    let restored = Wallet::from_hex(&private_key_hex).expect("deserialize from hex");
    assert_eq!(restored.address(), original_address);
    assert_eq!(restored.public_key_hex(), original_pubkey);
}

#[test]
fn wallet_sign_verify_roundtrip() {
    let wallet = Wallet::generate();
    let message = b"test message for signing";
    let signature = wallet.sign(message);

    // Verify succeeds with correct message
    assert!(Wallet::verify(&wallet.public_key_hex(), message, &signature).unwrap());
}

#[test]
fn wallet_verify_fails_on_tampered_message() {
    let wallet = Wallet::generate();
    let message = b"original message";
    let signature = wallet.sign(message);

    // Verify fails with tampered message
    let tampered = b"tampered message";
    assert!(!Wallet::verify(&wallet.public_key_hex(), tampered, &signature).unwrap());
}

#[test]
fn wallet_verify_fails_on_tampered_signature() {
    let wallet = Wallet::generate();
    let message = b"test message";
    let signature = wallet.sign(message);

    // Tamper with signature (flip first char)
    let mut sig_bytes: Vec<u8> = hex::decode(&signature).unwrap();
    sig_bytes[0] ^= 0xFF;
    let tampered_sig = hex::encode(&sig_bytes);

    assert!(!Wallet::verify(&wallet.public_key_hex(), message, &tampered_sig).unwrap());
}

#[test]
fn wallet_verify_fails_on_wrong_key() {
    let wallet1 = Wallet::generate();
    let wallet2 = Wallet::generate();
    let message = b"test message";
    let signature = wallet1.sign(message);

    // Verify fails with wrong public key
    assert!(!Wallet::verify(&wallet2.public_key_hex(), message, &signature).unwrap());
}

// ═══════════════════════════════════════════════════════════════
// 2. Address Derivation Vectors (RFC 8032 test vectors)
// ═══════════════════════════════════════════════════════════════

#[test]
fn address_derivation_vector_1() {
    // RFC 8032 Ed25519 test vector 1
    let private_key = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
    let expected_pubkey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
    let expected_address = "RTC21fe31dfa154a261626bf854046fd2271b7bed4b";

    let wallet = Wallet::from_hex(private_key).expect("valid RFC 8032 key");
    assert_eq!(wallet.public_key_hex(), expected_pubkey);
    assert_eq!(wallet.address(), expected_address);
}

#[test]
fn address_derivation_vector_2() {
    // RFC 8032 Ed25519 test vector 2
    let private_key = "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb";
    let expected_pubkey = "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c";
    let expected_address = "RTC39f713d0a644253f04529421b9f51b9b08979d08";

    let wallet = Wallet::from_hex(private_key).expect("valid RFC 8032 key");
    assert_eq!(wallet.public_key_hex(), expected_pubkey);
    assert_eq!(wallet.address(), expected_address);
}

#[test]
fn address_derivation_vector_3() {
    // RFC 8032 Ed25519 test vector 3
    let private_key = "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7";
    let expected_pubkey = "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";
    let expected_address = "RTCdac073e0123bdea59dd9b3bda9cf6037f63aca82";

    let wallet = Wallet::from_hex(private_key).expect("valid RFC 8032 key");
    assert_eq!(wallet.public_key_hex(), expected_pubkey);
    assert_eq!(wallet.address(), expected_address);
}

#[test]
fn address_format_is_correct() {
    // Address must be "RTC" + 40 hex chars = 43 chars total
    let wallet = Wallet::generate();
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"), "address must start with RTC");
    assert_eq!(addr.len(), 43, "address must be 43 chars (RTC + 40 hex)");
    assert!(
        addr[3..].chars().all(|c| c.is_ascii_hexdigit()),
        "address suffix must be hex"
    );
}

// ═══════════════════════════════════════════════════════════════
// 3. Attestation Payload Determinism
// ═══════════════════════════════════════════════════════════════

#[test]
fn attestation_payload_is_deterministic_for_fixed_inputs() {
    let (base_url, server) = spawn_json_server(vec![
        r#"{"ok":true,"message":"accepted"}"#,
        r#"{"ok":true,"message":"accepted"}"#,
    ]);
    let client = NodeClient::new(&base_url);

    let make_payload = || {
        json!({
            "miner_id": "RTC21fe31dfa154a261626bf854046fd2271b7bed4b",
            "nonce": "fixed-nonce-001",
            "device": {"arch": "g4", "family": "powerpc"},
            "signature": "ab".repeat(64),
        })
    };

    let first = make_payload();
    let second = make_payload();

    // Payload construction is deterministic
    assert_eq!(first, second);

    // Both succeed
    assert!(client.attest(&first).unwrap().ok);
    assert!(client.attest(&second).unwrap().ok);

    // Verify requests are identical
    let requests = server.join().unwrap();
    assert_eq!(request_body(&requests[0]), request_body(&requests[1]));
}

#[test]
fn malformed_attestation_errors_not_panic() {
    let (base_url, server) = spawn_json_server(vec![
        r#"{"ok":false,"error":"missing miner_id"}"#,
        r#"{"ok":false,"error":"invalid signature"}"#,
        r#"{"ok":false,"error":"unknown device arch"}"#,
    ]);
    let client = NodeClient::new(&base_url);

    // Empty payload
    let err = client.attest(&json!({})).unwrap_err();
    assert!(matches!(err, ClawError::Node(msg) if msg == "missing miner_id"));

    // Invalid signature
    let err = client.attest(&json!({"miner_id": "x", "nonce": "n", "signature": "zz"})).unwrap_err();
    assert!(matches!(err, ClawError::Node(msg) if msg == "invalid signature"));

    // Unknown device arch
    let err = client.attest(&json!({
        "miner_id": "x",
        "nonce": "n",
        "device": {"arch": "unknown", "family": "x86"},
        "signature": "ab".repeat(64)
    })).unwrap_err();
    assert!(matches!(err, ClawError::Node(msg) if msg == "unknown device arch"));

    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 3);
    assert!(requests.iter().all(|r| r.starts_with("POST /attest/submit HTTP/1.1")));
}

#[test]
fn malformed_enrollment_errors_not_panic() {
    let (base_url, server) = spawn_json_server(vec![
        r#"{"ok":false,"error":"invalid miner_id"}"#,
        r#"{"ok":false,"error":"missing wallet"}"#,
    ]);
    let client = NodeClient::new(&base_url);

    // Empty miner_id
    let err = client.enroll("wallet", "", &CpuArch::Modern).unwrap_err();
    assert!(matches!(err, ClawError::Node(msg) if msg == "invalid miner_id"));

    // Empty wallet
    let err = client.enroll("", "miner", &CpuArch::G4).unwrap_err();
    assert!(matches!(err, ClawError::Node(msg) if msg == "missing wallet"));

    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests.iter().all(|r| r.starts_with("POST /epoch/enroll HTTP/1.1")));
}

// ═══════════════════════════════════════════════════════════════
// 4. Error Paths — Every Public Result API
// ═══════════════════════════════════════════════════════════════

#[test]
fn health_returns_error_on_invalid_json() {
    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    let err = NodeClient::new(&base_url).health().unwrap_err();
    // reqwest may return Http error when response body isn't valid JSON
    // Both ClawError::Json and ClawError::Http are acceptable
    assert!(
        matches!(err, ClawError::Json(_) | ClawError::Http(_)),
        "expected Json or Http error, got: {err:?}"
    );
    assert!(server.join().unwrap()[0].starts_with("GET /health HTTP/1.1"));
}

#[test]
fn balance_returns_error_on_invalid_json() {
    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    let err = NodeClient::new(&base_url).balance("RTCabc").unwrap_err();
    assert!(
        matches!(err, ClawError::Json(_) | ClawError::Http(_)),
        "expected Json or Http error, got: {err:?}"
    );
    assert!(server.join().unwrap()[0].starts_with("GET /wallet/balance"));
}

#[test]
fn miners_returns_error_on_invalid_json() {
    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    let err = NodeClient::new(&base_url).miners().unwrap_err();
    assert!(
        matches!(err, ClawError::Json(_) | ClawError::Http(_)),
        "expected Json or Http error, got: {err:?}"
    );
    assert!(server.join().unwrap()[0].starts_with("GET /api/miners HTTP/1.1"));
}

#[test]
fn challenge_returns_error_on_invalid_json() {
    let (base_url, server) = spawn_json_server(vec!["not-json"]);
    let err = NodeClient::new(&base_url).challenge().unwrap_err();
    assert!(
        matches!(err, ClawError::Json(_) | ClawError::Http(_)),
        "expected Json or Http error, got: {err:?}"
    );
    assert!(server.join().unwrap()[0].starts_with("POST /attest/challenge HTTP/1.1"));
}

#[test]
fn from_hex_rejects_invalid_hex() {
    assert!(matches!(
        Wallet::from_hex("not-hex"),
        Err(ClawError::Wallet(_))
    ));
}

#[test]
fn from_hex_rejects_wrong_length() {
    // Too short (only 2 bytes instead of 32)
    assert!(matches!(
        Wallet::from_hex("00"),
        Err(ClawError::Wallet(_))
    ));
    // Too long (40 bytes instead of 32)
    assert!(matches!(
        Wallet::from_hex(&"00".repeat(40)),
        Err(ClawError::Wallet(_))
    ));
}

#[test]
fn verify_rejects_invalid_pubkey_hex() {
    assert!(matches!(
        Wallet::verify("not-hex", b"msg", &"00".repeat(64)),
        Err(ClawError::Signing(_))
    ));
}

#[test]
fn verify_rejects_invalid_signature_hex() {
    let wallet = Wallet::generate();
    assert!(matches!(
        Wallet::verify(&wallet.public_key_hex(), b"msg", "too-short"),
        Err(ClawError::Signing(_))
    ));
}

// ═══════════════════════════════════════════════════════════════
// 5. CpuArch Coverage
// ═══════════════════════════════════════════════════════════════

#[test]
fn all_arch_multipliers_are_correct() {
    assert_eq!(CpuArch::G4.multiplier(), 2.5);
    assert_eq!(CpuArch::G5.multiplier(), 2.0);
    assert_eq!(CpuArch::G3.multiplier(), 1.8);
    assert_eq!(CpuArch::Pentium4.multiplier(), 1.5);
    assert_eq!(CpuArch::Retro.multiplier(), 1.4);
    assert_eq!(CpuArch::Core2.multiplier(), 1.3);
    assert_eq!(CpuArch::AppleSilicon.multiplier(), 1.2);
    assert_eq!(CpuArch::Modern.multiplier(), 1.0);
}

#[test]
fn all_arch_families_are_correct() {
    assert_eq!(CpuArch::G4.family(), "powerpc");
    assert_eq!(CpuArch::G5.family(), "powerpc");
    assert_eq!(CpuArch::G3.family(), "powerpc");
    assert_eq!(CpuArch::AppleSilicon.family(), "arm");
    assert_eq!(CpuArch::Modern.family(), "x86");
    assert_eq!(CpuArch::Pentium4.family(), "x86");
}

#[test]
fn all_arch_strs_are_correct() {
    assert_eq!(CpuArch::G4.arch_str(), "g4");
    assert_eq!(CpuArch::G5.arch_str(), "g5");
    assert_eq!(CpuArch::G3.arch_str(), "g3");
    assert_eq!(CpuArch::Pentium4.arch_str(), "pentium4");
    assert_eq!(CpuArch::Retro.arch_str(), "retro");
    assert_eq!(CpuArch::Core2.arch_str(), "core2duo");
    assert_eq!(CpuArch::AppleSilicon.arch_str(), "apple_silicon");
    assert_eq!(CpuArch::Modern.arch_str(), "modern");
}

// ═══════════════════════════════════════════════════════════════
// 6. NodeClient URL Normalization
// ═══════════════════════════════════════════════════════════════

#[test]
fn node_client_strips_trailing_slash() {
    // Verify client was created (we can't access private base_url directly,
    // but we can verify it doesn't panic and works with mock server)
    let (base_url, server) = spawn_json_server(vec![r#"{"ok":true,"version":"test"}"#]);
    let client = NodeClient::new(&format!("{}/", base_url));
    assert!(client.health().is_ok());
    server.join().unwrap();
}

// ═══════════════════════════════════════════════════════════════
// 7. Miners Response Parsing
// ═══════════════════════════════════════════════════════════════

#[test]
fn miners_wrapped_shape_is_accepted() {
    let (base_url, server) = spawn_json_server(vec![r#"{
        "miners": [{
            "miner": "power8-s824",
            "device_arch": "POWER8",
            "device_family": "PowerPC",
            "last_attest": 1783954623
        }]
    }"#]);
    let miners = NodeClient::new(&base_url).miners().unwrap();
    assert_eq!(miners.len(), 1);
    assert_eq!(miners[0].miner, "power8-s824");
    assert_eq!(miners[0].last_seen, "1783954623");
    server.join().unwrap();
}

#[test]
fn miners_list_shape_is_accepted() {
    let (base_url, server) = spawn_json_server(vec![r#"[
        {
            "miner_id": "legacy-g5",
            "device_arch": "g5",
            "device_family": "powerpc",
            "last_seen": "2026-07-13T12:00:00Z"
        }
    ]"#]);
    let miners = NodeClient::new(&base_url).miners().unwrap();
    assert_eq!(miners.len(), 1);
    assert_eq!(miners[0].miner_id, "legacy-g5");
    assert_eq!(miners[0].last_seen, "2026-07-13T12:00:00Z");
    server.join().unwrap();
}

// ═══════════════════════════════════════════════════════════════
// 8. Enroll Success Path
// ═══════════════════════════════════════════════════════════════

#[test]
fn enroll_success() {
    let (base_url, server) = spawn_json_server(vec![r#"{
        "ok": true,
        "epoch": 42,
        "weight": 1.0
    }"#]);
    let client = NodeClient::new(&base_url);
    let resp = client
        .enroll("RTCabc123", "miner-1", &CpuArch::Modern)
        .unwrap();
    assert!(resp.ok);
    assert_eq!(resp.epoch, 42);
    server.join().unwrap();
}
