[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Mteja wa Rust kwa kuchimba RTC kwenye [RustChain](https://rustchain.org): uthibitishaji wa vifaa, pochi za Ed25519 na makubaliano ya Proof-of-Antiquity (PoA).

## Vipengele

- **Pochi za Ed25519**: tengeneza, tia saini na thibitisha kwa anwani za RTC
- **Mteja wa nodi**: ukaguzi wa afya, maswali ya salio na orodha za wachimbaji
- **Uthibitishaji wa vifaa**: tuma ushahidi wa PoA ili kupata RTC
- **Usajili wa epoki**: jisajili kwa mgawanyo wa zawadi
- **Utambuzi wa usanifu**: ulinganishaji wa vizidishi vya CPU (G4=2.5x, G5=2.0x n.k.)

## Kuanza haraka

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Tengeneza pochi mpya
    let wallet = Wallet::generate();
    println!("Anwani: {}", wallet.address());
    println!("Ufunguo wa umma: {}", wallet.public_key_hex());

    // Tia saini ujumbe
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Saini ni halali: {}", valid);

    // Unganisha kwenye mtandao
    let node = NodeClient::new("https://rustchain.org");

    // Kagua afya
    let health = node.health().unwrap();
    println!("Nodi v{} (muda wa kufanya kazi {}s)", health.version, health.uptime_s);

    // Kagua salio
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Salio: {} RTC", balance);

    // Kagua vizidishi vya umri wa vifaa
    println!("Bonasi ya G4: {}x", CpuArch::G4.multiplier());
    println!("Bonasi ya G5: {}x", CpuArch::G5.multiplier());
}
```

## Vizidishi vya umri wa vifaa

| Usanifu | Kizidishi | Mifano |
|---------|-----------|--------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium ya mapema |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Kisasa | 1.0x | x86_64 ya sasa, aarch64 |

## Leseni

MIT - [Elyan Labs](https://rustchain.org)
