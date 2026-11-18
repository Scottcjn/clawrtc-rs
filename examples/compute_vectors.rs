use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};

fn main() {
    let keys = [
        "0000000000000000000000000000000000000000000000000000000000000001",
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
    ];

    for (i, key_hex) in keys.iter().enumerate() {
        let bytes = hex::decode(key_hex).unwrap();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        let signing_key = SigningKey::from_bytes(&arr);
        let pubkey = signing_key.verifying_key().to_bytes();
        let hash = Sha256::digest(pubkey);
        let addr = format!("RTC{}", &hex::encode(hash)[..40]);
        println!("VECTOR_{}_ADDRESS: \"{}\"", i + 1, addr);
    }
}
