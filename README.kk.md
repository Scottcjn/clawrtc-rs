[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) желісінде RTC майнингіне арналған Rust клиенті: аппараттық аттестация, Ed25519 әмияндары және Proof-of-Antiquity (PoA) консенсусы.

## Мүмкіндіктер

- **Ed25519 әмияндары**: RTC мекенжайларымен жасау, қол қою және тексеру
- **Түйін клиенті**: денсаулық тексерулері, баланс сұраулары және майнер тізімдері
- **Аппараттық аттестация**: RTC табу үшін PoA дәлелдерін жіберу
- **Эпоха тіркеуі**: сыйақы үлестіруіне тіркелу
- **Архитектураны анықтау**: CPU көбейткіштерін картаға түсіру (G4=2.5x, G5=2.0x және т.б.)

## Жылдам бастау

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Жаңа әмиян жасаңыз
    let wallet = Wallet::generate();
    println!("Мекенжай: {}", wallet.address());
    println!("Ашық кілт: {}", wallet.public_key_hex());

    // Хабарламаға қол қойыңыз
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Қолтаңба жарамды: {}", valid);

    // Желіге қосылыңыз
    let node = NodeClient::new("https://rustchain.org");

    // Денсаулықты тексеріңіз
    let health = node.health().unwrap();
    println!("Түйін v{} (жұмыс уақыты {}s)", health.version, health.uptime_s);

    // Балансты тексеріңіз
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Баланс: {} RTC", balance);

    // Аппарат жас көбейткіштерін тексеріңіз
    println!("G4 бонусы: {}x", CpuArch::G4.multiplier());
    println!("G5 бонусы: {}x", CpuArch::G5.multiplier());
}
```

## Аппарат жас көбейткіштері

| Архитектура | Көбейткіш | Мысалдар |
|-------------|-----------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ерте Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Заманауи | 1.0x | Қазіргі x86_64, aarch64 |

## Лицензия

MIT - [Elyan Labs](https://rustchain.org)
