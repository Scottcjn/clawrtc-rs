[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust kliens RTC bányászathoz a [RustChain](https://rustchain.org) hálózaton: hardveres hitelesítés, Ed25519 tárcák és Proof-of-Antiquity (PoA) konszenzus.

## Funkciók

- **Ed25519 tárcák**: létrehozás, aláírás és ellenőrzés RTC-címekkel
- **Csomópont-kliens**: állapotellenőrzések, egyenleglekérdezések és bányászlisták
- **Hardveres hitelesítés**: PoA-bizonyítékok beküldése RTC szerzéséhez
- **Epoch-regisztráció**: regisztráció jutalomosztáshoz
- **Architektúra-felismerés**: CPU-szorzók leképezése (G4=2.5x, G5=2.0x stb.)

## Gyors kezdés

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Új tárca létrehozása
    let wallet = Wallet::generate();
    println!("Cím: {}", wallet.address());
    println!("Nyilvános kulcs: {}", wallet.public_key_hex());

    // Üzenet aláírása
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Aláírás érvényes: {}", valid);

    // Csatlakozás a hálózathoz
    let node = NodeClient::new("https://rustchain.org");

    // Állapot ellenőrzése
    let health = node.health().unwrap();
    println!("Csomópont v{} (üzemidő {}s)", health.version, health.uptime_s);

    // Egyenleg ellenőrzése
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Egyenleg: {} RTC", balance);

    // Hardverkor-szorzók ellenőrzése
    println!("G4 bónusz: {}x", CpuArch::G4.multiplier());
    println!("G5 bónusz: {}x", CpuArch::G5.multiplier());
}
```

## Hardverkor-szorzók

| Architektúra | Szorzó | Példák |
|--------------|--------|--------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, korai Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Jelenlegi x86_64, aarch64 |

## Licenc

MIT - [Elyan Labs](https://rustchain.org)
