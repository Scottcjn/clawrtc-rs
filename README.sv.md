[![BCOS Certifierad](https://img.shields.io/badge/BCOS-Certifierad-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-klient för RTC-mining på [RustChain](https://rustchain.org): hårdvaruintyg, Ed25519-plånböcker och Proof-of-Antiquity (PoA)-konsensus.

## Funktioner

- **Ed25519-plånböcker**: generera, signera och verifiera med RTC-adresser
- **Nodklient**: hälsokontroller, saldofrågor och miner-listor
- **Hårdvaruintyg**: skicka PoA-bevis för att tjäna RTC
- **Epokregistrering**: registrera dig för belöningsfördelning
- **Arkitekturdetektering**: CPU-multiplikatormappning (G4=2.5x, G5=2.0x osv.)

## Snabbstart

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generera en ny plånbok
    let wallet = Wallet::generate();
    println!("Adress: {}", wallet.address());
    println!("Publik nyckel: {}", wallet.public_key_hex());

    // Signera ett meddelande
    let sig = wallet.sign(b"hej rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hej rustchain", &sig).unwrap();
    println!("Signatur giltig: {}", valid);

    // Anslut till nätverket
    let node = NodeClient::new("https://rustchain.org");

    // Kontrollera hälsa
    let health = node.health().unwrap();
    println!("Nod v{} (drifttid {}s)", health.version, health.uptime_s);

    // Kontrollera saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Kontrollera antikvitetsmultiplikatorer
    println!("G4-bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-bonus: {}x", CpuArch::G5.multiplier());
}
```

## Antikvitetsmultiplikatorer

| Arkitektur | Multiplikator | Exempel |
|------------|---------------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, tidig Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Nuvarande x86_64, aarch64 |

## Licens

MIT - [Elyan Labs](https://rustchain.org)
