[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust klients RTC rakšanai [RustChain](https://rustchain.org): aparatūras atestācija, Ed25519 maki un Proof-of-Antiquity (PoA) konsenss.

## Iespējas

- **Ed25519 maki**: ģenerēšana, parakstīšana un pārbaude ar RTC adresēm
- **Mezgla klients**: veselības pārbaudes, bilances vaicājumi un racēju saraksti
- **Aparatūras atestācija**: PoA pierādījumu iesniegšana RTC nopelnīšanai
- **Epoha reģistrācija**: reģistrācija atlīdzību sadalei
- **Arhitektūras noteikšana**: CPU reizinātāju kartēšana (G4=2.5x, G5=2.0x utt.)

## Ātrais sākums

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Izveidot jaunu maku
    let wallet = Wallet::generate();
    println!("Adrese: {}", wallet.address());
    println!("Publiskā atslēga: {}", wallet.public_key_hex());

    // Parakstīt ziņojumu
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Paraksts derīgs: {}", valid);

    // Pieslēgties tīklam
    let node = NodeClient::new("https://rustchain.org");

    // Pārbaudīt veselību
    let health = node.health().unwrap();
    println!("Mezgls v{} (darbības laiks {}s)", health.version, health.uptime_s);

    // Pārbaudīt bilanci
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Bilance: {} RTC", balance);

    // Pārbaudīt aparatūras vecuma reizinātājus
    println!("G4 bonuss: {}x", CpuArch::G4.multiplier());
    println!("G5 bonuss: {}x", CpuArch::G5.multiplier());
}
```

## Aparatūras vecuma reizinātāji

| Arhitektūra | Reizinātājs | Piemēri |
|-------------|-------------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, agrīns Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderna | 1.0x | Pašreizējie x86_64, aarch64 |

## Licence

MIT - [Elyan Labs](https://rustchain.org)
