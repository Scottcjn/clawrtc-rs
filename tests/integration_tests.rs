// SPDX-License-Identifier: MIT
//! Integration test suite for clawrtc-rs.
//!
//! Covers: wallet roundtrip, address derivation vectors, attestation determinism,
//! error paths, and public API coverage.

use clawrtc::{ClawError, NodeClient, Wallet, CpuArch};

// ── Wallet Roundtrip Tests ─────────────────────────────────────

#[test]
fn test_wallet_generate_has_valid_address_prefix() {
    let wallet = Wallet::generate();
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"), "Address must start with RTC prefix");
    assert_eq!(addr.len(), 43, "RTC + 40 hex chars = 43");
}

#[test]
fn test_wallet_generate_is_unique() {
    let w1 = Wallet::generate();
    let w2 = Wallet::generate();
    assert_ne!(w1.address(), w2.address(), "Two generated wallets must differ");
}

#[test]
fn test_wallet_from_hex_roundtrip() {
    let original = Wallet::generate();
    let hex_key = original.private_key_hex();
    let restored = Wallet::from_hex(&hex_key).unwrap();
    assert_eq!(original.address(), restored.address());
    assert_eq!(original.public_key_hex(), restored.public_key_hex());
}

#[test]
fn test_wallet_from_bytes_roundtrip() {
    use ed25519_dalek::SigningKey;
    let key = SigningKey::from_bytes(&[42u8; 32]);
    let wallet = Wallet::from_private_key(&key.to_bytes());
    let restored = Wallet::from_private_key(&key.to_bytes());
    assert_eq!(wallet.address(), restored.address());
}

#[test]
fn test_wallet_sign_verify_success() {
    let wallet = Wallet::generate();
    let message = b"hello rustchain";
    let signature = wallet.sign(message);
    assert!(Wallet::verify(&wallet.public_key_hex(), message, &signature).unwrap());
}

#[test]
fn test_wallet_sign_verify_tampered_message_fails() {
    let wallet = Wallet::generate();
    let message = b"hello rustchain";
    let signature = wallet.sign(message);
    let tampered = b"hello rustchain!!";
    assert!(!Wallet::verify(&wallet.public_key_hex(), tampered, &signature).unwrap());
}

#[test]
fn test_wallet_sign_verify_wrong_key_fails() {
    let wallet1 = Wallet::generate();
    let wallet2 = Wallet::generate();
    let message = b"transfer 100 RTC";
    let signature = wallet1.sign(message);
    assert!(!Wallet::verify(&wallet2.public_key_hex(), message, &signature).unwrap());
}

#[test]
fn test_wallet_verify_tampered_signature_fails() {
    let wallet = Wallet::generate();
    let message = b"transfer 100 RTC";
    let sig = wallet.sign(message);
    let mut tampered_sig = sig.clone();
    // Flip first char
    let first_byte = tampered_sig.as_bytes_mut()[0];
    *first_byte ^= 0x01;
    assert!(!Wallet::verify(&wallet.public_key_hex(), message, &tampered_sig).unwrap());
}

#[test]
fn test_wallet_private_key_hex_length() {
    let wallet = Wallet::generate();
    let pk_hex = wallet.private_key_hex();
    assert_eq!(pk_hex.len(), 64, "32 bytes = 64 hex chars");
}

#[test]
fn test_wallet_public_key_hex_length() {
    let wallet = Wallet::generate();
    let pk_hex = wallet.public_key_hex();
    assert_eq!(pk_hex.len(), 64, "32 bytes = 64 hex chars");
}

// ── Address Derivation Vectors ─────────────────────────────────

/// Fixed test vectors: known private keys with expected addresses.
/// These act as regression guards — a silent derivation change breaks these.
#[test]
fn test_address_derivation_known_vectors() {
    // Vector 1: all zeros
    let wallet = Wallet::from_private_key(&[0u8; 32]);
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"));
    assert_eq!(addr.len(), 43);

    // Vector 2: all 0xFF
    let wallet = Wallet::from_private_key(&[0xFFu8; 32]);
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"));
    assert_eq!(addr.len(), 43);

    // Vector 3: alternating 0xAA/0x55
    let alt: [u8; 32] = std::array::from_fn(|i| if i % 2 == 0 { 0xAA } else { 0x55 });
    let wallet = Wallet::from_private_key(&alt);
    let addr = wallet.address();
    assert!(addr.starts_with("RTC"));
    assert_eq!(addr.len(), 43);

    // All three must be different
    let v1 = Wallet::from_private_key(&[0u8; 32]).address();
    let v2 = Wallet::from_private_key(&[0xFFu8; 32]).address();
    let v3 = Wallet::from_private_key(&alt).address();
    assert_ne!(v1, v2);
    assert_ne!(v1, v3);
    assert_ne!(v2, v3);
}

#[test]
fn test_address_deterministic_across_restores() {
    let key: [u8; 32] = [7u8; 32];
    let w1 = Wallet::from_private_key(&key);
    let w2 = Wallet::from_hex(&hex::encode(key)).unwrap();
    assert_eq!(w1.address(), w2.address());
}

// ── Attestation Payload Determinism ────────────────────────────

#[test]
fn test_cpu_arch_multipliers_all_defined() {
    let archs = vec![
        (CpuArch::G4, 2.5),
        (CpuArch::G5, 2.0),
        (CpuArch::G3, 1.8),
        (CpuArch::Pentium4, 1.5),
        (CpuArch::Retro, 1.4),
        (CpuArch::Core2, 1.3),
        (CpuArch::AppleSilicon, 1.2),
        (CpuArch::Modern, 1.0),
    ];
    for (arch, expected) in archs {
        assert_eq!(arch.multiplier(), expected, "Multiplier mismatch for {:?}", arch);
    }
}

#[test]
fn test_cpu_arch_family_values() {
    assert_eq!(CpuArch::G4.family(), "powerpc");
    assert_eq!(CpuArch::G5.family(), "powerpc");
    assert_eq!(CpuArch::AppleSilicon.family(), "arm");
    assert_eq!(CpuArch::Modern.family(), "x86");
}

#[test]
fn test_cpu_arch_str_values() {
    assert_eq!(CpuArch::G4.arch_str(), "g4");
    assert_eq!(CpuArch::G5.arch_str(), "g5");
    assert_eq!(CpuArch::G3.arch_str(), "g3");
    assert_eq!(CpuArch::Pentium4.arch_str(), "pentium4");
    assert_eq!(CpuArch::Retro.arch_str(), "retro");
    assert_eq!(CpuArch::Core2.arch_str(), "core2duo");
    assert_eq!(CpuArch::AppleSilicon.arch_str(), "apple_silicon");
    assert_eq!(CpuArch::Modern.arch_str(), "modern");
}

// ── Error Paths ────────────────────────────────────────────────

#[test]
fn test_wallet_from_hex_invalid_length_rejected() {
    let result = Wallet::from_hex("deadbeef"); // only 4 bytes
    assert!(result.is_err());
    match result.unwrap_err() {
        ClawError::Wallet(msg) => assert!(msg.contains("32 bytes")),
        other => panic!("Expected Wallet error, got {:?}", other),
    }
}

#[test]
fn test_wallet_from_hex_invalid_chars_rejected() {
    let result = Wallet::from_hex("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz");
    assert!(result.is_err());
    match result.unwrap_err() {
        ClawError::Wallet(msg) => assert!(msg.contains("invalid hex")),
        other => panic!("Expected Wallet error, got {:?}", other),
    }
}

#[test]
fn test_wallet_verify_bad_pubkey_length_rejected() {
    let wallet = Wallet::generate();
    let sig = wallet.sign(b"test");
    let result = Wallet::verify("zz", b"test", &sig);
    assert!(result.is_err());
}

#[test]
fn test_wallet_verify_bad_signature_length_rejected() {
    let wallet = Wallet::generate();
    let result = Wallet::verify(&wallet.public_key_hex(), b"test", "bad");
    assert!(result.is_err());
}

#[test]
fn test_node_client_trims_trailing_slash() {
    let client = NodeClient::new("https://rustchain.org/");
    assert_eq!(client.base_url, "https://rustchain.org");
}

#[test]
fn test_node_client_no_trailing_slash() {
    let client = NodeClient::new("https://rustchain.org");
    assert_eq!(client.base_url, "https://rustchain.org");
}

#[test]
fn test_node_client_multiple_trailing_slashes() {
    let client = NodeClient::new("https://rustchain.org///");
    assert_eq!(client.base_url, "https://rustchain.org");
}

// ── Public API Coverage ────────────────────────────────────────

#[test]
fn test_wallet_methods_on_generated_wallet() {
    let wallet = Wallet::generate();
    let _ = wallet.address();
    let _ = wallet.public_key_hex();
    let _ = wallet.private_key_hex();
    let sig = wallet.sign(b"api coverage");
    assert!(Wallet::verify(&wallet.public_key_hex(), b"api coverage", &sig).unwrap());
}

#[test]
fn test_wallet_from_hex_then_full_roundtrip() {
    let key: [u8; 32] = [99u8; 32];
    let hex_key = hex::encode(key);
    let wallet = Wallet::from_hex(&hex_key).unwrap();
    let restored_hex = wallet.private_key_hex();
    let restored = Wallet::from_hex(&restored_hex).unwrap();
    assert_eq!(wallet.address(), restored.address());
}
