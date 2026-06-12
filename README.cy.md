[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Cleient Rust ar gyfer cloddio RTC ar [RustChain](https://rustchain.org): ardystiad caledwedd, waledi Ed25519 a chonsensws Proof-of-Antiquity (PoA).

## Nodweddion

- **Waledi Ed25519**: cynhyrchu, llofnodi a gwirio gyda chyfeiriadau RTC
- **Cleient nod**: gwiriadau iechyd, ymholiadau balans a rhestrau cloddwyr
- **Ardystiad caledwedd**: cyflwyno prawf PoA i ennill RTC
- **Cofrestru epoc**: cofrestru ar gyfer dosbarthiad gwobrau
- **Canfod pensaernïaeth**: mapio lluosyddion CPU (G4=2.5x, G5=2.0x ac ati)

## Cychwyn cyflym

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Cynhyrchu waled newydd
    let wallet = Wallet::generate();
    println!("Cyfeiriad: {}", wallet.address());
    println!("Allwedd gyhoeddus: {}", wallet.public_key_hex());

    // Llofnodi neges
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Llofnod dilys: {}", valid);

    // Cysylltu â'r rhwydwaith
    let node = NodeClient::new("https://rustchain.org");

    // Gwirio iechyd
    let health = node.health().unwrap();
    println!("Nod v{} (amser i fyny {}s)", health.version, health.uptime_s);

    // Gwirio balans
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balans: {} RTC", balance);

    // Gwirio lluosyddion oedran caledwedd
    println!("Bonws G4: {}x", CpuArch::G4.multiplier());
    println!("Bonws G5: {}x", CpuArch::G5.multiplier());
}
```

## Lluosyddion oedran caledwedd

| Pensaernïaeth | Lluosydd | Enghreifftiau |
|---------------|----------|---------------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium cynnar |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | x86_64 cyfredol, aarch64 |

## Trwydded

MIT - [Elyan Labs](https://rustchain.org)
