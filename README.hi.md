[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) पर RTC माइनिंग के लिए Rust क्लाइंट: हार्डवेयर सत्यापन, Ed25519 वॉलेट और Proof-of-Antiquity (PoA) सहमति।

## विशेषताएं

- **Ed25519 वॉलेट**: RTC पतों के साथ बनाना, हस्ताक्षर करना और सत्यापित करना
- **नोड क्लाइंट**: स्वास्थ्य जांच, बैलेंस क्वेरी और माइनर सूचियां
- **हार्डवेयर सत्यापन**: RTC कमाने के लिए PoA प्रमाण जमा करना
- **एपोक पंजीकरण**: पुरस्कार वितरण के लिए पंजीकरण
- **आर्किटेक्चर पहचान**: CPU गुणक मैपिंग (G4=2.5x, G5=2.0x आदि)

## त्वरित शुरुआत

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // नया वॉलेट बनाएं
    let wallet = Wallet::generate();
    println!("पता: {}", wallet.address());
    println!("सार्वजनिक कुंजी: {}", wallet.public_key_hex());

    // संदेश पर हस्ताक्षर करें
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("हस्ताक्षर वैध: {}", valid);

    // नेटवर्क से जुड़ें
    let node = NodeClient::new("https://rustchain.org");

    // स्वास्थ्य जांचें
    let health = node.health().unwrap();
    println!("नोड v{} (अपटाइम {}s)", health.version, health.uptime_s);

    // बैलेंस जांचें
    let balance = node.balance(&wallet.address()).unwrap();
    println!("बैलेंस: {} RTC", balance);

    // हार्डवेयर आयु गुणक जांचें
    println!("G4 बोनस: {}x", CpuArch::G4.multiplier());
    println!("G5 बोनस: {}x", CpuArch::G5.multiplier());
}
```

## हार्डवेयर आयु गुणक

| आर्किटेक्चर | गुणक | उदाहरण |
|-------------|------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, शुरुआती Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| आधुनिक | 1.0x | वर्तमान x86_64, aarch64 |

## लाइसेंस

MIT - [Elyan Labs](https://rustchain.org)
