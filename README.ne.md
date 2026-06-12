[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) मा RTC माइनिङका लागि Rust क्लाइन्ट: हार्डवेयर अटेस्टेशन, Ed25519 वालेट र Proof-of-Antiquity (PoA) सहमति।

## सुविधाहरू

- **Ed25519 वालेट**: RTC ठेगानासँग सिर्जना, हस्ताक्षर र प्रमाणीकरण
- **नोड क्लाइन्ट**: स्वास्थ्य जाँच, ब्यालेन्स क्वेरी र माइनर सूचीहरू
- **हार्डवेयर अटेस्टेशन**: RTC कमाउन PoA प्रमाण पेश गर्ने
- **इपोक दर्ता**: पुरस्कार वितरणका लागि दर्ता
- **आर्किटेक्चर पहिचान**: CPU गुणक म्यापिङ (G4=2.5x, G5=2.0x आदि)

## छिटो सुरु

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // नयाँ वालेट सिर्जना गर्नुहोस्
    let wallet = Wallet::generate();
    println!("ठेगाना: {}", wallet.address());
    println!("सार्वजनिक कुञ्जी: {}", wallet.public_key_hex());

    // सन्देशमा हस्ताक्षर गर्नुहोस्
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("हस्ताक्षर मान्य: {}", valid);

    // नेटवर्कमा जडान गर्नुहोस्
    let node = NodeClient::new("https://rustchain.org");

    // स्वास्थ्य जाँच गर्नुहोस्
    let health = node.health().unwrap();
    println!("नोड v{} (अपटाइम {}s)", health.version, health.uptime_s);

    // ब्यालेन्स जाँच गर्नुहोस्
    let balance = node.balance(&wallet.address()).unwrap();
    println!("ब्यालेन्स: {} RTC", balance);

    // हार्डवेयर उमेर गुणक जाँच गर्नुहोस्
    println!("G4 बोनस: {}x", CpuArch::G4.multiplier());
    println!("G5 बोनस: {}x", CpuArch::G5.multiplier());
}
```

## हार्डवेयर उमेर गुणक

| आर्किटेक्चर | गुणक | उदाहरणहरू |
|-------------|------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, सुरुको Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| आधुनिक | 1.0x | हालको x86_64, aarch64 |

## इजाजतपत्र

MIT - [Elyan Labs](https://rustchain.org)
