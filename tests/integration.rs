// SPDX-License-Identifier: MIT
//! Integration test suite for clawrtc-rs.
//!
//! Coverage:
//! - Wallet roundtrip: generate -> serialize -> deserialize -> same address & pubkey;
//!   sign -> verify; verify fails on tampered message / sig / key
//! - Address derivation vectors: fixed keypairs with expected RTC… addresses
//! - Attestation payload determinism: same inputs -> same output;
//!   malformed/missing fields produce errors, not panics
//! - Error paths: every public Result-returning API exercised with a failing input
//! - CpuArch edge cases

use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use clawrtc::{ClawError, CpuArch, NodeClient, Wallet};
use serde_json::json;
use sha2::{Digest, Sha256};

// ── utility: compute expected address from raw private key ────

fn expected_address(priv_key: &[u8; 32]) -> String {
    let sk = ed25519_dalek::SigningKey::from_bytes(priv_key);
    let pk = sk.verifying_key().to_bytes();
    let hash = Sha256::digest(pk);
    format!("RTC{}", &hex::encode(hash)[..40])
}

// ── utility: run a mock HTTP server, returning (shutdown_fn, base_url) ──
//
// The returned function consumes the listener (dropping it ceases
// the accept loop), keeping the thread alive for the test duration.

type ShutdownFn = Box<dyn FnOnce() + Send>;

fn mock_server(body: &str, status_line: &str) -> (ShutdownFn, String) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let owned_body = body.to_string();
    let owned_status = status_line.to_string();

    // We wrap the listener so the closure moves it, then we send it
    // back out via a channel so the test can drop it after the request.
    let (tx, rx) = std::sync::mpsc::channel::<TcpListener>();

    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut _buf = [0_u8; 4096];
            let _ = stream.read(&mut _buf);
            let response = format!(
                "HTTP/1.1 {}\r\n\
                 Content-Type: application/json\r\n\
                 Content-Length: {}\r\n\
                 Connection: close\r\n\r\n{}",
                owned_status,
                owned_body.len(),
                owned_body
            );
            let _ = stream.write_all(response.as_bytes());
        }
        // Send listener back to be dropped on the test thread
        let _ = tx.send(listener);
    });

    let shutdown: ShutdownFn = Box::new(move || {
        if let Ok(l) = rx.recv_timeout(std::time::Duration::from_secs(5)) {
            drop(l);
        }
    });

    (shutdown, base_url)
}

// ═══════════════════════════════════════════════════════════════
// 1. Wallet roundtrip
// ═══════════════════════════════════════════════════════════════

#[test]
fn wallet_roundtrip_generate_serialize_deserialize() {
    let w1 = Wallet::generate();
    let priv_hex = w1.private_key_hex();
    let addr1 = w1.address();
    let pk1 = w1.public_key_hex();

    // Restore from hex
    let w2 = Wallet::from_hex(&priv_hex).expect("from_hex should succeed for valid key");
    assert_eq!(w2.address(), addr1, "address must match after from_hex");
    assert_eq!(
        w2.public_key_hex(),
        pk1,
        "pubkey must match after from_hex"
    );
    assert_eq!(
        w2.private_key_hex(),
        priv_hex,
        "private key hex must roundtrip"
    );

    // Restore from raw bytes
    let raw = hex::decode(&priv_hex).unwrap();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&raw);
    let w3 = Wallet::from_private_key(&arr);
    assert_eq!(
        w3.address(),
        addr1,
        "address must match from_private_key"
    );
    assert_eq!(
        w3.public_key_hex(),
        pk1,
        "pubkey must match from_private_key"
    );
}

#[test]
fn wallet_sign_then_verify_roundtrip() {
    let wallet = Wallet::generate();
    let msg = b"RustChain transfer: 42 RTC to RTCabc123def456";
    let sig = wallet.sign(msg);

    let valid = Wallet::verify(&wallet.public_key_hex(), msg, &sig)
        .expect("verify should not error on valid inputs");
    assert!(valid, "fresh signature must verify");
}

#[test]
fn wallet_verify_rejects_tampered_message() {
    let wallet = Wallet::generate();
    let msg = b"original message";
    let sig = wallet.sign(msg);

    let tampered = b"tampered message";
    let valid = Wallet::verify(&wallet.public_key_hex(), tampered, &sig)
        .expect("verify should not error");
    assert!(!valid, "tampered message must NOT verify");
}

#[test]
fn wallet_verify_rejects_tampered_signature() {
    let wallet = Wallet::generate();
    let msg = b"some message";
    let sig = wallet.sign(msg);

    let mut sig_bytes = hex::decode(&sig).unwrap();
    sig_bytes[0] ^= 0xff; // flip all bits in first byte
    let bad_sig = hex::encode(&sig_bytes);

    let valid = Wallet::verify(&wallet.public_key_hex(), msg, &bad_sig)
        .expect("verify should not error");
    assert!(!valid, "tampered signature must NOT verify");
}

#[test]
fn wallet_verify_rejects_wrong_key() {
    let wallet_a = Wallet::generate();
    let wallet_b = Wallet::generate();
    let msg = b"message signed by A";
    let sig = wallet_a.sign(msg);

    let valid = Wallet::verify(&wallet_b.public_key_hex(), msg, &sig)
        .expect("verify should not error");
    assert!(!valid, "wrong public key must NOT verify");
}

// ═══════════════════════════════════════════════════════════════
// 2. Address derivation vectors (regression vectors)
// ═══════════════════════════════════════════════════════════════

#[test]
fn address_vector_all_ones() {
    let priv_key = [0x01u8; 32];
    let wallet = Wallet::from_private_key(&priv_key);
    let expected = expected_address(&priv_key);
    assert_eq!(
        wallet.address(),
        expected,
        "address derivation for all-0x01 key changed"
    );
    assert_eq!(wallet.address().len(), 43, "address must be 43 chars");
    assert!(
        wallet.address().starts_with("RTC"),
        "address must start with RTC"
    );
    // Log the actual address for documentation
    eprintln!("address_vector_all_ones: {}", wallet.address());
}

#[test]
fn address_vector_all_0x42() {
    let priv_key = [0x42u8; 32];
    let wallet = Wallet::from_private_key(&priv_key);
    let expected = expected_address(&priv_key);
    assert_eq!(
        wallet.address(),
        expected,
        "address derivation for all-0x42 key changed"
    );
    eprintln!("address_vector_all_0x42: {}", wallet.address());
}

#[test]
fn address_vector_counting() {
    let mut priv_key = [0u8; 32];
    for (i, byte) in priv_key.iter_mut().enumerate() {
        *byte = i as u8;
    }
    let wallet = Wallet::from_private_key(&priv_key);
    let expected = expected_address(&priv_key);
    assert_eq!(
        wallet.address(),
        expected,
        "address derivation for counting-pattern key changed"
    );
    eprintln!("address_vector_counting: {}", wallet.address());
}

#[test]
fn address_vector_alternating() {
    let mut priv_key = [0u8; 32];
    for (i, byte) in priv_key.iter_mut().enumerate() {
        *byte = if i % 2 == 0 { 0xaa } else { 0x55 };
    }
    let wallet = Wallet::from_private_key(&priv_key);
    let expected = expected_address(&priv_key);
    assert_eq!(
        wallet.address(),
        expected,
        "address derivation for alternating-pattern key changed"
    );
    eprintln!("address_vector_alternating: {}", wallet.address());
}

// ═══════════════════════════════════════════════════════════════
// 3. Attestation payload determinism
// ═══════════════════════════════════════════════════════════════

#[test]
fn attest_payload_serialization_determinism() {
    let payload_a = json!({
        "nonce": "abc123",
        "miner_pubkey": "RTCTestAddress",
        "device": { "family": "powerpc", "arch": "g4" },
        "signature": "deadbeef"
    });
    let payload_b = json!({
        "nonce": "abc123",
        "miner_pubkey": "RTCTestAddress",
        "device": { "family": "powerpc", "arch": "g4" },
        "signature": "deadbeef"
    });

    assert_eq!(
        serde_json::to_string(&payload_a).unwrap(),
        serde_json::to_string(&payload_b).unwrap(),
        "same JSON inputs must produce same serialization"
    );
}

#[test]
fn challenge_deserialize_valid() {
    let challenge: clawrtc::Challenge =
        serde_json::from_str(r#"{"nonce":"abc123def456"}"#).unwrap();
    assert_eq!(challenge.nonce, "abc123def456");
}

#[test]
fn challenge_deserialize_missing_nonce_is_error() {
    let result: Result<clawrtc::Challenge, _> = serde_json::from_str(r#"{}"#);
    assert!(
        result.is_err(),
        "missing nonce field should produce error, not panic"
    );
}

#[test]
fn challenge_deserialize_with_extra_fields_does_not_panic() {
    let challenge: clawrtc::Challenge =
        serde_json::from_str(r#"{"nonce":"abc","extra":"ignored","nested":{"a":1}}"#).unwrap();
    assert_eq!(
        challenge.nonce, "abc",
        "extra fields must be silently ignored"
    );
}

#[test]
fn attest_response_deserialize_valid() {
    let resp: clawrtc::AttestResponse =
        serde_json::from_str(r#"{"ok":true,"message":"attestation recorded"}"#).unwrap();
    assert!(resp.ok);
    assert_eq!(resp.message, "attestation recorded");
}

#[test]
fn attest_response_deserialize_ok_false_is_not_a_panic() {
    let resp: clawrtc::AttestResponse =
        serde_json::from_str(r#"{"ok":false,"message":"invalid proof"}"#).unwrap();
    assert!(!resp.ok);
    assert_eq!(resp.message, "invalid proof");
}

#[test]
fn enroll_response_deserialize_valid() {
    let resp: clawrtc::EnrollResponse =
        serde_json::from_str(r#"{"ok":true,"epoch":42,"weight":2.5}"#).unwrap();
    assert!(resp.ok);
    assert_eq!(resp.epoch, 42);
    assert_eq!(resp.weight, 2.5);
}

#[test]
fn enroll_response_deserialize_missing_fields_default_to_zero() {
    let resp: clawrtc::EnrollResponse =
        serde_json::from_str(r#"{"ok":true}"#).unwrap();
    assert!(resp.ok);
    assert_eq!(resp.epoch, 0, "missing epoch should default to 0");
    assert_eq!(resp.weight, 0.0, "missing weight should default to 0.0");
}

#[test]
fn challenge_wrong_type_for_nonce_is_error() {
    let result: Result<clawrtc::Challenge, _> =
        serde_json::from_str(r#"{"nonce":12345}"#);
    assert!(result.is_err(), "non-string nonce should fail, not panic");
}

// ═══════════════════════════════════════════════════════════════
// 4. Error paths
// ═══════════════════════════════════════════════════════════════

// ── Wallet::from_hex ──────────────────────────────────────────

#[test]
fn wallet_from_hex_rejects_invalid_hex_chars() {
    let err = Wallet::from_hex("not-hex-at-all").err()
        .expect("from_hex should return Err for invalid hex");
    assert!(
        matches!(err, ClawError::Wallet(_)),
        "expected Wallet error, got {err:?}"
    );
    assert!(
        format!("{err}").contains("invalid hex"),
        "error should mention hex formatting: {err}"
    );
}

#[test]
fn wallet_from_hex_rejects_wrong_length() {
    let err = Wallet::from_hex("aabbccddee").err()
        .expect("from_hex should return Err for short key");
    assert!(
        matches!(err, ClawError::Wallet(_)),
        "expected Wallet error, got {err:?}"
    );
    assert!(
        format!("{err}").contains("32 bytes"),
        "error should mention length requirement: {err}"
    );
}

#[test]
fn wallet_from_hex_rejects_empty_string() {
    assert!(
        Wallet::from_hex("").is_err(),
        "from_hex should reject empty string"
    );
}

#[test]
fn wallet_from_hex_rejects_odd_length_hex() {
    assert!(
        Wallet::from_hex("aabbccddee0").is_err(),
        "from_hex should reject odd-length hex"
    );
}

// ── Wallet::verify ────────────────────────────────────────────

#[test]
fn wallet_verify_rejects_bad_pubkey_hex() {
    let err = Wallet::verify("not-hex", b"msg", "aabb").err()
        .expect("verify should return Err for bad pubkey hex");
    assert!(
        matches!(err, ClawError::Signing(_)),
        "expected Signing error, got {err:?}"
    );
}

#[test]
fn wallet_verify_rejects_wrong_pubkey_length() {
    let sig = "00".repeat(64);
    let err = Wallet::verify("aabbccddee11223344556677889900aabbcc", b"msg", &sig)
        .err()
        .expect("verify should return Err for short pubkey");
    assert!(
        matches!(err, ClawError::Signing(_)),
        "expected Signing error for wrong-length pubkey, got {err:?}"
    );
}

#[test]
fn wallet_verify_rejects_bad_signature_hex() {
    let wallet = Wallet::generate();
    let err = Wallet::verify(&wallet.public_key_hex(), b"msg", "not-hex-sig")
        .err()
        .expect("verify should return Err for bad sig hex");
    assert!(
        matches!(err, ClawError::Signing(_)),
        "expected Signing error, got {err:?}"
    );
}

#[test]
fn wallet_verify_rejects_wrong_signature_length() {
    let wallet = Wallet::generate();
    let err = Wallet::verify(&wallet.public_key_hex(), b"msg", "aa")
        .err()
        .expect("verify should return Err for short sig");
    assert!(
        matches!(err, ClawError::Signing(_)),
        "expected Signing error, got {err:?}"
    );
    assert!(
        format!("{err}").contains("64 bytes") || format!("{err}").contains("signature"),
        "error should mention signature length: {err}"
    );
}

#[test]
fn wallet_verify_rejects_invalid_pubkey_point() {
    // ed25519-dalek's from_bytes accepts any 32 bytes; point validation
    // happens in verify_strict, which returns verify_strict().is_ok() = false
    let bad_pubkey = "ff".repeat(32);
    let sig = "00".repeat(64);
    let result = Wallet::verify(&bad_pubkey, b"msg", &sig)
        .expect("verify should return Ok even for invalid point (validation deferred)");
    assert!(!result, "verify_strict should reject invalid curve point");
}

#[test]
fn wallet_verify_rejects_empty_pubkey() {
    let wallet = Wallet::generate();
    let msg = b"test message";
    let sig = wallet.sign(msg);

    // Empty pubkey hex should fail
    let err = Wallet::verify("", msg, &sig)
        .err()
        .expect("verify should return Err for empty pubkey");
    assert!(
        matches!(err, ClawError::Signing(_)),
        "empty pubkey should produce error"
    );

    // But empty message should still verify (Ed25519 supports it)
    let empty_msg = b"";
    let empty_sig = wallet.sign(empty_msg);
    let valid = Wallet::verify(&wallet.public_key_hex(), empty_msg, &empty_sig)
        .expect("verify with empty message should not error");
    assert!(valid, "empty message should verify");
}

// ── NodeClient mock-based error paths ─────────────────────────

#[test]
fn node_client_health_returns_ok_on_valid_response() {
    let (_shutdown, base_url) = mock_server(
        r#"{"ok":true,"version":"0.2.0","uptime_s":3600.0,"db_rw":true}"#,
        "200 OK",
    );
    let client = NodeClient::new(&base_url);
    let health = client.health().expect("health should succeed");
    assert!(health.ok);
    assert_eq!(health.version, "0.2.0");
    assert!((health.uptime_s - 3600.0).abs() < f64::EPSILON);
    assert!(health.db_rw);
    drop(_shutdown);
}

#[test]
fn node_client_health_returns_error_on_malformed_response() {
    let (_shutdown, base_url) = mock_server("not-json-at-all", "200 OK");
    let client = NodeClient::new(&base_url);
    let err = client.health().err()
        .expect("health should return Err for malformed JSON");
    assert!(
        matches!(err, ClawError::Http(_)) || matches!(err, ClawError::Json(_)),
        "expected Http or Json error for malformed response, got {err:?}"
    );
    drop(_shutdown);
}

#[test]
fn node_client_balance_defaults_when_field_missing() {
    // Response has no balance_rtc or amount_rtc — field has #[serde(default)]
    let (_shutdown, base_url) = mock_server(r#"{"wrong_field":42}"#, "200 OK");
    let client = NodeClient::new(&base_url);
    let balance = client.balance("RTCtest")
        .expect("balance should succeed with default value");
    assert_eq!(balance, 0.0, "missing balance field should default to 0.0");
    drop(_shutdown);
}

#[test]
fn node_client_attest_returns_error_on_node_failure() {
    let (_shutdown, base_url) = mock_server(
        r#"{"ok":false,"error":"invalid attestation proof"}"#,
        "200 OK",
    );
    let client = NodeClient::new(&base_url);
    let payload = json!({ "nonce": "test" });
    let err = client.attest(&payload).err()
        .expect("attest should return Err for failed attestation");
    assert!(
        matches!(err, ClawError::Node(_)),
        "expected Node error for failed attestation, got {err:?}"
    );
    assert!(
        format!("{err}").contains("invalid attestation proof"),
        "should include server error message: {err}"
    );
    drop(_shutdown);
}

#[test]
fn node_client_attest_returns_error_when_ok_field_missing() {
    let (_shutdown, base_url) = mock_server(r#"{"status":"ok"}"#, "200 OK");
    let client = NodeClient::new(&base_url);
    let payload = json!({ "nonce": "test" });
    let err = client.attest(&payload).err()
        .expect("attest should return Err when 'ok' field is missing");
    assert!(
        matches!(err, ClawError::Node(_)),
        "expected Node error when 'ok' field is missing, got {err:?}"
    );
    drop(_shutdown);
}

#[test]
fn node_client_enroll_returns_error_on_node_failure() {
    let (_shutdown, base_url) = mock_server(
        r#"{"ok":false,"error":"wallet not found"}"#,
        "200 OK",
    );
    let client = NodeClient::new(&base_url);
    let err = client
        .enroll("RTCtest", "miner-01", &CpuArch::G4)
        .err()
        .expect("enroll should return Err for failed enrollment");
    assert!(
        matches!(err, ClawError::Node(_)),
        "expected Node error for failed enrollment, got {err:?}"
    );
    assert!(
        format!("{err}").contains("wallet not found"),
        "should include server error message: {err}"
    );
    drop(_shutdown);
}

#[test]
fn node_client_miners_returns_error_on_invalid_json() {
    let (_shutdown, base_url) = mock_server("broken json {", "200 OK");
    let client = NodeClient::new(&base_url);
    let err = client.miners().err()
        .expect("miners should return Err for invalid JSON");
    assert!(
        matches!(err, ClawError::Http(_)) || matches!(err, ClawError::Json(_)),
        "expected Http or Json error for invalid miners response, got {err:?}"
    );
    drop(_shutdown);
}

#[test]
fn node_client_challenge_returns_error_on_empty_response() {
    let (_shutdown, base_url) = mock_server("", "200 OK");
    let client = NodeClient::new(&base_url);
    let err = client.challenge().err()
        .expect("challenge should return Err for empty response");
    assert!(
        matches!(err, ClawError::Http(_)) || matches!(err, ClawError::Json(_)),
        "expected Http or Json error for empty response, got {err:?}"
    );
    drop(_shutdown);
}

#[test]
fn node_client_attest_returns_json_error_on_invalid_response_body() {
    let (_shutdown, base_url) = mock_server("not valid json at all", "200 OK");
    let client = NodeClient::new(&base_url);
    let payload = json!({ "nonce": "test" });
    let err = client.attest(&payload).err()
        .expect("attest should return Err for invalid JSON");
    assert!(
        matches!(err, ClawError::Http(_)) || matches!(err, ClawError::Json(_)),
        "expected Http or Json error for invalid response, got {err:?}"
    );
    drop(_shutdown);
}

// ── MinersResponse deserialisation edge cases ─────────────────

#[test]
fn miners_response_empty_list() {
    let miners: Vec<clawrtc::MinerInfo> = serde_json::from_str("[]").unwrap();
    assert!(miners.is_empty(), "empty list should deserialize to empty vec");
}

#[test]
fn miners_response_wrapped_with_empty_miners() {
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum MinersResponse {
        #[allow(dead_code)]
    List(Vec<clawrtc::MinerInfo>),
        Wrapped { miners: Vec<clawrtc::MinerInfo> },
    }
    let resp: MinersResponse =
        serde_json::from_str(r#"{"miners":[],"pagination":{"count":0,"total":0}}"#).unwrap();
    match resp {
        MinersResponse::Wrapped { miners } => assert!(miners.is_empty()),
        _ => panic!("expected wrapped response"),
    }
}

// ── CpuArch determinism and edge cases ────────────────────────

#[test]
fn cpu_arch_all_variants_have_positive_multipliers() {
    for variant in &[
        CpuArch::G4,
        CpuArch::G5,
        CpuArch::G3,
        CpuArch::Pentium4,
        CpuArch::Retro,
        CpuArch::Core2,
        CpuArch::AppleSilicon,
        CpuArch::Modern,
    ] {
        let mult = variant.multiplier();
        assert!(
            mult > 0.0,
            "{variant:?} multiplier must be positive, got {mult}"
        );
    }
}

#[test]
fn cpu_arch_family_and_arch_are_non_empty() {
    for variant in &[
        CpuArch::G4,
        CpuArch::G5,
        CpuArch::G3,
        CpuArch::Pentium4,
        CpuArch::Retro,
        CpuArch::Core2,
        CpuArch::AppleSilicon,
        CpuArch::Modern,
    ] {
        assert!(
            !variant.family().is_empty(),
            "{variant:?} family should not be empty"
        );
        assert!(
            !variant.arch_str().is_empty(),
            "{variant:?} arch_str should not be empty"
        );
    }
}

#[test]
fn cpu_arch_all_methods_are_deterministic() {
    for variant in &[
        CpuArch::G4,
        CpuArch::G5,
        CpuArch::G3,
        CpuArch::Pentium4,
        CpuArch::Retro,
        CpuArch::Core2,
        CpuArch::AppleSilicon,
        CpuArch::Modern,
    ] {
        assert_eq!(variant.multiplier(), variant.multiplier());
        assert_eq!(variant.family(), variant.family());
        assert_eq!(variant.arch_str(), variant.arch_str());
    }
}

// ── Multiple wallets have unique addresses ────────────────────

#[test]
fn multiple_generated_wallets_have_unique_addresses() {
    let count = 20;
    let mut addresses = HashSet::with_capacity(count);
    for _ in 0..count {
        let wallet = Wallet::generate();
        assert!(
            addresses.insert(wallet.address()),
            "generated wallets should have unique addresses"
        );
    }
    assert_eq!(addresses.len(), count);
}

// ── Address format invariant ──────────────────────────────────

#[test]
fn all_generated_addresses_are_43_chars_and_start_with_rtc() {
    for _ in 0..20 {
        let wallet = Wallet::generate();
        let addr = wallet.address();
        assert_eq!(addr.len(), 43, "address must be 43 chars, got {addr}");
        assert!(
            addr.starts_with("RTC"),
            "address must start with RTC, got {addr}"
        );
        // The remaining 40 chars should be valid hex
        let hex_part = &addr[3..];
        assert_eq!(
            hex::decode(hex_part).unwrap().len(),
            20,
            "address hex part should decode to 20 bytes"
        );
    }
}

// ── NodeClient trailing-slash handling (indirect) ─────────────

#[test]
fn node_client_works_with_trailing_slashes_in_url() {
    let (_shutdown, base_url) = mock_server(
        r#"{"ok":true,"version":"1.0.0","uptime_s":0.0}"#,
        "200 OK",
    );
    // Multiple trailing slashes should still work
    let client = NodeClient::new(&format!("{base_url}///"));
    let health = client.health().expect("health with trailing slashes should succeed");
    assert!(health.ok);
    drop(_shutdown);
}

// ── Challenge roundtrip JSON ──────────────────────────────────

#[test]
fn challenge_roundtrip_json() {
    let original = clawrtc::Challenge {
        nonce: "test-nonce-123".to_string(),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: clawrtc::Challenge = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.nonce, original.nonce);
}
