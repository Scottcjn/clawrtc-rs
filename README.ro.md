[![BCOS Certificat](https://img.shields.io/badge/BCOS-Certificat-brightgreen?style=flat)](BCOS.md)

# clawrtc

Client Rust pentru mineritul RTC pe [RustChain](https://rustchain.org): atestare hardware, portofele Ed25519 și consens Proof-of-Antiquity (PoA).

## Funcționalități

- **Portofele Ed25519**: generare, semnare și verificare cu adrese RTC
- **Client de nod**: verificări de stare, interogări de sold, liste de mineri
- **Atestare hardware**: trimite dovezi PoA pentru a câștiga RTC
- **Înscriere în epocă**: înregistrare pentru distribuirea recompenselor
- **Detectarea arhitecturii**: maparea multiplicatorilor CPU (G4=2.5x, G5=2.0x etc.)

## Pornire rapidă

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generează un portofel nou
    let wallet = Wallet::generate();
    println!("Adresă: {}", wallet.address());
    println!("Cheie publică: {}", wallet.public_key_hex());

    // Semnează un mesaj
    let sig = wallet.sign(b"salut rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"salut rustchain", &sig).unwrap();
    println!("Semnătură validă: {}", valid);

    // Conectează-te la rețea
    let node = NodeClient::new("https://rustchain.org");

    // Verifică starea
    let health = node.health().unwrap();
    println!("Nod v{} (timp activ {}s)", health.version, health.uptime_s);

    // Verifică soldul
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Sold: {} RTC", balance);

    // Verifică multiplicatorii de vechime
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Multiplicatori de vechime

| Arhitectură | Multiplicator | Exemple |
|-------------|---------------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium timpuriu |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | x86_64 actual, aarch64 |

## Licență

MIT - [Elyan Labs](https://rustchain.org)
