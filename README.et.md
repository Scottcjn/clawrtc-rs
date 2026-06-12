[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust klient RTC kaevandamiseks [RustChainis](https://rustchain.org): riistvara atesteerimine, Ed25519 rahakotid ja Proof-of-Antiquity (PoA) konsensus.

## Funktsioonid

- **Ed25519 rahakotid**: loomine, allkirjastamine ja kontrollimine RTC-aadressidega
- **Sõlmeklient**: tervisekontrollid, saldopäringud ja kaevurite loendid
- **Riistvara atesteerimine**: PoA-tõendite esitamine RTC teenimiseks
- **Epohhi registreerimine**: registreerumine preemiate jaotamiseks
- **Arhitektuuri tuvastamine**: CPU kordajate kaardistamine (G4=2.5x, G5=2.0x jne)

## Kiirstart

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Loo uus rahakott
    let wallet = Wallet::generate();
    println!("Aadress: {}", wallet.address());
    println!("Avalik võti: {}", wallet.public_key_hex());

    // Allkirjasta sõnum
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Allkiri kehtib: {}", valid);

    // Ühendu võrguga
    let node = NodeClient::new("https://rustchain.org");

    // Kontrolli seisundit
    let health = node.health().unwrap();
    println!("Sõlm v{} (tööaeg {}s)", health.version, health.uptime_s);

    // Kontrolli saldot
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Kontrolli riistvara vanuse kordajaid
    println!("G4 boonus: {}x", CpuArch::G4.multiplier());
    println!("G5 boonus: {}x", CpuArch::G5.multiplier());
}
```

## Riistvara vanuse kordajad

| Arhitektuur | Kordaja | Näited |
|-------------|---------|--------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, varane Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moodne | 1.0x | Praegune x86_64, aarch64 |

## Litsents

MIT - [Elyan Labs](https://rustchain.org)
