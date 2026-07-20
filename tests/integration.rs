use clawrtc::{Wallet, NodeClient};

// ── Wallet roundtrip ───────────────────────────────────────────

#[test]
fn test_wallet_generate_address_format() {
    let wallet = Wallet::generate();
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"), "Address should start with RTC: {addr}");
    assert_eq!(addr.len(), 43, "Address length should be 43: {addr}");
}

#[test]
fn test_wallet_sign_and_verify() {
    let wallet = Wallet::generate();
    let pubkey = wallet.public_key_hex();
    let message = b"hello rustchain";
    let signature = wallet.sign(message);
    let ok = Wallet::verify(&pubkey, message, &signature).unwrap();
    assert!(ok, "Signature should verify");
}

#[test]
fn test_wallet_verify_wrong_message() {
    let wallet = Wallet::generate();
    let pubkey = wallet.public_key_hex();
    let sig = wallet.sign(b"real message");
    let ok = Wallet::verify(&pubkey, b"tampered message", &sig).unwrap();
    assert!(!ok, "Wrong message should not verify");
}

#[test]
fn test_wallet_verify_wrong_key() {
    let alice = Wallet::generate();
    let bob = Wallet::generate();
    let sig = alice.sign(b"alice message");
    let ok = Wallet::verify(&bob.public_key_hex(), b"alice message", &sig).unwrap();
    assert!(!ok, "Wrong key should not verify");
}

#[test]
fn test_wallet_private_key_hex_roundtrip() {
    let wallet = Wallet::generate();
    let hex = wallet.private_key_hex();
    let restored = Wallet::from_hex(&hex).expect("from_hex should succeed");
    assert_eq!(wallet.address(), restored.address(), "Addresses should match after hex roundtrip");
    assert_eq!(wallet.public_key_hex(), restored.public_key_hex(), "Pubkeys should match");
}

// ── Error paths ─────────────────────────────────────────────────

#[test]
fn test_wallet_from_hex_invalid_fails() {
    let result = Wallet::from_hex("not-hex");
    assert!(result.is_err(), "Invalid hex should fail");
}

#[test]
fn test_wallet_from_hex_short_fails() {
    let result = Wallet::from_hex("aabb");
    assert!(result.is_err(), "Short hex should fail");
}

#[test]
fn test_wallet_from_hex_empty_fails() {
    let result = Wallet::from_hex("");
    assert!(result.is_err(), "Empty hex should fail");
}

#[test]
fn test_wallet_verify_bad_pubkey_fails() {
    let wallet = Wallet::generate();
    let sig = wallet.sign(b"test");
    let result = Wallet::verify("not-hex", b"test", &sig);
    assert!(result.is_err(), "Bad pubkey hex should error");
}

#[test]
fn test_wallet_verify_bad_sig_fails() {
    let wallet = Wallet::generate();
    let pubkey = wallet.public_key_hex();
    let result = Wallet::verify(&pubkey, b"test", "not-hex");
    assert!(result.is_err(), "Bad sig hex should error");
}

// ── Address derivation vectors ──────────────────────────────────

/// Regression vectors: fixed private key hex → expected RTC address.
const ADDRESS_VECTORS: &[(&str, &str)] = &[
    (
        "b966bcc0206a2d5a09c8a2af5a697d78fe8d0295df85ae3e11a23d9a9cc21be1",
        "RTCe6fe5c4171e3ee2e1a38e6c2d5b7ebe28da6d79e89aa7a58e6392f5e7c85f3c4",
    ),
    (
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "RTCf7e6b303744290a3e647c1e7f4b80e4a672ba0d7bf7231f3d73d8e8f5c237523",
    ),
    (
        "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
        "RTC37b1c7c4ac8e73e4bc97dccd700e8c75dfe400a410b2a098cf3078f57e9f1283",
    ),
];

#[test]
fn test_address_derivation_vectors() {
    for (priv_hex, expected_addr) in ADDRESS_VECTORS {
        let wallet = Wallet::from_hex(priv_hex)
            .expect("Wallet from hex should succeed");
        assert_eq!(
            wallet.address(),
            *expected_addr,
            "Address mismatch for private key {priv_hex}"
        );
    }
}

// ── Node client ─────────────────────────────────────────────────

#[test]
fn test_node_client_health_fails_on_bad_url() {
    let node = NodeClient::new("https://0.0.0.0:1");
    let result = node.health();
    assert!(result.is_err(), "Unreachable node should error");
}

#[test]
fn test_node_client_balance_empty_address() {
    let node = NodeClient::new("https://rustchain.org");
    let result = node.balance("");
    assert!(result.is_err(), "Empty address should error");
}

// ── Determinism ─────────────────────────────────────────────────

#[test]
fn test_wallet_deterministic_from_hex() {
    let hex_key = "b966bcc0206a2d5a09c8a2af5a697d78fe8d0295df85ae3e11a23d9a9cc21be1";
    let w1 = Wallet::from_hex(hex_key).unwrap();
    let w2 = Wallet::from_hex(hex_key).unwrap();
    assert_eq!(w1.address(), w2.address(), "Same key should produce same address");
    assert_eq!(w1.public_key_hex(), w2.public_key_hex(), "Same key should produce same pubkey");
    assert_eq!(w1.private_key_hex(), w2.private_key_hex(), "Same key should produce same privkey");
}
