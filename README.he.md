[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

לקוח Rust לכריית RTC ב-[RustChain](https://rustchain.org): אימות חומרה, ארנקי Ed25519 וקונצנזוס Proof-of-Antiquity (PoA).

## תכונות

- **ארנקי Ed25519**: יצירה, חתימה ואימות עם כתובות RTC
- **לקוח צומת**: בדיקות תקינות, שאילתות יתרה ורשימות כורים
- **אימות חומרה**: שליחת הוכחות PoA כדי להרוויח RTC
- **רישום תקופה**: הרשמה לחלוקת תגמולים
- **זיהוי ארכיטקטורה**: מיפוי מכפילי CPU (G4=2.5x, G5=2.0x וכו')

## התחלה מהירה

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // יצירת ארנק חדש
    let wallet = Wallet::generate();
    println!("כתובת: {}", wallet.address());
    println!("מפתח ציבורי: {}", wallet.public_key_hex());

    // חתימה על הודעה
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("חתימה תקפה: {}", valid);

    // התחברות לרשת
    let node = NodeClient::new("https://rustchain.org");

    // בדיקת תקינות
    let health = node.health().unwrap();
    println!("צומת v{} (זמן פעולה {}s)", health.version, health.uptime_s);

    // בדיקת יתרה
    let balance = node.balance(&wallet.address()).unwrap();
    println!("יתרה: {} RTC", balance);

    // בדיקת מכפילי גיל חומרה
    println!("בונוס G4: {}x", CpuArch::G4.multiplier());
    println!("בונוס G5: {}x", CpuArch::G5.multiplier());
}
```

## מכפילי גיל חומרה

| ארכיטקטורה | מכפיל | דוגמאות |
|------------|-------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium מוקדם |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| מודרני | 1.0x | x86_64 נוכחי, aarch64 |

## רישיון

MIT - [Elyan Labs](https://rustchain.org)
