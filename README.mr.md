[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) वर RTC मायनिंगसाठी Rust क्लायंट: हार्डवेअर अटेस्टेशन, Ed25519 वॉलेट्स आणि Proof-of-Antiquity (PoA) सहमती.

## वैशिष्ट्ये

- **Ed25519 वॉलेट्स**: RTC पत्त्यांसह तयार करणे, स्वाक्षरी करणे आणि पडताळणे
- **नोड क्लायंट**: आरोग्य तपासणी, शिल्लक प्रश्न आणि मायनर याद्या
- **हार्डवेअर अटेस्टेशन**: RTC मिळवण्यासाठी PoA पुरावे सादर करणे
- **एपॉक नोंदणी**: बक्षीस वितरणासाठी नोंदणी
- **आर्किटेक्चर शोध**: CPU गुणक मॅपिंग (G4=2.5x, G5=2.0x इ.)

## जलद सुरुवात

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // नवीन वॉलेट तयार करा
    let wallet = Wallet::generate();
    println!("पत्ता: {}", wallet.address());
    println!("सार्वजनिक की: {}", wallet.public_key_hex());

    // संदेशावर स्वाक्षरी करा
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("स्वाक्षरी वैध: {}", valid);

    // नेटवर्कला जोडा
    let node = NodeClient::new("https://rustchain.org");

    // आरोग्य तपासा
    let health = node.health().unwrap();
    println!("नोड v{} (अपटाइम {}s)", health.version, health.uptime_s);

    // शिल्लक तपासा
    let balance = node.balance(&wallet.address()).unwrap();
    println!("शिल्लक: {} RTC", balance);

    // हार्डवेअर वय गुणक तपासा
    println!("G4 बोनस: {}x", CpuArch::G4.multiplier());
    println!("G5 बोनस: {}x", CpuArch::G5.multiplier());
}
```

## हार्डवेअर वय गुणक

| आर्किटेक्चर | गुणक | उदाहरणे |
|-------------|------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, सुरुवातीचा Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| आधुनिक | 1.0x | सध्याचे x86_64, aarch64 |

## परवाना

MIT - [Elyan Labs](https://rustchain.org)
