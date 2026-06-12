[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-biðlari fyrir RTC-námuvinnslu á [RustChain](https://rustchain.org): vélbúnaðarstaðfesting, Ed25519-veski og Proof-of-Antiquity (PoA)-samstaða.

## Eiginleikar

- **Ed25519-veski**: búa til, undirrita og staðfesta með RTC-vistföngum
- **Hnútabiðlari**: heilsuprófanir, stöðufyrirspurnir og námumannalistar
- **Vélbúnaðarstaðfesting**: senda PoA-sannanir til að vinna sér inn RTC
- **Skeiðsskráning**: skráning fyrir verðlaunaúthlutun
- **Arkitektúrgreining**: kortlagning CPU-margfaldara (G4=2.5x, G5=2.0x o.s.frv.)

## Fljót byrjun

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Búa til nýtt veski
    let wallet = Wallet::generate();
    println!("Vistfang: {}", wallet.address());
    println!("Opinber lykill: {}", wallet.public_key_hex());

    // Undirrita skilaboð
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Undirskrift gild: {}", valid);

    // Tengjast netinu
    let node = NodeClient::new("https://rustchain.org");

    // Athuga heilsu
    let health = node.health().unwrap();
    println!("Hnútur v{} (uppitími {}s)", health.version, health.uptime_s);

    // Athuga stöðu
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Staða: {} RTC", balance);

    // Athuga aldursmargfaldara vélbúnaðar
    println!("G4-bónus: {}x", CpuArch::G4.multiplier());
    println!("G5-bónus: {}x", CpuArch::G5.multiplier());
}
```

## Aldursmargfaldarar vélbúnaðar

| Arkitektúr | Margfaldari | Dæmi |
|------------|-------------|------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, snemma Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Nútímalegt | 1.0x | Núverandi x86_64, aarch64 |

## Leyfi

MIT - [Elyan Labs](https://rustchain.org)
