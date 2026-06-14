[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-клиент для майнинга RTC в [RustChain](https://rustchain.org): аппаратная аттестация, кошельки Ed25519 и консенсус Proof-of-Antiquity (PoA).

## Возможности

- **Кошельки Ed25519** — создание, подпись и проверка с адресами RTC
- **Клиент узла** — проверки состояния, запросы баланса и списки майнеров
- **Аппаратная аттестация** — отправка PoA-доказательств для получения RTC
- **Регистрация в эпохе** — запись на распределение вознаграждений
- **Определение архитектуры** — сопоставление CPU с множителями (G4=2.5x, G5=2.0x и т. д.)

## Быстрый старт

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Создать новый кошелёк
    let wallet = Wallet::generate();
    println!("Адрес: {}", wallet.address());
    println!("Публичный ключ: {}", wallet.public_key_hex());

    // Подписать сообщение
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Подпись действительна: {}", valid);

    // Подключиться к сети
    let node = NodeClient::new("https://rustchain.org");

    // Проверить состояние узла
    let health = node.health().unwrap();
    println!("Узел v{} (время работы {}s)", health.version, health.uptime_s);

    // Проверить баланс
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Баланс: {} RTC", balance);

    // Проверить множители давности оборудования
    println!("Бонус G4: {}x", CpuArch::G4.multiplier());
    println!("Бонус G5: {}x", CpuArch::G5.multiplier());
}
```

## Множители давности оборудования

| Архитектура | Множитель | Примеры |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ранние Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Современные x86_64, aarch64 |

## Лицензия

MIT — [Elyan Labs](https://rustchain.org)
