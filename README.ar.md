[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

عميل Rust لتعدين RTC على [RustChain](https://rustchain.org) — يتضمن توثيق العتاد، محافظ Ed25519، وإجماع إثبات القِدم (Proof-of-Antiquity / PoA).

## الميزات

- **محافظ Ed25519** — إنشاء المحافظ والتوقيع والتحقق باستخدام عناوين RTC
- **عميل العقدة** — فحوصات الصحة، الاستعلام عن الرصيد، وقوائم المعدّنين
- **توثيق العتاد** — إرسال إثباتات PoA لكسب RTC
- **التسجيل في الحقبة** — التسجيل للمشاركة في توزيع المكافآت
- **اكتشاف المعمارية** — ربط معمارية المعالج بمعاملات المضاعفة (G4=2.5x و G5=2.0x وغيرها)

## البدء السريع

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // إنشاء محفظة جديدة
    let wallet = Wallet::generate();
    println!("Address: {}", wallet.address());
    let public_key_hex = wallet.public_key_hex();
    println!("Public Key: {}", public_key_hex);

    // توقيع رسالة
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&public_key_hex, b"hello rustchain", &sig).unwrap();
    println!("Signature valid: {}", valid);

    // الاتصال بالشبكة
    let node = NodeClient::new("https://rustchain.org");

    // فحص صحة العقدة
    let health = node.health().unwrap();
    println!("Node v{} (uptime {}s)", health.version, health.uptime_s);

    // الاستعلام عن الرصيد
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Balance: {} RTC", balance);

    // التحقق من معاملات إثبات القِدم
    println!("G4 bonus: {}x", CpuArch::G4.multiplier());
    println!("G5 bonus: {}x", CpuArch::G5.multiplier());
}
```

## معاملات إثبات القِدم

| المعمارية | معامل المضاعفة | أمثلة |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4، Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5، Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3، Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension، HP Pavilion |
| Retro x86 | 1.4x | 486، 386، إصدارات Pentium المبكرة |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1، M2، M3 |
| Modern | 1.0x | x86_64 و aarch64 الحاليتان |

## الترخيص

MIT — [Elyan Labs](https://rustchain.org)
