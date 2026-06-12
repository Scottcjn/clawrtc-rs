[![BCOS Gecertificeerd](https://img.shields.io/badge/BCOS-Gecertificeerd-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-client voor RTC-mining op [RustChain](https://rustchain.org): hardware-attestatie, Ed25519-wallets en Proof-of-Antiquity (PoA)-consensus.

## Functies

- **Ed25519-wallets**: genereren, ondertekenen en verifiëren met RTC-adressen
- **Node-client**: statuscontroles, saldo-opvragingen en miner-lijsten
- **Hardware-attestatie**: PoA-bewijzen indienen om RTC te verdienen
- **Epoch-inschrijving**: registreren voor verdeling van beloningen
- **Architectuurdetectie**: CPU-multipliermapping (G4=2.5x, G5=2.0x, enz.)

## Snelstart

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Genereer een nieuwe wallet
    let wallet = Wallet::generate();
    println!("Adres: {}", wallet.address());
    println!("Publieke sleutel: {}", wallet.public_key_hex());

    // Onderteken een bericht
    let sig = wallet.sign(b"hallo rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hallo rustchain", &sig).unwrap();
    println!("Handtekening geldig: {}", valid);

    // Maak verbinding met het netwerk
    let node = NodeClient::new("https://rustchain.org");

    // Controleer de status
    let health = node.health().unwrap();
    println!("Node v{} (uptime {}s)", health.version, health.uptime_s);

    // Controleer het saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Controleer de antiquity-multipliers
    println!("G4-bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-bonus: {}x", CpuArch::G5.multiplier());
}
```

## Antiquity-multipliers

| Architectuur | Multiplier | Voorbeelden |
|-------------|------------|-------------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, vroege Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Huidige x86_64, aarch64 |

## Licentie

MIT - [Elyan Labs](https://rustchain.org)
