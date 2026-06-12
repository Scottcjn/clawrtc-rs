[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-kliënt vir RTC-mynbou op [RustChain](https://rustchain.org): hardeware-attestasie, Ed25519-beursies en Proof-of-Antiquity (PoA)-konsensus.

## Kenmerke

- **Ed25519-beursies**: genereer, onderteken en verifieer met RTC-adresse
- **Node-kliënt**: gesondheidskontroles, saldo-navrae en mynwerkerlyste
- **Hardeware-attestasie**: dien PoA-bewyse in om RTC te verdien
- **Epoch-registrasie**: registreer vir beloningsverspreiding
- **Argitektuuropsporing**: CPU-vermenigvuldigerkartering (G4=2.5x, G5=2.0x, ens.)

## Vinnige begin

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Genereer 'n nuwe beursie
    let wallet = Wallet::generate();
    println!("Adres: {}", wallet.address());
    println!("Publieke sleutel: {}", wallet.public_key_hex());

    // Onderteken 'n boodskap
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Handtekening geldig: {}", valid);

    // Koppel aan die netwerk
    let node = NodeClient::new("https://rustchain.org");

    // Kontroleer gesondheid
    let health = node.health().unwrap();
    println!("Node v{} (optyd {}s)", health.version, health.uptime_s);

    // Kontroleer saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Kontroleer hardeware-ouderdomsvermenigvuldigers
    println!("G4-bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-bonus: {}x", CpuArch::G5.multiplier());
}
```

## Hardeware-ouderdomsvermenigvuldigers

| Argitektuur | Vermenigvuldiger | Voorbeelde |
|-------------|------------------|------------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, vroeë Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Huidige x86_64, aarch64 |

## Lisensie

MIT - [Elyan Labs](https://rustchain.org)
