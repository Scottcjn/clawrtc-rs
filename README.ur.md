[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) پر RTC مائننگ کے لیے Rust کلائنٹ: ہارڈویئر اٹیسٹیشن، Ed25519 والٹس اور Proof-of-Antiquity (PoA) اتفاق رائے۔

## خصوصیات

- **Ed25519 والٹس**: RTC پتوں کے ساتھ بنانا، دستخط کرنا اور تصدیق کرنا
- **نوڈ کلائنٹ**: صحت کی جانچ، بیلنس سوالات اور مائنر فہرستیں
- **ہارڈویئر اٹیسٹیشن**: RTC کمانے کے لیے PoA ثبوت جمع کروانا
- **ایپوک رجسٹریشن**: انعامات کی تقسیم کے لیے رجسٹر کرنا
- **آرکیٹیکچر شناخت**: CPU ملٹی پلائر میپنگ (G4=2.5x، G5=2.0x وغیرہ)

## فوری آغاز

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // نیا والٹ بنائیں
    let wallet = Wallet::generate();
    println!("پتہ: {}", wallet.address());
    println!("عوامی کلید: {}", wallet.public_key_hex());

    // پیغام پر دستخط کریں
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("دستخط درست: {}", valid);

    // نیٹ ورک سے جڑیں
    let node = NodeClient::new("https://rustchain.org");

    // صحت چیک کریں
    let health = node.health().unwrap();
    println!("نوڈ v{} (اپ ٹائم {}s)", health.version, health.uptime_s);

    // بیلنس چیک کریں
    let balance = node.balance(&wallet.address()).unwrap();
    println!("بیلنس: {} RTC", balance);

    // ہارڈویئر عمر کے ملٹی پلائر چیک کریں
    println!("G4 بونس: {}x", CpuArch::G4.multiplier());
    println!("G5 بونس: {}x", CpuArch::G5.multiplier());
}
```

## ہارڈویئر عمر کے ملٹی پلائر

| آرکیٹیکچر | ملٹی پلائر | مثالیں |
|-----------|-------------|--------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ابتدائی Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| جدید | 1.0x | موجودہ x86_64, aarch64 |

## لائسنس

MIT - [Elyan Labs](https://rustchain.org)
