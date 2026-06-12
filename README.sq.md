[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Klient Rust për nxjerrjen e RTC në [RustChain](https://rustchain.org): vërtetim hardueri, kuleta Ed25519 dhe konsensus Proof-of-Antiquity (PoA).

## Veçori

- **Kuleta Ed25519**: gjenerim, nënshkrim dhe verifikim me adresa RTC
- **Klient nyjeje**: kontrolle shëndeti, pyetje bilanci dhe lista minatorësh
- **Vërtetim hardueri**: dërgim provash PoA për të fituar RTC
- **Regjistrim epoke**: regjistrim për shpërndarje shpërblimesh
- **Zbulim arkitekture**: hartëzim shumëzuesish CPU (G4=2.5x, G5=2.0x etj.)

## Nisje e shpejtë

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Gjenero një kuletë të re
    let wallet = Wallet::generate();
    println!("Adresa: {}", wallet.address());
    println!("Çelësi publik: {}", wallet.public_key_hex());

    // Nënshkruaj një mesazh
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Nënshkrimi i vlefshëm: {}", valid);

    // Lidhu me rrjetin
    let node = NodeClient::new("https://rustchain.org");

    // Kontrollo shëndetin
    let health = node.health().unwrap();
    println!("Nyja v{} (koha aktive {}s)", health.version, health.uptime_s);

    // Kontrollo bilancin
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Bilanci: {} RTC", balance);

    // Kontrollo shumëzuesit e moshës së harduerit
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Shumëzuesit e moshës së harduerit

| Arkitektura | Shumëzuesi | Shembuj |
|-------------|------------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium i hershëm |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | x86_64 aktual, aarch64 |

## Licenca

MIT - [Elyan Labs](https://rustchain.org)
