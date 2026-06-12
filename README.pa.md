[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) ਉੱਤੇ RTC ਮਾਈਨਿੰਗ ਲਈ Rust ਕਲਾਇੰਟ: ਹਾਰਡਵੇਅਰ ਅਟੈਸਟੇਸ਼ਨ, Ed25519 ਵਾਲਿਟ ਅਤੇ Proof-of-Antiquity (PoA) ਕਨਸੈਂਸਸ।

## ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ

- **Ed25519 ਵਾਲਿਟ**: RTC ਪਤਿਆਂ ਨਾਲ ਬਣਾਉਣਾ, ਸਾਈਨ ਕਰਨਾ ਅਤੇ ਜਾਂਚਣਾ
- **ਨੋਡ ਕਲਾਇੰਟ**: ਸਿਹਤ ਜਾਂਚ, ਬੈਲੈਂਸ ਕਵੈਰੀਆਂ ਅਤੇ ਮਾਈਨਰ ਸੂਚੀਆਂ
- **ਹਾਰਡਵੇਅਰ ਅਟੈਸਟੇਸ਼ਨ**: RTC ਕਮਾਉਣ ਲਈ PoA ਸਬੂਤ ਭੇਜਣਾ
- **ਏਪੋਕ ਰਜਿਸਟ੍ਰੇਸ਼ਨ**: ਇਨਾਮ ਵੰਡ ਲਈ ਰਜਿਸਟਰ ਕਰਨਾ
- **ਆਰਕੀਟੈਕਚਰ ਪਛਾਣ**: CPU ਗੁਣਕ ਮੈਪਿੰਗ (G4=2.5x, G5=2.0x ਆਦਿ)

## ਤੇਜ਼ ਸ਼ੁਰੂਆਤ

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // ਨਵਾਂ ਵਾਲਿਟ ਬਣਾਓ
    let wallet = Wallet::generate();
    println!("ਪਤਾ: {}", wallet.address());
    println!("ਪਬਲਿਕ ਕੀ: {}", wallet.public_key_hex());

    // ਸੁਨੇਹੇ 'ਤੇ ਸਾਈਨ ਕਰੋ
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("ਸਾਈਨਚਰ ਵੈਧ: {}", valid);

    // ਨੈੱਟਵਰਕ ਨਾਲ ਜੁੜੋ
    let node = NodeClient::new("https://rustchain.org");

    // ਸਿਹਤ ਜਾਂਚੋ
    let health = node.health().unwrap();
    println!("ਨੋਡ v{} (ਅਪਟਾਈਮ {}s)", health.version, health.uptime_s);

    // ਬੈਲੈਂਸ ਜਾਂਚੋ
    let balance = node.balance(&wallet.address()).unwrap();
    println!("ਬੈਲੈਂਸ: {} RTC", balance);

    // ਹਾਰਡਵੇਅਰ ਉਮਰ ਗੁਣਕ ਜਾਂਚੋ
    println!("G4 ਬੋਨਸ: {}x", CpuArch::G4.multiplier());
    println!("G5 ਬੋਨਸ: {}x", CpuArch::G5.multiplier());
}
```

## ਹਾਰਡਵੇਅਰ ਉਮਰ ਗੁਣਕ

| ਆਰਕੀਟੈਕਚਰ | ਗੁਣਕ | ਉਦਾਹਰਨਾਂ |
|-----------|------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, ਸ਼ੁਰੂਆਤੀ Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| ਆਧੁਨਿਕ | 1.0x | ਮੌਜੂਦਾ x86_64, aarch64 |

## ਲਾਇਸੈਂਸ

MIT - [Elyan Labs](https://rustchain.org)
