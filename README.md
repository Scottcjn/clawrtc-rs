# clawrtc

Rust client for [RustChain](https://rustchain.org) RTC mining — hardware attestation, Ed25519 wallets, and Proof-of-Antiquity (PoA) consensus.

## Features

- **Ed25519 Wallets** — generate, sign, and verify with RTC addresses
- **Node Client** — health checks, balance queries, miner listings
- **Hardware Attestation** — submit PoA proofs to earn RTC
- **Epoch Enrollment** — register for reward distribution
- **Architecture Detection** — CPU multiplier mapping (G4=2.5x, G5=2.0x, etc.)

## Quick Start

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generate a new wallet
    let wallet = Wallet::generate();
    println!("Address: {}", wallet.address());
    println!("Public Key: {}", wallet.public_key_hex());

    // Sign a message
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Signature valid: {}", valid);

    // Connect to the network
    let node = NodeClient::new("https://rustchain.org");

    // Check health
    let health = node.health().unwrap();
    println!("Node v{} (uptime {}s)", health.version, health.uptime_s);

    // Check balance
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balance: {} RTC", balance);

    // Check antiquity multipliers
    println!("G4 bonus: {}x", CpuArch::G4.multiplier());
    println!("G5 bonus: {}x", CpuArch::G5.multiplier());
}
```

## Antiquity Multipliers

| Architecture | Multiplier | Examples |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, early Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Current x86_64, aarch64 |

## License

MIT — [Elyan Labs](https://rustchain.org)
