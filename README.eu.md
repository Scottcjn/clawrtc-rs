[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) sarean RTC meatzaritzarako Rust bezeroa: hardware-ziurtapena, Ed25519 diru-zorroak eta Proof-of-Antiquity (PoA) adostasuna.

## Ezaugarriak

- **Ed25519 diru-zorroak**: RTC helbideekin sortu, sinatu eta egiaztatu
- **Nodo-bezeroa**: osasun-egiaztapenak, saldo-kontsultak eta meatzari-zerrendak
- **Hardware-ziurtapena**: PoA frogak bidali RTC irabazteko
- **Epoka-erregistroa**: sarien banaketan izena ematea
- **Arkitektura-detekzioa**: CPU biderkatzaileen mapaketa (G4=2.5x, G5=2.0x, etab.)

## Hasiera azkarra

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Sortu diru-zorro berri bat
    let wallet = Wallet::generate();
    println!("Helbidea: {}", wallet.address());
    println!("Gako publikoa: {}", wallet.public_key_hex());

    // Sinatu mezu bat
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Sinadura baliozkoa: {}", valid);

    // Konektatu sarera
    let node = NodeClient::new("https://rustchain.org");

    // Egiaztatu osasuna
    let health = node.health().unwrap();
    println!("Nodoa v{} (martxan {}s)", health.version, health.uptime_s);

    // Egiaztatu saldoa
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldoa: {} RTC", balance);

    // Egiaztatu hardware-adinaren biderkatzaileak
    println!("G4 hobaria: {}x", CpuArch::G4.multiplier());
    println!("G5 hobaria: {}x", CpuArch::G5.multiplier());
}
```

## Hardware-adinaren biderkatzaileak

| Arkitektura | Biderkatzailea | Adibideak |
|-------------|----------------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, lehen Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modernoa | 1.0x | Egungo x86_64, aarch64 |

## Lizentzia

MIT - [Elyan Labs](https://rustchain.org)
