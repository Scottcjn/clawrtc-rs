[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

عميل Rust لتعدين RTC على [RustChain](https://rustchain.org): إثبات العتاد، محافظ Ed25519، وتوافق Proof-of-Antiquity (PoA).

## الميزات

- **محافظ Ed25519**: إنشاء وتوقيع وتحقق باستخدام عناوين RTC
- **عميل العقدة**: فحوصات الصحة، استعلامات الرصيد، وقوائم المعدنين
- **إثبات العتاد**: إرسال أدلة PoA لكسب RTC
- **تسجيل الحقبة**: التسجيل لتوزيع المكافآت
- **اكتشاف المعمارية**: ربط مضاعفات CPU (مثل G4=2.5x و G5=2.0x)

## البدء السريع

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // إنشاء محفظة جديدة
    let wallet = Wallet::generate();
    println!("العنوان: {}", wallet.address());
    println!("المفتاح العام: {}", wallet.public_key_hex());

    // توقيع رسالة
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("التوقيع صالح: {}", valid);

    // الاتصال بالشبكة
    let node = NodeClient::new("https://rustchain.org");

    // فحص الصحة
    let health = node.health().unwrap();
    println!("العقدة v{} (مدة التشغيل {}s)", health.version, health.uptime_s);

    // فحص الرصيد
    let balance = node.balance(&wallet.address()).unwrap();
    println!("الرصيد: {} RTC", balance);

    // فحص مضاعفات عمر العتاد
    println!("مكافأة G4: {}x", CpuArch::G4.multiplier());
    println!("مكافأة G5: {}x", CpuArch::G5.multiplier());
}
```

## مضاعفات عمر العتاد

| المعمارية | المضاعف | أمثلة |
|-----------|---------|-------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium مبكر |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| حديث | 1.0x | x86_64 الحالي، aarch64 |

## الترخيص

MIT - [Elyan Labs](https://rustchain.org)
