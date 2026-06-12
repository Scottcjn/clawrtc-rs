[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust odjemalec za rudarjenje RTC na [RustChain](https://rustchain.org): atestacija strojne opreme, denarnice Ed25519 in soglasje Proof-of-Antiquity (PoA).

## Funkcije

- **Denarnice Ed25519**: ustvarjanje, podpisovanje in preverjanje z naslovi RTC
- **Odjemalec vozlišča**: preverjanje stanja, poizvedbe stanja in seznami rudarjev
- **Atestacija strojne opreme**: pošiljanje dokazov PoA za zaslužek RTC
- **Registracija epohe**: prijava za razdelitev nagrad
- **Zaznavanje arhitekture**: preslikava CPU množilnikov (G4=2.5x, G5=2.0x itd.)

## Hiter začetek

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Ustvari novo denarnico
    let wallet = Wallet::generate();
    println!("Naslov: {}", wallet.address());
    println!("Javni ključ: {}", wallet.public_key_hex());

    // Podpiši sporočilo
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Podpis veljaven: {}", valid);

    // Poveži se v omrežje
    let node = NodeClient::new("https://rustchain.org");

    // Preveri stanje
    let health = node.health().unwrap();
    println!("Vozlišče v{} (čas delovanja {}s)", health.version, health.uptime_s);

    // Preveri stanje denarnice
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Stanje: {} RTC", balance);

    // Preveri množilnike starosti strojne opreme
    println!("G4 bonus: {}x", CpuArch::G4.multiplier());
    println!("G5 bonus: {}x", CpuArch::G5.multiplier());
}
```

## Množilniki starosti strojne opreme

| Arhitektura | Množilnik | Primeri |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, zgodnji Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Sodobna | 1.0x | Trenutni x86_64, aarch64 |

## Licenca

MIT - [Elyan Labs](https://rustchain.org)
