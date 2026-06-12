[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust klijent za rudarenje RTC-a na [RustChain](https://rustchain.org): hardverska atestacija, Ed25519 novčanici i Proof-of-Antiquity (PoA) konsenzus.

## Funkcije

- **Ed25519 novčanici**: generisanje, potpisivanje i provera sa RTC adresama
- **Klijent čvora**: provere stanja, upiti balansa i liste rudara
- **Hardverska atestacija**: slanje PoA dokaza za zaradu RTC-a
- **Registracija epohe**: prijava za raspodelu nagrada
- **Otkrivanje arhitekture**: mapiranje CPU množilaca (G4=2.5x, G5=2.0x itd.)

## Brzi početak

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generiši novi novčanik
    let wallet = Wallet::generate();
    println!("Adresa: {}", wallet.address());
    println!("Javni ključ: {}", wallet.public_key_hex());

    // Potpiši poruku
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Potpis važeći: {}", valid);

    // Poveži se na mrežu
    let node = NodeClient::new("https://rustchain.org");

    // Proveri stanje
    let health = node.health().unwrap();
    println!("Čvor v{} (vreme rada {}s)", health.version, health.uptime_s);

    // Proveri balans
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balans: {} RTC", balance);

    // Proveri množioce starosti hardvera
    println!("G4 bonus: {}x", CpuArch::G4.multiplier());
    println!("G5 bonus: {}x", CpuArch::G5.multiplier());
}
```

## Množioci starosti hardvera

| Arhitektura | Množilac | Primeri |
|-------------|----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, rani Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderna | 1.0x | Trenutni x86_64, aarch64 |

## Licenca

MIT - [Elyan Labs](https://rustchain.org)
