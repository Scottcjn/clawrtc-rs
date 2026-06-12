[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org)లో RTC మైనింగ్ కోసం Rust క్లయింట్: హార్డ్‌వేర్ ధృవీకరణ, Ed25519 వాలెట్లు మరియు Proof-of-Antiquity (PoA) కన్సెన్సస్.

## లక్షణాలు

- **Ed25519 వాలెట్లు**: RTC చిరునామాలతో సృష్టించడం, సంతకం చేయడం మరియు ధృవీకరించడం
- **నోడ్ క్లయింట్**: ఆరోగ్య తనిఖీలు, బ్యాలెన్స్ ప్రశ్నలు మరియు మైనర్ జాబితాలు
- **హార్డ్‌వేర్ ధృవీకరణ**: RTC సంపాదించేందుకు PoA ఆధారాలు సమర్పించడం
- **ఎపాక్ నమోదు**: బహుమతి పంపిణీకి నమోదు
- **ఆర్కిటెక్చర్ గుర్తింపు**: CPU గుణక మ్యాపింగ్ (G4=2.5x, G5=2.0x మొదలైనవి)

## త్వరిత ప్రారంభం

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // కొత్త వాలెట్ సృష్టించండి
    let wallet = Wallet::generate();
    println!("చిరునామా: {}", wallet.address());
    println!("పబ్లిక్ కీ: {}", wallet.public_key_hex());

    // సందేశంపై సంతకం చేయండి
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("సంతకం చెల్లుబాటు: {}", valid);

    // నెట్‌వర్క్‌కు కనెక్ట్ అవ్వండి
    let node = NodeClient::new("https://rustchain.org");

    // ఆరోగ్యాన్ని తనిఖీ చేయండి
    let health = node.health().unwrap();
    println!("నోడ్ v{} (అప్‌టైమ్ {}s)", health.version, health.uptime_s);

    // బ్యాలెన్స్ తనిఖీ చేయండి
    let balance = node.balance(&wallet.address()).unwrap();
    println!("బ్యాలెన్స్: {} RTC", balance);

    // హార్డ్‌వేర్ వయస్సు గుణకాలను తనిఖీ చేయండి
    println!("G4 బోనస్: {}x", CpuArch::G4.multiplier());
    println!("G5 బోనస్: {}x", CpuArch::G5.multiplier());
}
```

## హార్డ్‌వేర్ వయస్సు గుణకాలు

| ఆర్కిటెక్చర్ | గుణకం | ఉదాహరణలు |
|--------------|-------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ప్రారంభ Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| ఆధునిక | 1.0x | ప్రస్తుత x86_64, aarch64 |

## లైసెన్స్

MIT - [Elyan Labs](https://rustchain.org)
