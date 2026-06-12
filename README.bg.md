[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust клиент за копаене на RTC в [RustChain](https://rustchain.org): хардуерна атестация, Ed25519 портфейли и консенсус Proof-of-Antiquity (PoA).

## Функции

- **Ed25519 портфейли**: генериране, подписване и проверка с RTC адреси
- **Клиент за възел**: проверки на състоянието, заявки за баланс и списъци с миньори
- **Хардуерна атестация**: изпращане на PoA доказателства за печелене на RTC
- **Регистрация за епоха**: записване за разпределение на награди
- **Откриване на архитектура**: картографиране на CPU множители (G4=2.5x, G5=2.0x и др.)

## Бърз старт

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Генериране на нов портфейл
    let wallet = Wallet::generate();
    println!("Адрес: {}", wallet.address());
    println!("Публичен ключ: {}", wallet.public_key_hex());

    // Подписване на съобщение
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Подписът е валиден: {}", valid);

    // Свързване към мрежата
    let node = NodeClient::new("https://rustchain.org");

    // Проверка на състоянието
    let health = node.health().unwrap();
    println!("Възел v{} (време на работа {}s)", health.version, health.uptime_s);

    // Проверка на баланс
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Баланс: {} RTC", balance);

    // Проверка на множителите за възраст на хардуера
    println!("G4 бонус: {}x", CpuArch::G4.multiplier());
    println!("G5 бонус: {}x", CpuArch::G5.multiplier());
}
```

## Множители за възраст на хардуера

| Архитектура | Множител | Примери |
|-------------|----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ранен Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Съвременна | 1.0x | Текущи x86_64, aarch64 |

## Лиценз

MIT - [Elyan Labs](https://rustchain.org)
