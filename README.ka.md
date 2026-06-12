[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust კლიენტი RTC მაინინგისთვის [RustChain](https://rustchain.org)-ზე: აპარატურის ატესტაცია, Ed25519 საფულეები და Proof-of-Antiquity (PoA) კონსენსუსი.

## შესაძლებლობები

- **Ed25519 საფულეები**: შექმნა, ხელმოწერა და შემოწმება RTC მისამართებით
- **კვანძის კლიენტი**: ჯანმრთელობის შემოწმება, ბალანსის მოთხოვნები და მაინერების სიები
- **აპარატურის ატესტაცია**: PoA მტკიცებულებების გაგზავნა RTC-ის მოსაპოვებლად
- **ეპოქის რეგისტრაცია**: ჯილდოების განაწილებისთვის რეგისტრაცია
- **არქიტექტურის აღმოჩენა**: CPU გამრავლების რუკა (G4=2.5x, G5=2.0x და სხვ.)

## სწრაფი დაწყება

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // შექმენით ახალი საფულე
    let wallet = Wallet::generate();
    println!("მისამართი: {}", wallet.address());
    println!("საჯარო გასაღები: {}", wallet.public_key_hex());

    // მოაწერეთ ხელი შეტყობინებას
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("ხელმოწერა სწორია: {}", valid);

    // დაუკავშირდით ქსელს
    let node = NodeClient::new("https://rustchain.org");

    // შეამოწმეთ ჯანმრთელობა
    let health = node.health().unwrap();
    println!("კვანძი v{} (მუშაობის დრო {}s)", health.version, health.uptime_s);

    // შეამოწმეთ ბალანსი
    let balance = node.balance(&wallet.address()).unwrap();
    println!("ბალანსი: {} RTC", balance);

    // შეამოწმეთ აპარატურის ასაკის გამრავლები
    println!("G4 ბონუსი: {}x", CpuArch::G4.multiplier());
    println!("G5 ბონუსი: {}x", CpuArch::G5.multiplier());
}
```

## აპარატურის ასაკის გამრავლები

| არქიტექტურა | გამრავლი | მაგალითები |
|-------------|----------|------------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ადრეული Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| თანამედროვე | 1.0x | მიმდინარე x86_64, aarch64 |

## ლიცენზია

MIT - [Elyan Labs](https://rustchain.org)
