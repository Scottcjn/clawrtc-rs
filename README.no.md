[![BCOS Sertifisert](https://img.shields.io/badge/BCOS-Sertifisert-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-klient for RTC-mining på [RustChain](https://rustchain.org): maskinvareattestering, Ed25519-lommebøker og Proof-of-Antiquity (PoA)-konsensus.

## Funksjoner

- **Ed25519-lommebøker**: generer, signer og verifiser med RTC-adresser
- **Node-klient**: helsesjekker, saldoforespørsler og utvinnerlister
- **Maskinvareattestering**: send inn PoA-bevis for å tjene RTC
- **Epokeregistrering**: registrer deg for belønningsfordeling
- **Arkitekturdeteksjon**: CPU-multiplikatorkartlegging (G4=2.5x, G5=2.0x osv.)

## Hurtigstart

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generer en ny lommebok
    let wallet = Wallet::generate();
    println!("Adresse: {}", wallet.address());
    println!("Offentlig nøkkel: {}", wallet.public_key_hex());

    // Signer en melding
    let sig = wallet.sign(b"hei rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hei rustchain", &sig).unwrap();
    println!("Signatur gyldig: {}", valid);

    // Koble til nettverket
    let node = NodeClient::new("https://rustchain.org");

    // Sjekk helse
    let health = node.health().unwrap();
    println!("Node v{} (oppetid {}s)", health.version, health.uptime_s);

    // Sjekk saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Sjekk aldersmultiplikatorer
    println!("G4-bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-bonus: {}x", CpuArch::G5.multiplier());
}
```

## Aldersmultiplikatorer

| Arkitektur | Multiplikator | Eksempler |
|------------|---------------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, tidlig Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderne | 1.0x | Dagens x86_64, aarch64 |

## Lisens

MIT - [Elyan Labs](https://rustchain.org)
