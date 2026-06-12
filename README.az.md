[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) üzərində RTC mədənçiliyi üçün Rust müştərisi: aparat attestasiya, Ed25519 pul kisələri və Proof-of-Antiquity (PoA) konsensusu.

## Xüsusiyyətlər

- **Ed25519 pul kisələri**: RTC ünvanları ilə yaratma, imzalama və yoxlama
- **Düyün müştərisi**: sağlamlıq yoxlamaları, balans sorğuları və mədənçi siyahıları
- **Aparat attestasiya**: RTC qazanmaq üçün PoA sübutlarının göndərilməsi
- **Epoxa qeydiyyatı**: mükafat paylanması üçün qeydiyyat
- **Arxitektura aşkarlanması**: CPU çarpan xəritəsi (G4=2.5x, G5=2.0x və s.)

## Sürətli başlanğıc

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Yeni pul kisəsi yaradın
    let wallet = Wallet::generate();
    println!("Ünvan: {}", wallet.address());
    println!("Açıq açar: {}", wallet.public_key_hex());

    // Mesajı imzalayın
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("İmza etibarlıdır: {}", valid);

    // Şəbəkəyə qoşulun
    let node = NodeClient::new("https://rustchain.org");

    // Sağlamlığı yoxlayın
    let health = node.health().unwrap();
    println!("Düyün v{} (işləmə vaxtı {}s)", health.version, health.uptime_s);

    // Balansı yoxlayın
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balans: {} RTC", balance);

    // Aparat yaş çarpanlarını yoxlayın
    println!("G4 bonusu: {}x", CpuArch::G4.multiplier());
    println!("G5 bonusu: {}x", CpuArch::G5.multiplier());
}
```

## Aparat yaş çarpanları

| Arxitektura | Çarpan | Nümunələr |
|-------------|-------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, erkən Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Müasir | 1.0x | Cari x86_64, aarch64 |

## Lisenziya

MIT - [Elyan Labs](https://rustchain.org)
