//! # ClawRTC
//!
//! Rust client for RustChain RTC mining — hardware attestation,
//! Ed25519 wallets, and Proof-of-Antiquity (PoA) consensus.
//!
//! ## Quick Start
//!
//! ```no_run
//! use clawrtc::{NodeClient, Wallet};
//!
//! // Create a wallet
//! let wallet = Wallet::generate();
//! println!("Address: {}", wallet.address());
//!
//! // Connect to the RustChain network
//! let node = NodeClient::new("https://rustchain.org");
//!
//! // Check balance
//! let balance = node.balance(&wallet.address()).unwrap();
//! println!("Balance: {} RTC", balance);
//!
//! // Check node health
//! let health = node.health().unwrap();
//! println!("Node: {} (uptime {}s)", health.version, health.uptime_s);
//! ```
//!
//! ## Features
//!
//! - **Ed25519 Wallets** — generate, sign, verify with RTC addresses
//! - **Node Client** — health, balance, miners, epochs
//! - **Hardware Detection** — CPU architecture for PoA multipliers
//! - **Attestation** — submit hardware proofs to earn RTC

use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Errors returned by the ClawRTC client.
#[derive(Debug, thiserror::Error)]
pub enum ClawError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Node error: {0}")]
    Node(String),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Wallet error: {0}")]
    Wallet(String),
    #[error("Signing error: {0}")]
    Signing(String),
}

pub type Result<T> = std::result::Result<T, ClawError>;

// ── Data types ──────────────────────────────────────────────────

/// Node health status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealth {
    pub ok: bool,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub uptime_s: f64,
    #[serde(default)]
    pub db_rw: bool,
}

/// Active miner info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerInfo {
    #[serde(default)]
    pub miner: String,
    #[serde(default)]
    pub miner_id: String,
    #[serde(default)]
    pub device_arch: String,
    #[serde(default)]
    pub device_family: String,
    #[serde(default)]
    pub last_seen: String,
}

/// Epoch enrollment response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollResponse {
    pub ok: bool,
    #[serde(default)]
    pub epoch: u64,
    #[serde(default)]
    pub weight: f64,
}

/// Attestation challenge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub nonce: String,
}

/// Attestation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestResponse {
    pub ok: bool,
    #[serde(default)]
    pub message: String,
}

/// Balance response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    #[serde(default)]
    pub balance_rtc: f64,
}

/// CPU architecture for PoA multipliers.
#[derive(Debug, Clone, PartialEq)]
pub enum CpuArch {
    /// PowerPC G4 (2.5x antiquity multiplier)
    G4,
    /// PowerPC G5 (2.0x multiplier)
    G5,
    /// PowerPC G3 (1.8x multiplier)
    G3,
    /// Pentium 4 (1.5x multiplier)
    Pentium4,
    /// Retro x86 (1.4x multiplier)
    Retro,
    /// Core 2 Duo (1.3x multiplier)
    Core2,
    /// Apple Silicon M1/M2/M3 (1.2x multiplier)
    AppleSilicon,
    /// Modern x86_64 or aarch64 (1.0x base)
    Modern,
}

impl CpuArch {
    /// Base antiquity multiplier for this architecture.
    pub fn multiplier(&self) -> f64 {
        match self {
            CpuArch::G4 => 2.5,
            CpuArch::G5 => 2.0,
            CpuArch::G3 => 1.8,
            CpuArch::Pentium4 => 1.5,
            CpuArch::Retro => 1.4,
            CpuArch::Core2 => 1.3,
            CpuArch::AppleSilicon => 1.2,
            CpuArch::Modern => 1.0,
        }
    }

    /// Device family string for attestation payloads.
    pub fn family(&self) -> &str {
        match self {
            CpuArch::G4 | CpuArch::G5 | CpuArch::G3 => "powerpc",
            CpuArch::AppleSilicon => "arm",
            _ => "x86",
        }
    }

    /// Architecture string for attestation payloads.
    pub fn arch_str(&self) -> &str {
        match self {
            CpuArch::G4 => "g4",
            CpuArch::G5 => "g5",
            CpuArch::G3 => "g3",
            CpuArch::Pentium4 => "pentium4",
            CpuArch::Retro => "retro",
            CpuArch::Core2 => "core2duo",
            CpuArch::AppleSilicon => "apple_silicon",
            CpuArch::Modern => "modern",
        }
    }
}

// ── Wallet ──────────────────────────────────────────────────────

/// An Ed25519 wallet for RustChain RTC.
///
/// Generates an Ed25519 keypair and derives an RTC address
/// from the SHA-256 hash of the public key.
pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    /// Generate a new random wallet.
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }

    /// Restore a wallet from a 32-byte private key.
    pub fn from_private_key(bytes: &[u8; 32]) -> Self {
        Self {
            signing_key: SigningKey::from_bytes(bytes),
        }
    }

    /// Restore a wallet from a hex-encoded private key.
    pub fn from_hex(hex_key: &str) -> Result<Self> {
        let bytes = hex::decode(hex_key)
            .map_err(|e| ClawError::Wallet(format!("invalid hex: {e}")))?;
        if bytes.len() != 32 {
            return Err(ClawError::Wallet("private key must be 32 bytes".into()));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Self::from_private_key(&arr))
    }

    /// The RTC address (RTC + first 40 hex chars of SHA-256(pubkey)).
    pub fn address(&self) -> String {
        let pubkey = self.signing_key.verifying_key().to_bytes();
        let hash = Sha256::digest(pubkey);
        format!("RTC{}", &hex::encode(hash)[..40])
    }

    /// Public key as hex string.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    /// Private key as hex string.
    pub fn private_key_hex(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    /// Sign a message, returning the signature as hex.
    pub fn sign(&self, message: &[u8]) -> String {
        let sig = self.signing_key.sign(message);
        hex::encode(sig.to_bytes())
    }

    /// Verify a signature against a public key.
    pub fn verify(pubkey_hex: &str, message: &[u8], signature_hex: &str) -> Result<bool> {
        let pubkey_bytes = hex::decode(pubkey_hex)
            .map_err(|e| ClawError::Signing(format!("bad pubkey hex: {e}")))?;
        let sig_bytes = hex::decode(signature_hex)
            .map_err(|e| ClawError::Signing(format!("bad sig hex: {e}")))?;

        if pubkey_bytes.len() != 32 {
            return Err(ClawError::Signing("pubkey must be 32 bytes".into()));
        }
        if sig_bytes.len() != 64 {
            return Err(ClawError::Signing("signature must be 64 bytes".into()));
        }

        let mut pk_arr = [0u8; 32];
        pk_arr.copy_from_slice(&pubkey_bytes);
        let verifying_key = VerifyingKey::from_bytes(&pk_arr)
            .map_err(|e| ClawError::Signing(format!("invalid pubkey: {e}")))?;

        let mut sig_arr = [0u8; 64];
        sig_arr.copy_from_slice(&sig_bytes);
        let signature = ed25519_dalek::Signature::from_bytes(&sig_arr);

        Ok(verifying_key.verify_strict(message, &signature).is_ok())
    }
}

// ── Node Client ─────────────────────────────────────────────────

/// RustChain node client for mining operations.
///
/// Provides health checks, balance queries, attestation,
/// and epoch enrollment.
pub struct NodeClient {
    base_url: String,
    http: Client,
}

impl NodeClient {
    /// Create a new node client pointing at the given base URL.
    ///
    /// Default node: `https://rustchain.org`
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: Client::builder()
                .user_agent("ClawRTC/0.1.0 (Rust; Elyan Labs)")
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Check node health.
    pub fn health(&self) -> Result<NodeHealth> {
        let resp: NodeHealth = self
            .http
            .get(format!("{}/health", self.base_url))
            .send()?
            .json()?;
        Ok(resp)
    }

    /// Query RTC balance for a wallet address.
    pub fn balance(&self, wallet: &str) -> Result<f64> {
        let resp: BalanceResponse = self
            .http
            .get(format!("{}/api/balance?wallet={wallet}", self.base_url))
            .send()?
            .json()?;
        Ok(resp.balance_rtc)
    }

    /// List active miners on the network.
    pub fn miners(&self) -> Result<Vec<MinerInfo>> {
        let resp: Vec<MinerInfo> = self
            .http
            .get(format!("{}/api/miners", self.base_url))
            .send()?
            .json()?;
        Ok(resp)
    }

    /// Request an attestation challenge nonce.
    pub fn challenge(&self) -> Result<Challenge> {
        let resp: Challenge = self
            .http
            .post(format!("{}/attest/challenge", self.base_url))
            .json(&serde_json::json!({}))
            .send()?
            .json()?;
        Ok(resp)
    }

    /// Submit a hardware attestation.
    pub fn attest(&self, payload: &serde_json::Value) -> Result<AttestResponse> {
        let resp: serde_json::Value = self
            .http
            .post(format!("{}/attest/submit", self.base_url))
            .json(payload)
            .send()?
            .json()?;

        if resp.get("ok").and_then(|v| v.as_bool()) == Some(true) {
            Ok(serde_json::from_value(resp)?)
        } else {
            Err(ClawError::Node(
                resp.get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("attestation failed")
                    .to_string(),
            ))
        }
    }

    /// Enroll in the current epoch for reward eligibility.
    pub fn enroll(
        &self,
        wallet: &str,
        miner_id: &str,
        arch: &CpuArch,
    ) -> Result<EnrollResponse> {
        let payload = serde_json::json!({
            "miner_pubkey": wallet,
            "miner_id": miner_id,
            "device": {
                "family": arch.family(),
                "arch": arch.arch_str(),
            }
        });

        let resp: serde_json::Value = self
            .http
            .post(format!("{}/epoch/enroll", self.base_url))
            .json(&payload)
            .send()?
            .json()?;

        if resp.get("ok").and_then(|v| v.as_bool()) == Some(true) {
            Ok(serde_json::from_value(resp)?)
        } else {
            Err(ClawError::Node(
                resp.get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("enrollment failed")
                    .to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_generation() {
        let wallet = Wallet::generate();
        assert!(wallet.address().starts_with("RTC"));
        assert_eq!(wallet.address().len(), 43); // "RTC" + 40 hex chars
    }

    #[test]
    fn test_wallet_roundtrip() {
        let w1 = Wallet::generate();
        let w2 = Wallet::from_hex(&w1.private_key_hex()).unwrap();
        assert_eq!(w1.address(), w2.address());
        assert_eq!(w1.public_key_hex(), w2.public_key_hex());
    }

    #[test]
    fn test_sign_verify() {
        let wallet = Wallet::generate();
        let msg = b"transfer 100 RTC";
        let sig = wallet.sign(msg);
        assert!(Wallet::verify(&wallet.public_key_hex(), msg, &sig).unwrap());
    }

    #[test]
    fn test_arch_multipliers() {
        assert_eq!(CpuArch::G4.multiplier(), 2.5);
        assert_eq!(CpuArch::G5.multiplier(), 2.0);
        assert_eq!(CpuArch::Modern.multiplier(), 1.0);
    }

    #[test]
    fn test_node_client_creation() {
        let client = NodeClient::new("https://rustchain.org/");
        assert_eq!(client.base_url, "https://rustchain.org");
    }
}
