[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust հաճախորդ RTC մայնինգի համար [RustChain](https://rustchain.org)-ում՝ ապարատային attestացիա, Ed25519 դրամապանակներ և Proof-of-Antiquity (PoA) կոնսենսուս։

## Հնարավորություններ

- **Ed25519 դրամապանակներ**: ստեղծում, ստորագրում և ստուգում RTC հասցեներով
- **Հանգույցի հաճախորդ**: առողջության ստուգումներ, մնացորդի հարցումներ և մայնինգ կատարողների ցուցակներ
- **Ապարատային attestացիա**: PoA ապացույցների ուղարկում RTC վաստակելու համար
- **Էպոխայի գրանցում**: գրանցում պարգևների բաշխման համար
- **Ճարտարապետության հայտնաբերում**: CPU բազմապատկիչների քարտեզագրում (G4=2.5x, G5=2.0x և այլն)

## Արագ մեկնարկ

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Ստեղծել նոր դրամապանակ
    let wallet = Wallet::generate();
    println!("Հասցե: {}", wallet.address());
    println!("Հանրային բանալի: {}", wallet.public_key_hex());

    // Ստորագրել հաղորդագրություն
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Ստորագրությունը վավեր է: {}", valid);

    // Միանալ ցանցին
    let node = NodeClient::new("https://rustchain.org");

    // Ստուգել առողջությունը
    let health = node.health().unwrap();
    println!("Հանգույց v{} (աշխատաժամանակ {}s)", health.version, health.uptime_s);

    // Ստուգել մնացորդը
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Մնացորդ: {} RTC", balance);

    // Ստուգել ապարատային տարիքի բազմապատկիչները
    println!("G4 բոնուս: {}x", CpuArch::G4.multiplier());
    println!("G5 բոնուս: {}x", CpuArch::G5.multiplier());
}
```

## Ապարատային տարիքի բազմապատկիչներ

| Ճարտարապետություն | Բազմապատկիչ | Օրինակներ |
|-------------------|-------------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, վաղ Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Ժամանակակից | 1.0x | ընթացիկ x86_64, aarch64 |

## Լիցենզիա

MIT - [Elyan Labs](https://rustchain.org)
