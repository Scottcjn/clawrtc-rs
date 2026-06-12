[![BCOS Certifikováno](https://img.shields.io/badge/BCOS-Certifikov%C3%A1no-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rustový klient pro těžbu RTC na [RustChainu](https://rustchain.org) - hardwarová atestace, peněženky Ed25519 a konsenzus Proof-of-Antiquity (PoA).

## Funkce

- **Peněženky Ed25519** - generování, podepisování a ověřování s RTC adresami
- **Klient uzlu** - kontroly stavu, dotazy na zůstatek, seznamy minerů
- **Hardwarová atestace** - odesílání důkazů PoA pro zisk RTC
- **Registrace do epochy** - přihlášení k rozdělení odměn
- **Detekce architektury** - mapování násobků CPU (G4=2.5x, G5=2.0x atd.)

## Rychlý start

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Vygenerovat novou peněženku
    let wallet = Wallet::generate();
    println!("Adresa: {}", wallet.address());
    println!("Veřejný klíč: {}", wallet.public_key_hex());

    // Podepsat zprávu
    let sig = wallet.sign(b"ahoj rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"ahoj rustchain", &sig).unwrap();
    println!("Podpis je platný: {}", valid);

    // Připojit se k síti
    let node = NodeClient::new("https://rustchain.org");

    // Zkontrolovat stav uzlu
    let health = node.health().unwrap();
    println!("Uzel v{} (doba běhu {}s)", health.version, health.uptime_s);

    // Zkontrolovat zůstatek
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Zůstatek: {} RTC", balance);

    // Zkontrolovat násobky podle stáří hardwaru
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Násobky podle stáří hardwaru

| Architektura | Násobek | Příklady |
|-------------|---------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, rané Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderní | 1.0x | Současné x86_64, aarch64 |

## Licence

MIT - [Elyan Labs](https://rustchain.org)
