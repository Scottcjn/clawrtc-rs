// SPDX-License-Identifier: MIT
//!
//! Regression vectors: fixed keypairs with deterministic RTC addresses.
//!
//! Covers bounty criterion 2:
//!   At least 3 fixed keypairs with expected `RTC…` addresses
//!   committed as regression vectors (so a silent derivation change
//!   breaks the build).
//!
//! Address derivation:
//!   address = "RTC" + SHA-256(Ed25519 pubkey)[..40] in hex

use clawrtc::Wallet;

// ── Vector 1: All-zero key ──────────────────────────────────────

/// Private key (32 bytes, hex)
const KEY_1: &str = "0000000000000000000000000000000000000000000000000000000000000000";
/// Expected Ed25519 public key
const PUBKEY_1: &str = "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29";
/// Expected address: RTC + SHA-256(pubkey)[..40]
const ADDR_1: &str = "RTC139e3940e64b5491722088d9a0d741628fc826e0";

#[test]
fn vector_1_all_zero_key() {
    let wallet = Wallet::from_hex(KEY_1).expect("valid hex key");
    assert_eq!(
        wallet.public_key_hex(),
        PUBKEY_1,
        "vector 1: public key mismatch"
    );
    assert_eq!(wallet.address(), ADDR_1, "vector 1: address mismatch");
}

// ── Vector 2: All-0x11 key ──────────────────────────────────────

const KEY_2: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const PUBKEY_2: &str = "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";
const ADDR_2: &str = "RTC10ba682c8ad13513971e8b56881aab8bd702bb80";

#[test]
fn vector_2_all_ones_key() {
    let wallet = Wallet::from_hex(KEY_2).expect("valid hex key");
    assert_eq!(
        wallet.public_key_hex(),
        PUBKEY_2,
        "vector 2: public key mismatch"
    );
    assert_eq!(wallet.address(), ADDR_2, "vector 2: address mismatch");
}

// ── Vector 3: Known pattern key ─────────────────────────────────

const KEY_3: &str = "deadbeefcafebabe000000000000000000000000000000000000000000000000";
const PUBKEY_3: &str = "6b02ea91e8350e83a72dc713a8496bdd15da4f4a8c73fed8d54eb0a1650eecc7";
const ADDR_3: &str = "RTCdc8a1d6c605a89d01992d8cb27fb79c9349cc5dd";

#[test]
fn vector_3_pattern_key() {
    let wallet = Wallet::from_hex(KEY_3).expect("valid hex key");
    assert_eq!(
        wallet.public_key_hex(),
        PUBKEY_3,
        "vector 3: public key mismatch"
    );
    assert_eq!(wallet.address(), ADDR_3, "vector 3: address mismatch");
}

// ── Vector 4: Incremental key ───────────────────────────────────

const KEY_4: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
const PUBKEY_4: &str = "befb02bd154baa145c4e26c7cb27b8c51f30c29e9182f948da4951808f6103ce";
const ADDR_4: &str = "RTCd382263912a08ca6183a552e0b9a97ce3b405991";

#[test]
fn vector_4_incremental_key() {
    let wallet = Wallet::from_hex(KEY_4).expect("valid hex key");
    assert_eq!(
        wallet.public_key_hex(),
        PUBKEY_4,
        "vector 4: public key mismatch"
    );
    assert_eq!(wallet.address(), ADDR_4, "vector 4: address mismatch");
}

// ── Sanity checks on vector addresses ───────────────────────────

#[test]
fn all_vectors_have_valid_address_format() {
    for addr in &[ADDR_1, ADDR_2, ADDR_3, ADDR_4] {
        assert!(addr.starts_with("RTC"), "address must start with RTC");
        assert_eq!(addr.len(), 43, "address must be RTC + 40 hex chars");
        let hex_part = &addr[3..];
        assert!(
            hex_part.chars().all(|c| c.is_ascii_hexdigit()),
            "address suffix must be hex: got {hex_part}"
        );
    }
}
