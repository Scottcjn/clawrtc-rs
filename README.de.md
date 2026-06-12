[![BCOS Zertifiziert](https://img.shields.io/badge/BCOS-Zertifiziert-brightgreen?style=flat)](BCOS.md)

# clawrtc

Rust-Client für [RustChain](https://rustchain.org)-RTC-Mining - Hardware-Attestierung, Ed25519-Wallets und Proof-of-Antiquity-(PoA)-Konsens.

## Funktionen

- **Ed25519-Wallets** - Erzeugen, Signieren und Verifizieren mit RTC-Adressen
- **Node-Client** - Statusprüfungen, Saldoabfragen und Miner-Listen
- **Hardware-Attestierung** - PoA-Nachweise einreichen, um RTC zu verdienen
- **Epochen-Registrierung** - Registrierung für die Verteilung von Belohnungen
- **Architekturerkennung** - CPU-Multiplikator-Zuordnung (G4=2.5x, G5=2.0x usw.)

## Schnellstart

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Neues Wallet erzeugen
    let wallet = Wallet::generate();
    println!("Adresse: {}", wallet.address());
    println!("Öffentlicher Schlüssel: {}", wallet.public_key_hex());

    // Nachricht signieren
    let sig = wallet.sign(b"hallo rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hallo rustchain", &sig).unwrap();
    println!("Signatur gültig: {}", valid);

    // Mit dem Netzwerk verbinden
    let node = NodeClient::new("https://rustchain.org");

    // Status prüfen
    let health = node.health().unwrap();
    println!("Node v{} (Laufzeit {}s)", health.version, health.uptime_s);

    // Saldo prüfen
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Antiquity-Multiplikatoren prüfen
    println!("G4-Bonus: {}x", CpuArch::G4.multiplier());
    println!("G5-Bonus: {}x", CpuArch::G5.multiplier());
}
```

## Antiquity-Multiplikatoren

| Architektur | Multiplikator | Beispiele |
|-------------|---------------|-----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, frühe Pentium-Systeme |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Aktuelle x86_64- und aarch64-Systeme |

## Lizenz

MIT - [Elyan Labs](https://rustchain.org)
