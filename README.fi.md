[![BCOS Sertifioitu](https://img.shields.io/badge/BCOS-Sertifioitu-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-asiakasohjelma RTC-louhintaan [RustChainissa](https://rustchain.org): laitteistoattestointi, Ed25519-lompakot ja Proof-of-Antiquity (PoA) -konsensus.

## Ominaisuudet

- **Ed25519-lompakot**: luo, allekirjoita ja vahvista RTC-osoitteilla
- **Solmuasiakas**: terveystarkistukset, saldokyselyt ja louhijalistaukset
- **Laitteistoattestointi**: lähetä PoA-todisteita RTC:n ansaitsemiseksi
- **Epookkiin ilmoittautuminen**: rekisteröidy palkkioiden jakoon
- **Arkkitehtuurin tunnistus**: CPU-kertoimien kartoitus (G4=2.5x, G5=2.0x jne.)

## Pika-aloitus

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Luo uusi lompakko
    let wallet = Wallet::generate();
    println!("Osoite: {}", wallet.address());
    println!("Julkinen avain: {}", wallet.public_key_hex());

    // Allekirjoita viesti
    let sig = wallet.sign(b"hei rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hei rustchain", &sig).unwrap();
    println!("Allekirjoitus kelpaa: {}", valid);

    // Yhdistä verkkoon
    let node = NodeClient::new("https://rustchain.org");

    // Tarkista tila
    let health = node.health().unwrap();
    println!("Solmu v{} (käyntiaika {}s)", health.version, health.uptime_s);

    // Tarkista saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Tarkista ikäpohjaiset kertoimet
    println!("G4-bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-bonus: {}x", CpuArch::G5.multiplier());
}
```

## Ikäpohjaiset kertoimet

| Arkkitehtuuri | Kerroin | Esimerkit |
|---------------|---------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, varhainen Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderni | 1.0x | Nykyinen x86_64, aarch64 |

## Lisenssi

MIT - [Elyan Labs](https://rustchain.org)
