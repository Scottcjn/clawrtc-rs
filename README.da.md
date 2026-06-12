[![BCOS Certificeret](https://img.shields.io/badge/BCOS-Certificeret-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-klient til RTC-mining på [RustChain](https://rustchain.org): hardwareattestation, Ed25519-wallets og Proof-of-Antiquity (PoA)-konsensus.

## Funktioner

- **Ed25519-wallets**: generér, signer og verificér med RTC-adresser
- **Node-klient**: sundhedstjek, saldoforespørgsler og miner-lister
- **Hardwareattestation**: indsend PoA-beviser for at tjene RTC
- **Epoketilmelding**: registrering til belønningsfordeling
- **Arkitekturdetektion**: CPU-multiplikatormapping (G4=2.5x, G5=2.0x osv.)

## Hurtig start

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generér en ny wallet
    let wallet = Wallet::generate();
    println!("Adresse: {}", wallet.address());
    println!("Offentlig nøgle: {}", wallet.public_key_hex());

    // Signér en besked
    let sig = wallet.sign(b"hej rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hej rustchain", &sig).unwrap();
    println!("Signatur gyldig: {}", valid);

    // Opret forbindelse til netværket
    let node = NodeClient::new("https://rustchain.org");

    // Tjek sundhed
    let health = node.health().unwrap();
    println!("Node v{} (oppetid {}s)", health.version, health.uptime_s);

    // Tjek saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Tjek antikvitetsmultiplikatorer
    println!("G4-bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-bonus: {}x", CpuArch::G5.multiplier());
}
```

## Antikvitetsmultiplikatorer

| Arkitektur | Multiplikator | Eksempler |
|------------|---------------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, tidlig Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderne | 1.0x | Nuværende x86_64, aarch64 |

## Licens

MIT - [Elyan Labs](https://rustchain.org)
