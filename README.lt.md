[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust klientas RTC kasimui [RustChain](https://rustchain.org): aparatinės įrangos atestavimas, Ed25519 piniginės ir Proof-of-Antiquity (PoA) konsensusas.

## Funkcijos

- **Ed25519 piniginės**: generavimas, pasirašymas ir tikrinimas su RTC adresais
- **Mazgo klientas**: būsenos patikros, balanso užklausos ir kasėjų sąrašai
- **Aparatinės įrangos atestavimas**: PoA įrodymų teikimas RTC uždirbti
- **Epochos registracija**: registracija atlygių paskirstymui
- **Architektūros aptikimas**: CPU daugiklių susiejimas (G4=2.5x, G5=2.0x ir t. t.)

## Greita pradžia

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Sukurti naują piniginę
    let wallet = Wallet::generate();
    println!("Adresas: {}", wallet.address());
    println!("Viešasis raktas: {}", wallet.public_key_hex());

    // Pasirašyti pranešimą
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Parašas galioja: {}", valid);

    // Prisijungti prie tinklo
    let node = NodeClient::new("https://rustchain.org");

    // Patikrinti būseną
    let health = node.health().unwrap();
    println!("Mazgas v{} (veikimo laikas {}s)", health.version, health.uptime_s);

    // Patikrinti balansą
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balansas: {} RTC", balance);

    // Patikrinti aparatinės įrangos amžiaus daugiklius
    println!("G4 premija: {}x", CpuArch::G4.multiplier());
    println!("G5 premija: {}x", CpuArch::G5.multiplier());
}
```

## Aparatinės įrangos amžiaus daugikliai

| Architektūra | Daugiklis | Pavyzdžiai |
|--------------|-----------|------------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ankstyvas Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderni | 1.0x | Dabartiniai x86_64, aarch64 |

## Licencija

MIT - [Elyan Labs](https://rustchain.org)
