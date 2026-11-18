use clawrtc::Wallet;

// ── Fixed test vectors for address derivation regression ────────────────
// Computed from ed25519-dalek + sha2. These MUST NOT change.
const VECTOR_1_PRIVKEY: &str = "0000000000000000000000000000000000000000000000000000000000000001";
const VECTOR_1_ADDRESS: &str = "RTC4a67330b803d5c88757afb9328615344a89c4983";

const VECTOR_2_PRIVKEY: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const VECTOR_2_ADDRESS: &str = "RTCaf822958f2d75afb91f8a8f4da253230d63bebf8";

const VECTOR_3_PRIVKEY: &str = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
const VECTOR_3_ADDRESS: &str = "RTCe78d76a96abf71cf1c6fc032e971de8fd8349a2b";

// ── 1. Wallet Roundtrip ─────────────────────────────────────────────────

#[test]
fn wallet_roundtrip_generate_serialize_deserialize() {
    let w1 = Wallet::generate();
    let privkey_hex = w1.private_key_hex();
    let pubkey_hex = w1.public_key_hex();
    let address = w1.address();

    let w2 = Wallet::from_hex(&privkey_hex).expect("restore from hex must succeed");
    assert_eq!(w2.private_key_hex(), privkey_hex);
    assert_eq!(w2.public_key_hex(), pubkey_hex);
    assert_eq!(w2.address(), address);
}

#[test]
fn wallet_sign_verify_roundtrip() {
    let wallet = Wallet::generate();
    let msg = b"integration test message for roundtrip";
    let sig = wallet.sign(msg);

    assert!(
        Wallet::verify(&wallet.public_key_hex(), msg, &sig).unwrap(),
        "valid signature must verify"
    );
}

#[test]
fn wallet_verify_rejects_tampered_message() {
    let wallet = Wallet::generate();
    let sig = wallet.sign(b"original message");

    let result = Wallet::verify(&wallet.public_key_hex(), b"tampered message", &sig);
    match result {
        Ok(false) => {} // expected: verification fails cleanly
        Ok(true) => panic!("tampered message must not verify"),
        Err(_) => {} // also acceptable if strict parsing rejects it
    }
}

#[test]
fn wallet_verify_rejects_tampered_signature() {
    let wallet = Wallet::generate();
    let msg = b"message for tamper test";
    let sig = wallet.sign(msg);

    // Flip a byte in the signature
    let mut tampered_bytes = hex::decode(&sig).unwrap();
    tampered_bytes[0] ^= 0xff;
    let tampered_sig = hex::encode(&tampered_bytes);

    let result = Wallet::verify(&wallet.public_key_hex(), msg, &tampered_sig);
    match result {
        Ok(false) => {}
        Ok(true) => panic!("tampered signature must not verify"),
        Err(_) => {}
    }
}

#[test]
fn wallet_verify_rejects_wrong_key() {
    let w1 = Wallet::generate();
    let w2 = Wallet::generate();
    let msg = b"wrong key test";
    let sig = w1.sign(msg);

    let result = Wallet::verify(&w2.public_key_hex(), msg, &sig);
    match result {
        Ok(false) => {}
        Ok(true) => panic!("signature from different key must not verify"),
        Err(_) => {}
    }
}

// ── 2. Address Derivation Vectors ───────────────────────────────────────

#[test]
fn address_derivation_vector_1() {
    let wallet = Wallet::from_hex(VECTOR_1_PRIVKEY).unwrap();
    let addr = wallet.address();
    assert!(
        addr.starts_with("RTC"),
        "address must start with RTC prefix"
    );
    assert_eq!(addr.len(), 43, "RTC + 40 hex chars = 43 total");
    assert_eq!(
        addr, VECTOR_1_ADDRESS,
        "address derivation changed — update vector or fix regression"
    );
}

#[test]
fn address_derivation_vector_2() {
    let wallet = Wallet::from_hex(VECTOR_2_PRIVKEY).unwrap();
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"));
    assert_eq!(addr.len(), 43);
    assert_eq!(
        addr, VECTOR_2_ADDRESS,
        "address derivation changed — update vector or fix regression"
    );
}

#[test]
fn address_derivation_vector_3() {
    let wallet = Wallet::from_hex(VECTOR_3_PRIVKEY).unwrap();
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"));
    assert_eq!(addr.len(), 43);
    assert_eq!(
        addr, VECTOR_3_ADDRESS,
        "address derivation changed — update vector or fix regression"
    );
}

// ── 3. Error Paths ──────────────────────────────────────────────────────

#[test]
fn wallet_from_hex_rejects_invalid_hex() {
    let result = Wallet::from_hex("not_valid_hex_at_all!!!");
    assert!(result.is_err(), "invalid hex must return error");
    let err_msg = format!("{}", result.err().unwrap());
    assert!(
        err_msg.contains("invalid hex") || err_msg.contains("Wallet"),
        "error should mention hex/wallet issue: got '{}'",
        err_msg
    );
}

#[test]
fn wallet_from_hex_rejects_wrong_length() {
    // Valid hex but only 16 bytes instead of 32
    let result = Wallet::from_hex("deadbeefdeadbeefdeadbeefdeadbeef");
    assert!(result.is_err(), "wrong-length key must return error");
    let err_msg = format!("{}", result.err().unwrap());
    assert!(
        err_msg.contains("32 bytes") || err_msg.contains("Wallet"),
        "error should mention length requirement: got '{}'",
        err_msg
    );
}

#[test]
fn wallet_verify_rejects_bad_pubkey_hex() {
    let result = Wallet::verify("zzzz_not_hex", b"msg", &"aa".repeat(64));
    assert!(result.is_err());
}

#[test]
fn wallet_verify_rejects_bad_signature_hex() {
    let wallet = Wallet::generate();
    let result = Wallet::verify(&wallet.public_key_hex(), b"msg", "zzzz_not_hex");
    assert!(result.is_err());
}

#[test]
fn wallet_verify_rejects_wrong_pubkey_length() {
    // Valid hex but only 16 bytes
    let short_pubkey = "aa".repeat(16);
    let result = Wallet::verify(&short_pubkey, b"msg", &"bb".repeat(64));
    assert!(result.is_err());
    let err_msg = format!("{}", result.err().unwrap());
    assert!(
        err_msg.contains("32 bytes") || err_msg.contains("pubkey"),
        "should mention pubkey length: got '{}'",
        err_msg
    );
}

#[test]
fn wallet_verify_rejects_wrong_signature_length() {
    let wallet = Wallet::generate();
    // Valid hex but only 32 bytes instead of 64
    let short_sig = "cc".repeat(32);
    let result = Wallet::verify(&wallet.public_key_hex(), b"msg", &short_sig);
    assert!(result.is_err());
    let err_msg = format!("{}", result.err().unwrap());
    assert!(
        err_msg.contains("64 bytes") || err_msg.contains("signature"),
        "should mention signature length: got '{}'",
        err_msg
    );
}
