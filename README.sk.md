[![BCOS Certifikované](https://img.shields.io/badge/BCOS-Certifikovan%C3%A9-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rustový klient na ťažbu RTC v sieti [RustChain](https://rustchain.org) - hardvérová atestácia, peňaženky Ed25519 a konsenzus Proof-of-Antiquity (PoA).

## Funkcie

- **Peňaženky Ed25519** - generovanie, podpisovanie a overovanie s RTC adresami
- **Klient uzla** - kontroly stavu, dotazy na zostatok a zoznamy minerov
- **Hardvérová atestácia** - odosielanie dôkazov PoA na získanie RTC
- **Registrácia do epochy** - prihlásenie sa do rozdeľovania odmien
- **Detekcia architektúry** - mapovanie násobkov CPU (G4=2.5x, G5=2.0x atď.)

## Rýchly štart

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Vygenerovať novú peňaženku
    let wallet = Wallet::generate();
    println!("Adresa: {}", wallet.address());
    println!("Verejný kľúč: {}", wallet.public_key_hex());

    // Podpísať správu
    let sig = wallet.sign(b"ahoj rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"ahoj rustchain", &sig).unwrap();
    println!("Podpis je platný: {}", valid);

    // Pripojiť sa k sieti
    let node = NodeClient::new("https://rustchain.org");

    // Skontrolovať stav uzla
    let health = node.health().unwrap();
    println!("Uzol v{} (doba behu {}s)", health.version, health.uptime_s);

    // Skontrolovať zostatok
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Zostatok: {} RTC", balance);

    // Skontrolovať násobky podľa veku hardvéru
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Násobky podľa veku hardvéru

| Architektúra | Násobok | Príklady |
|--------------|---------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, skoré Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderné | 1.0x | Súčasné x86_64, aarch64 |

## Licencia

MIT - [Elyan Labs](https://rustchain.org)
