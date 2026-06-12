[![BCOS Сертифіковано](https://img.shields.io/badge/BCOS-%D0%A1%D0%B5%D1%80%D1%82%D0%B8%D1%84%D1%96%D0%BA%D0%BE%D0%B2%D0%B0%D0%BD%D0%BE-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-клієнт для майнінгу RTC у [RustChain](https://rustchain.org): апаратна атестація, гаманці Ed25519 і консенсус Proof-of-Antiquity (PoA).

## Можливості

- **Гаманці Ed25519**: генерування, підписування й перевірка з RTC-адресами
- **Клієнт вузла**: перевірки стану, запити балансу, списки майнерів
- **Апаратна атестація**: надсилання PoA-доказів для заробітку RTC
- **Реєстрація в епосі**: запис для розподілу винагород
- **Визначення архітектури**: мапінг CPU-множників (G4=2.5x, G5=2.0x тощо)

## Швидкий старт

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Згенерувати новий гаманець
    let wallet = Wallet::generate();
    println!("Адреса: {}", wallet.address());
    println!("Публічний ключ: {}", wallet.public_key_hex());

    // Підписати повідомлення
    let sig = wallet.sign(b"pryvit rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"pryvit rustchain", &sig).unwrap();
    println!("Підпис чинний: {}", valid);

    // Під'єднатися до мережі
    let node = NodeClient::new("https://rustchain.org");

    // Перевірити стан
    let health = node.health().unwrap();
    println!("Вузол v{} (час роботи {}s)", health.version, health.uptime_s);

    // Перевірити баланс
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Баланс: {} RTC", balance);

    // Перевірити множники давності обладнання
    println!("Бонус G4: {}x", CpuArch::G4.multiplier());
    println!("Бонус G5: {}x", CpuArch::G5.multiplier());
}
```

## Множники давності обладнання

| Архітектура | Множник | Приклади |
|-------------|---------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ранній Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Сучасна | 1.0x | Поточні x86_64, aarch64 |

## Ліцензія

MIT - [Elyan Labs](https://rustchain.org)
