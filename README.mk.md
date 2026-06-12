[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust клиент за рударење RTC на [RustChain](https://rustchain.org): хардверска атестација, Ed25519 паричници и Proof-of-Antiquity (PoA) консензус.

## Функции

- **Ed25519 паричници**: генерирање, потпишување и проверка со RTC адреси
- **Клиент за јазол**: здравствени проверки, барања за салдо и листи на рудари
- **Хардверска атестација**: испраќање PoA докази за заработка на RTC
- **Регистрација на епоха**: регистрација за распределба на награди
- **Откривање архитектура**: мапирање CPU множители (G4=2.5x, G5=2.0x итн.)

## Брз почеток

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Генерирај нов паричник
    let wallet = Wallet::generate();
    println!("Адреса: {}", wallet.address());
    println!("Јавен клуч: {}", wallet.public_key_hex());

    // Потпиши порака
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Потписот е валиден: {}", valid);

    // Поврзи се на мрежата
    let node = NodeClient::new("https://rustchain.org");

    // Провери здравје
    let health = node.health().unwrap();
    println!("Јазол v{} (време на работа {}s)", health.version, health.uptime_s);

    // Провери салдо
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Салдо: {} RTC", balance);

    // Провери множители за старост на хардверот
    println!("G4 бонус: {}x", CpuArch::G4.multiplier());
    println!("G5 бонус: {}x", CpuArch::G5.multiplier());
}
```

## Множители за старост на хардверот

| Архитектура | Множител | Примери |
|-------------|----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ран Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Модерна | 1.0x | Тековни x86_64, aarch64 |

## Лиценца

MIT - [Elyan Labs](https://rustchain.org)
