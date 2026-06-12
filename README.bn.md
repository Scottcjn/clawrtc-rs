[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org)-এ RTC মাইনিংয়ের জন্য Rust ক্লায়েন্ট: হার্ডওয়্যার অ্যাটেস্টেশন, Ed25519 ওয়ালেট এবং Proof-of-Antiquity (PoA) কনসেনসাস।

## বৈশিষ্ট্য

- **Ed25519 ওয়ালেট**: RTC ঠিকানা দিয়ে তৈরি, স্বাক্ষর এবং যাচাই
- **নোড ক্লায়েন্ট**: স্বাস্থ্য পরীক্ষা, ব্যালেন্স অনুসন্ধান এবং মাইনার তালিকা
- **হার্ডওয়্যার অ্যাটেস্টেশন**: RTC অর্জনের জন্য PoA প্রমাণ জমা
- **ইপোক নিবন্ধন**: পুরস্কার বিতরণের জন্য নিবন্ধন
- **আর্কিটেকচার শনাক্তকরণ**: CPU গুণক ম্যাপিং (G4=2.5x, G5=2.0x ইত্যাদি)

## দ্রুত শুরু

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // নতুন ওয়ালেট তৈরি করুন
    let wallet = Wallet::generate();
    println!("ঠিকানা: {}", wallet.address());
    println!("পাবলিক কী: {}", wallet.public_key_hex());

    // বার্তা স্বাক্ষর করুন
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("স্বাক্ষর বৈধ: {}", valid);

    // নেটওয়ার্কে সংযোগ করুন
    let node = NodeClient::new("https://rustchain.org");

    // স্বাস্থ্য পরীক্ষা করুন
    let health = node.health().unwrap();
    println!("নোড v{} (আপটাইম {}s)", health.version, health.uptime_s);

    // ব্যালেন্স দেখুন
    let balance = node.balance(&wallet.address()).unwrap();
    println!("ব্যালেন্স: {} RTC", balance);

    // হার্ডওয়্যার বয়স গুণক পরীক্ষা করুন
    println!("G4 বোনাস: {}x", CpuArch::G4.multiplier());
    println!("G5 বোনাস: {}x", CpuArch::G5.multiplier());
}
```

## হার্ডওয়্যার বয়স গুণক

| আর্কিটেকচার | গুণক | উদাহরণ |
|-------------|------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, প্রাথমিক Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| আধুনিক | 1.0x | বর্তমান x86_64, aarch64 |

## লাইসেন্স

MIT - [Elyan Labs](https://rustchain.org)
