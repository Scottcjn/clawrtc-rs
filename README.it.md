[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Client Rust per il mining RTC su [RustChain](https://rustchain.org): attestazione hardware, wallet Ed25519 e consenso Proof-of-Antiquity (PoA).

## Funzionalità

- **Wallet Ed25519**: genera, firma e verifica con indirizzi RTC
- **Client del nodo**: controlli di salute, interrogazioni del saldo, elenco dei miner
- **Attestazione hardware**: invia prove PoA per guadagnare RTC
- **Iscrizione all'epoca**: registrazione per la distribuzione delle ricompense
- **Rilevamento dell'architettura**: mappatura dei moltiplicatori CPU (G4=2.5x, G5=2.0x, ecc.)

## Avvio rapido

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Genera un nuovo wallet
    let wallet = Wallet::generate();
    println!("Indirizzo: {}", wallet.address());
    println!("Chiave pubblica: {}", wallet.public_key_hex());

    // Firma un messaggio
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Firma valida: {}", valid);

    // Connettiti alla rete
    let node = NodeClient::new("https://rustchain.org");

    // Controlla lo stato del nodo
    let health = node.health().unwrap();
    println!("Nodo v{} (tempo di attività {}s)", health.version, health.uptime_s);

    // Controlla il saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Controlla i moltiplicatori di antichità
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Moltiplicatori di antichità

| Architettura | Moltiplicatore | Esempi |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, primi Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderno | 1.0x | x86_64 e aarch64 attuali |

## Licenza

MIT: [Elyan Labs](https://rustchain.org)
