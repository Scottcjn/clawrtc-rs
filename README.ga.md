[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Cliant Rust le haghaidh mianadóireacht RTC ar [RustChain](https://rustchain.org): fianú crua-earraí, sparáin Ed25519 agus comhdhearcadh Proof-of-Antiquity (PoA).

## Gnéithe

- **Sparáin Ed25519**: giniúint, síniú agus fíorú le seoltaí RTC
- **Cliant nód**: seiceálacha sláinte, fiosruithe iarmhéid agus liostaí mianadóirí
- **Fianú crua-earraí**: cruthúnais PoA a chur isteach chun RTC a thuilleamh
- **Clárú eipeach**: clárú le haghaidh dáileadh luaíochtaí
- **Braite ailtireachta**: mapáil iolraitheoirí CPU (G4=2.5x, G5=2.0x srl.)

## Tús tapa

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Gin sparán nua
    let wallet = Wallet::generate();
    println!("Seoladh: {}", wallet.address());
    println!("Eochair phoiblí: {}", wallet.public_key_hex());

    // Sínigh teachtaireacht
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Síniú bailí: {}", valid);

    // Ceangail leis an líonra
    let node = NodeClient::new("https://rustchain.org");

    // Seiceáil sláinte
    let health = node.health().unwrap();
    println!("Nód v{} (am suas {}s)", health.version, health.uptime_s);

    // Seiceáil iarmhéid
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Iarmhéid: {} RTC", balance);

    // Seiceáil iolraitheoirí aoise crua-earraí
    println!("Bónas G4: {}x", CpuArch::G4.multiplier());
    println!("Bónas G5: {}x", CpuArch::G5.multiplier());
}
```

## Iolraitheoirí aoise crua-earraí

| Ailtireacht | Iolraitheoir | Samplaí |
|-------------|--------------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium luath |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Nua-aimseartha | 1.0x | x86_64 reatha, aarch64 |

## Ceadúnas

MIT - [Elyan Labs](https://rustchain.org)
