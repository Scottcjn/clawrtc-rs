[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Klien Rust untuk melombong RTC di [RustChain](https://rustchain.org): pengesahan perkakasan, dompet Ed25519 dan konsensus Proof-of-Antiquity (PoA).

## Ciri-ciri

- **Dompet Ed25519**: jana, tandatangan dan sahkan dengan alamat RTC
- **Klien nod**: semakan kesihatan, pertanyaan baki dan senarai pelombong
- **Pengesahan perkakasan**: hantar bukti PoA untuk memperoleh RTC
- **Pendaftaran epok**: daftar untuk agihan ganjaran
- **Pengesanan seni bina**: pemetaan pengganda CPU (G4=2.5x, G5=2.0x dan seterusnya)

## Mula pantas

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Jana dompet baharu
    let wallet = Wallet::generate();
    println!("Alamat: {}", wallet.address());
    println!("Kunci awam: {}", wallet.public_key_hex());

    // Tandatangan mesej
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Tandatangan sah: {}", valid);

    // Sambung ke rangkaian
    let node = NodeClient::new("https://rustchain.org");

    // Semak kesihatan
    let health = node.health().unwrap();
    println!("Nod v{} (masa hidup {}s)", health.version, health.uptime_s);

    // Semak baki
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Baki: {} RTC", balance);

    // Semak pengganda usia perkakasan
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Pengganda usia perkakasan

| Seni bina | Pengganda | Contoh |
|-----------|-----------|--------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium awal |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moden | 1.0x | x86_64 semasa, aarch64 |

## Lesen

MIT - [Elyan Labs](https://rustchain.org)
