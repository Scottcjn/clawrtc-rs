// SPDX-License-Identifier: MIT
use clawrtc::Wallet;
use sha2::{Digest, Sha256};

const PUBLIC_KEY_HEX: &str = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
const SIGNATURE_HEX: &str = concat!(
    "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155",
    "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
);

fn rtc_address(public_key_hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    let public_key = hex::decode(public_key_hex)?;
    if public_key.len() != 32 {
        return Err("an Ed25519 public key must contain exactly 32 bytes".into());
    }

    let digest = Sha256::digest(public_key);
    Ok(format!("RTC{}", &hex::encode(digest)[..40]))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // RFC 8032 test vector 1 signs an empty message. It is public test data;
    // this example never loads a private key or contacts a RustChain node.
    let signature_valid = Wallet::verify(PUBLIC_KEY_HEX, b"", SIGNATURE_HEX)?;
    let tampered_message_valid = Wallet::verify(PUBLIC_KEY_HEX, b"x", SIGNATURE_HEX)?;
    let address = rtc_address(PUBLIC_KEY_HEX)?;

    println!("address={address}");
    println!("signature_valid={signature_valid}");
    println!("tampered_message_valid={tampered_message_valid}");

    if !signature_valid || tampered_message_valid {
        return Err("public proof verification did not produce the expected result".into());
    }

    Ok(())
}
