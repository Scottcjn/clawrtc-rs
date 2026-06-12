[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) tarmog'ida RTC qazish uchun Rust mijozi: apparat attestatsiyasi, Ed25519 hamyonlari va Proof-of-Antiquity (PoA) konsensusi.

## Xususiyatlar

- **Ed25519 hamyonlari**: RTC manzillari bilan yaratish, imzolash va tekshirish
- **Tugun mijozi**: sog'liq tekshiruvlari, balans so'rovlari va konchi ro'yxatlari
- **Apparat attestatsiyasi**: RTC olish uchun PoA dalillarini yuborish
- **Epoxa ro'yxati**: mukofot taqsimoti uchun ro'yxatdan o'tish
- **Arxitektura aniqlash**: CPU ko'paytirgichlarini xaritalash (G4=2.5x, G5=2.0x va hokazo)

## Tez boshlash

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Yangi hamyon yarating
    let wallet = Wallet::generate();
    println!("Manzil: {}", wallet.address());
    println!("Ochiq kalit: {}", wallet.public_key_hex());

    // Xabarni imzolang
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Imzo haqiqiy: {}", valid);

    // Tarmoqqa ulaning
    let node = NodeClient::new("https://rustchain.org");

    // Sog'liqni tekshiring
    let health = node.health().unwrap();
    println!("Tugun v{} (ish vaqti {}s)", health.version, health.uptime_s);

    // Balansni tekshiring
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balans: {} RTC", balance);

    // Apparat yoshi ko'paytirgichlarini tekshiring
    println!("G4 bonusi: {}x", CpuArch::G4.multiplier());
    println!("G5 bonusi: {}x", CpuArch::G5.multiplier());
}
```

## Apparat yoshi ko'paytirgichlari

| Arxitektura | Ko'paytirgich | Misollar |
|-------------|---------------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ilk Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Zamonaviy | 1.0x | Hozirgi x86_64, aarch64 |

## Litsenziya

MIT - [Elyan Labs](https://rustchain.org)
