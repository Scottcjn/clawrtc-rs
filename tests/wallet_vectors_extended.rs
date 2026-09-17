// SPDX-License-Identifier: MIT
//! Extended wallet integration tests for clawrtc-rs.
//!
//! These complement the existing `tests/public_api.rs` suite with coverage
//! that was not yet exercised by it:
//!
//!   * `Wallet::from_private_key` roundtrip determinism
//!   * cross-verification of `Wallet::sign` against a *raw* ed25519-dalek check
//!     (proves the crate emits standards-compliant Ed25519 signatures)
//!   * address / public-key format invariants across many random wallets
//!   * `Wallet::verify` returns `Ok(false)` — never panics, never `Err` — on a
//!     tampered message, a tampered signature, or the wrong key
//!   * `Wallet::from_hex` error variants (odd-length hex, 0x-prefix, wrong length)
//!   * address stability across repeated derivation
//!   * extra RFC 8032 address-derivation regression vectors
//!
//! Item → test mapping (for the bounty write-up):
//!   1. Wallet roundtrip        → `from_private_key_is_deterministic`,
//!                                `address_is_stable_across_repeated_calls`
//!   2. Address derivation vecs  → `rfc8032_extra_vectors`
//!   3. Attestation determinism  → (covered in `attestation_flow.rs`)
//!   4. Error paths             → `verify_returns_false_not_err_on_tampered_inputs`,
//!                                `from_hex_rejects_invalid_inputs_with_wallet_error`
//!   + cross-compat             → `signature_is_64_bytes_and_verifies_with_raw_ed25519`,
//!                                `address_format_invariant_over_many_wallets`

use clawrtc::Wallet;
use ed25519_dalek::{Signature, VerifyingKey};

#[test]
fn from_private_key_is_deterministic() {
    let key = [7u8; 32];
    let a = Wallet::from_private_key(&key);
    let b = Wallet::from_private_key(&key);
    assert_eq!(a.address(), b.address());
    assert_eq!(a.public_key_hex(), b.public_key_hex());
    assert_eq!(a.private_key_hex(), b.private_key_hex());
}

#[test]
fn address_is_stable_across_repeated_calls() {
    let w = Wallet::generate();
    let first = w.address();
    assert!(first.starts_with("RTC"));
    assert_eq!(first.len(), 43);
    for _ in 0..10 {
        assert_eq!(w.address(), first);
    }
}

#[test]
fn address_format_invariant_over_many_wallets() {
    for _ in 0..25 {
        let w = Wallet::generate();
        let addr = w.address();
        // "RTC" + 40 hex chars = 43 chars total.
        assert!(addr.starts_with("RTC"));
        assert_eq!(addr.len(), 43);
        let body = &addr[3..];
        assert_eq!(body.len(), 40);
        assert!(body.chars().all(|c| c.is_ascii_hexdigit()));
        // The public key is 32 bytes => 64 hex chars.
        assert_eq!(w.public_key_hex().len(), 64);
        // The private key is 32 bytes => 64 hex chars.
        assert_eq!(w.private_key_hex().len(), 64);
    }
}

#[test]
fn signature_is_64_bytes_and_verifies_with_raw_ed25519() {
    let w = Wallet::generate();
    let msg = b"integration payload for cross-check";
    let sig_hex = w.sign(msg);

    // Ed25519 signatures are 64 bytes => 128 hex chars.
    assert_eq!(sig_hex.len(), 128);
    let sig_bytes = hex::decode(&sig_hex).expect("signature is valid hex");
    assert_eq!(sig_bytes.len(), 64);

    let pk_bytes = hex::decode(w.public_key_hex()).expect("pubkey is valid hex");
    assert_eq!(pk_bytes.len(), 32);

    let mut pk_arr = [0u8; 32];
    pk_arr.copy_from_slice(&pk_bytes);
    let vk = VerifyingKey::from_bytes(&pk_arr).expect("valid public key");

    let mut sig_arr = [0u8; 64];
    sig_arr.copy_from_slice(&sig_bytes);
    let sig = Signature::from_bytes(&sig_arr);

    // The crate must produce a signature that a stock Ed25519 verifier accepts.
    assert!(vk.verify_strict(msg, &sig).is_ok());
}

#[test]
fn verify_returns_false_not_err_on_tampered_inputs() {
    let w = Wallet::generate();
    let msg = b"original message";
    let sig = w.sign(msg);

    // Tampered message: verification must fail gracefully (Ok(false)), not error.
    let r1 = Wallet::verify(&w.public_key_hex(), b"tampered message", &sig);
    assert!(
        matches!(r1, Ok(false)),
        "tampered message must verify to Ok(false), got {r1:?}"
    );

    // Tampered signature (flip the final hex digit): Ok(false), never panic.
    let mut bad = sig.clone();
    let last = bad.pop().unwrap();
    bad.push(if last == '0' { '1' } else { '0' });
    let r2 = Wallet::verify(&w.public_key_hex(), msg, &bad);
    assert!(
        matches!(r2, Ok(false)),
        "tampered signature must verify to Ok(false), got {r2:?}"
    );

    // Wrong key (a different wallet's public key): Ok(false).
    let other = Wallet::generate();
    let r3 = Wallet::verify(&other.public_key_hex(), msg, &sig);
    assert!(
        matches!(r3, Ok(false)),
        "wrong key must verify to Ok(false), got {r3:?}"
    );

    // Correct inputs verify to true.
    assert_eq!(Wallet::verify(&w.public_key_hex(), msg, &sig), Ok(true));
}

#[test]
fn from_hex_rejects_invalid_inputs_with_wallet_error() {
    // Odd-length hex cannot decode.
    assert!(Wallet::from_hex("abc").is_err());
    // Valid hex but the wrong length (31 bytes).
    assert!(Wallet::from_hex(&"00".repeat(31)).is_err());
    // Non-hex characters.
    assert!(Wallet::from_hex("zzzz").is_err());
    // A "0x" prefix is not valid input for hex::decode.
    assert!(Wallet::from_hex(&format!("0x{}", "11".repeat(32))).is_err());
    // A valid 32-byte key works.
    assert!(Wallet::from_hex(&"11".repeat(32)).is_ok());
    // The restored wallet exposes the same address as a from_private_key wallet.
    let restored = Wallet::from_hex(&"11".repeat(32)).expect("valid 32-byte key");
    let direct = Wallet::from_private_key(&[0x11u8; 32]);
    assert_eq!(restored.address(), direct.address());
}

#[test]
fn rfc8032_extra_vectors_stable_addresses() {
    // RFC 8032 Ed25519 test vectors (private, public, expected RTC address).
    // RTC address = "RTC" + SHA-256(public key) hex, truncated to 40 hex chars.
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
        (
            "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42",
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
            "RTC5f9b247e2a654719f198e4f241d6b0df9a1a937a",
        ),
    ];

    for (private_key, public_key, address) in vectors {
        let wallet = Wallet::from_hex(private_key).expect("valid RFC 8032 private key");
        assert_eq!(wallet.public_key_hex(), public_key);
        assert_eq!(wallet.address(), address);
    }
}
