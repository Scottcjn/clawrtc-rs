[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) પર RTC માઇનિંગ માટે Rust ક્લાયન્ટ: હાર્ડવેર એટેસ્ટેશન, Ed25519 વૉલેટ અને Proof-of-Antiquity (PoA) કન્સેન્સસ.

## સુવિધાઓ

- **Ed25519 વૉલેટ**: RTC સરનામાં સાથે જનરેટ, સહી અને ચકાસણી
- **નોડ ક્લાયન્ટ**: હેલ્થ ચેક, બેલેન્સ ક્વેરી અને માઇનર યાદીઓ
- **હાર્ડવેર એટેસ્ટેશન**: RTC કમાવવા માટે PoA પુરાવા સબમિટ કરવું
- **એપોક નોંધણી**: ઇનામ વિતરણ માટે નોંધણી
- **આર્કિટેક્ચર શોધ**: CPU ગુણક મેપિંગ (G4=2.5x, G5=2.0x વગેરે)

## ઝડપી શરૂઆત

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // નવું વૉલેટ બનાવો
    let wallet = Wallet::generate();
    println!("સરનામું: {}", wallet.address());
    println!("જાહેર કી: {}", wallet.public_key_hex());

    // સંદેશ પર સહી કરો
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("સહી માન્ય: {}", valid);

    // નેટવર્ક સાથે જોડાઓ
    let node = NodeClient::new("https://rustchain.org");

    // હેલ્થ તપાસો
    let health = node.health().unwrap();
    println!("નોડ v{} (અપટાઇમ {}s)", health.version, health.uptime_s);

    // બેલેન્સ તપાસો
    let balance = node.balance(&wallet.address()).unwrap();
    println!("બેલેન્સ: {} RTC", balance);

    // હાર્ડવેર ઉંમર ગુણકો તપાસો
    println!("G4 બોનસ: {}x", CpuArch::G4.multiplier());
    println!("G5 બોનસ: {}x", CpuArch::G5.multiplier());
}
```

## હાર્ડવેર ઉંમર ગુણકો

| આર્કિટેક્ચર | ગુણક | ઉદાહરણો |
|-------------|------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, શરૂઆતનું Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| આધુનિક | 1.0x | હાલનું x86_64, aarch64 |

## લાઇસન્સ

MIT - [Elyan Labs](https://rustchain.org)
