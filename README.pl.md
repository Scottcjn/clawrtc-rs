[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Klient Rust do kopania RTC w [RustChain](https://rustchain.org) - atestacja sprzętu, portfele Ed25519 oraz konsensus Proof-of-Antiquity (PoA).

## Funkcje

- **Portfele Ed25519** - generowanie, podpisywanie i weryfikacja z adresami RTC
- **Klient węzła** - kontrole stanu, zapytania o saldo i listy minerów
- **Atestacja sprzętu** - wysyłanie dowodów PoA w celu zdobywania RTC
- **Rejestracja w epoce** - zapis do dystrybucji nagród
- **Wykrywanie architektury** - mapowanie mnożników CPU (G4=2.5x, G5=2.0x itd.)

## Szybki start

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Wygeneruj nowy portfel
    let wallet = Wallet::generate();
    println!("Adres: {}", wallet.address());
    println!("Klucz publiczny: {}", wallet.public_key_hex());

    // Podpisz wiadomość
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Podpis poprawny: {}", valid);

    // Połącz się z siecią
    let node = NodeClient::new("https://rustchain.org");

    // Sprawdź stan węzła
    let health = node.health().unwrap();
    println!("Węzeł v{} (czas działania {}s)", health.version, health.uptime_s);

    // Sprawdź saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Sprawdź mnożniki antyczności
    println!("Premia G4: {}x", CpuArch::G4.multiplier());
    println!("Premia G5: {}x", CpuArch::G5.multiplier());
}
```

## Mnożniki antyczności

| Architektura | Mnożnik | Przykłady |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, wczesny Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Nowoczesne | 1.0x | Obecne x86_64, aarch64 |

## Licencja

MIT - [Elyan Labs](https://rustchain.org)
