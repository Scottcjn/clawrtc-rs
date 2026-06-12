[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

کلاینت Rust برای استخراج RTC روی [RustChain](https://rustchain.org): گواهی سخت افزار، کیف پول های Ed25519 و اجماع Proof-of-Antiquity (PoA).

## قابلیت ها

- **کیف پول های Ed25519**: ساخت، امضا و تایید با آدرس های RTC
- **کلاینت نود**: بررسی سلامت، پرس وجوی موجودی و فهرست ماینرها
- **گواهی سخت افزار**: ارسال اثبات های PoA برای کسب RTC
- **ثبت نام اپوک**: ثبت نام برای توزیع پاداش
- **تشخیص معماری**: نگاشت ضریب های CPU (G4=2.5x، G5=2.0x و غیره)

## شروع سریع

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // ساخت کیف پول جدید
    let wallet = Wallet::generate();
    println!("آدرس: {}", wallet.address());
    println!("کلید عمومی: {}", wallet.public_key_hex());

    // امضای پیام
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("امضا معتبر است: {}", valid);

    // اتصال به شبکه
    let node = NodeClient::new("https://rustchain.org");

    // بررسی سلامت
    let health = node.health().unwrap();
    println!("نود v{} (زمان کار {}s)", health.version, health.uptime_s);

    // بررسی موجودی
    let balance = node.balance(&wallet.address()).unwrap();
    println!("موجودی: {} RTC", balance);

    // بررسی ضریب های سن سخت افزار
    println!("پاداش G4: {}x", CpuArch::G4.multiplier());
    println!("پاداش G5: {}x", CpuArch::G5.multiplier());
}
```

## ضریب های سن سخت افزار

| معماری | ضریب | نمونه ها |
|--------|------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium اولیه |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| مدرن | 1.0x | x86_64 فعلی، aarch64 |

## مجوز

MIT - [Elyan Labs](https://rustchain.org)
