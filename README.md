[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

**clawrtc** is the official Rust client library and CLI for [RustChain](https://rustchain.org) RTC mining, providing type-safe Ed25519 wallet management, hardware attestation, and Proof-of-Antiquity (PoA) consensus integration.

## What is clawrtc?

clawrtc enables developers and miners to interact with the RustChain network programmatically. It handles wallet generation, transaction signing, node health checks, balance queries, miner listings, and hardware attestation required to earn RTC tokens through Proof-of-Antiquity mining on vintage or modern hardware.

## FAQ

### How do I generate a new RTC wallet?
Use `Wallet::generate()` to create an Ed25519 keypair. The resulting address starts with `RTC` and can be used for balance queries and attestation submissions.

### How do I check my RTC balance?
Create a `NodeClient` pointing to `https://rustchain.org` (or a custom node) and call `balance(&wallet.address())`. Balances are returned as `f64` RTC values.

### What hardware qualifies for Proof-of-Antiquity bonuses?
Vintage architectures receive multiplicative rewards: PowerPC G4 (2.5x), G5 (2.0x), G3 (1.8x), Pentium 4 (1.5x), Retro x86 (1.4x), Core 2 Duo (1.3x), Apple Silicon (1.2x). Modern x86_64/aarch64 runs at 1.0x baseline. See the Antiquity Multipliers table below for full details.

### How do I use the CLI?
Run `clawrtc status` for node health, `clawrtc wallet balance RTC_ADDRESS` for balance queries, or `clawrtc miner stats --json` for machine-readable miner listings. Add `--node URL` to override the default endpoint.

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

    // List miners. Current and legacy /api/miners response shapes are supported.
    for miner in node.miners().unwrap() {
        println!("{} last seen {}", miner.miner, miner.last_seen);
    }

    // Check balance
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balance: {} RTC", balance);

    // Check antiquity multipliers
    println!("G4 bonus: {}x", CpuArch::G4.multiplier());
    println!("G5 bonus: {}x", CpuArch::G5.multiplier());
}
```

## CLI

The global `--node` option defaults to `https://rustchain.org`. Add `--json` to
the status, balance, or miner commands for machine-readable output.

```bash
clawrtc status
clawrtc status --wallet RTC_ADDRESS --json
clawrtc wallet balance RTC_ADDRESS
clawrtc miner stats --json
```

`miner stats` reads `/api/miners`. The client accepts both the legacy bare list
and the current `{ "miners": [...], "pagination": {...} }` envelope. The public
`MinerInfo.last_seen` field remains a string: legacy `last_seen` text is kept as
is, while the current numeric `last_attest` Unix timestamp is converted to its
decimal string form.

`NodeClient::balance` and `wallet balance` use the current
`/wallet/balance?miner_id=...` endpoint. The node's `amount_rtc` response field
is returned by the Rust API as `f64`.

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

## Development

```bash
cargo fmt --all -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## License

MIT — [Elyan Labs](https://rustchain.org)


## RustChain bounty #16256

[BOUNTY: 10 RTC] clawrtc-rs: integration test suite — wallet roundtrip, address vectors, attestation determinism, error paths

Submitted by 0wmz. See the bounty issue for scope.
