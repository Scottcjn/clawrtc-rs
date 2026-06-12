[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) இல் RTC சுரங்கத்திற்கான Rust கிளையன்ட்: வன்பொருள் சான்றளிப்பு, Ed25519 பணப்பைகள் மற்றும் Proof-of-Antiquity (PoA) ஒப்புமை.

## அம்சங்கள்

- **Ed25519 பணப்பைகள்**: RTC முகவரிகளுடன் உருவாக்குதல், கையொப்பமிடுதல் மற்றும் சரிபார்த்தல்
- **நோடு கிளையன்ட்**: ஆரோக்கியச் சோதனைகள், இருப்பு கேள்விகள் மற்றும் சுரங்கர் பட்டியல்கள்
- **வன்பொருள் சான்றளிப்பு**: RTC சம்பாதிக்க PoA ஆதாரங்களை சமர்ப்பித்தல்
- **எபாக் பதிவு**: வெகுமதி பகிர்வுக்குப் பதிவு செய்தல்
- **கட்டமைப்பு கண்டறிதல்**: CPU பெருக்கி வரைபடம் (G4=2.5x, G5=2.0x போன்றவை)

## விரைவு தொடக்கம்

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // புதிய பணப்பை உருவாக்கு
    let wallet = Wallet::generate();
    println!("முகவரி: {}", wallet.address());
    println!("பொது விசை: {}", wallet.public_key_hex());

    // செய்தியை கையொப்பமிடு
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("கையொப்பம் செல்லுபடியாகும்: {}", valid);

    // வலையுடன் இணை
    let node = NodeClient::new("https://rustchain.org");

    // ஆரோக்கியம் சரிபார்
    let health = node.health().unwrap();
    println!("நோடு v{} (இயங்கிய நேரம் {}s)", health.version, health.uptime_s);

    // இருப்பு சரிபார்
    let balance = node.balance(&wallet.address()).unwrap();
    println!("இருப்பு: {} RTC", balance);

    // வன்பொருள் வயது பெருக்கிகளை சரிபார்
    println!("G4 போனஸ்: {}x", CpuArch::G4.multiplier());
    println!("G5 போனஸ்: {}x", CpuArch::G5.multiplier());
}
```

## வன்பொருள் வயது பெருக்கிகள்

| கட்டமைப்பு | பெருக்கி | உதாரணங்கள் |
|------------|----------|------------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ஆரம்ப Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| நவீன | 1.0x | தற்போதைய x86_64, aarch64 |

## உரிமம்

MIT - [Elyan Labs](https://rustchain.org)
