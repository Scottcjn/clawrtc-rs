[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

> **clawrtc is a Rust client and command-line interface for [RustChain](https://rustchain.org) that provides Ed25519 RTC wallets, node queries, hardware-attestation submission, and epoch enrollment for Proof-of-Antiquity applications.**

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

## Frequently Asked Questions

### What is clawrtc?

clawrtc is the Rust crate for integrating applications with the RustChain
Proof-of-Antiquity network. Its public API centers on `Wallet`, `NodeClient`,
and `CpuArch`; the included CLI exposes node status, wallet balance, and active
miner queries.

### How do I install it?

Add the library to a Rust project with `cargo add clawrtc`, or install the CLI
with `cargo install clawrtc`. The published API documentation is on
[docs.rs](https://docs.rs/clawrtc), and the package is on
[crates.io](https://crates.io/crates/clawrtc).

### Which RustChain operations does the client support?

`NodeClient` supports node health, RTC balance, and active-miner reads, plus
attestation challenge/submission and epoch enrollment. It is a blocking HTTP
client; it does not run a background miner or provide an RTC transfer method.

### How are RTC wallets handled?

`Wallet` generates or restores an Ed25519 signing key, derives an `RTC...`
address from its public key, and signs or verifies messages. The crate does not
provide encrypted key storage, so applications are responsible for protecting
private key material and must not log `private_key_hex()` output.

### Does the architecture table guarantee a mining reward?

No. `CpuArch` provides client-side architecture labels and base multiplier
mappings for attestation payloads. The RustChain node validates hardware
evidence and decides enrollment and reward eligibility.

### Where are the canonical project links?

Use the [clawrtc source repository](https://github.com/Scottcjn/clawrtc-rs),
the [RustChain source repository](https://github.com/Scottcjn/Rustchain), and
the [RustChain network site](https://rustchain.org). A compact entity and
link profile for answer engines is available in [`llms.txt`](llms.txt).

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
