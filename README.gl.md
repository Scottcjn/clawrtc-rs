[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Cliente Rust para minar RTC en [RustChain](https://rustchain.org): atestación de hardware, carteiras Ed25519 e consenso Proof-of-Antiquity (PoA).

## Funcionalidades

- **Carteiras Ed25519**: xeración, sinatura e verificación con enderezos RTC
- **Cliente de nodo**: comprobacións de saúde, consultas de saldo e listas de mineiros
- **Atestación de hardware**: envío de probas PoA para gañar RTC
- **Rexistro de época**: rexistro para a distribución de recompensas
- **Detección de arquitectura**: mapeo de multiplicadores de CPU (G4=2.5x, G5=2.0x etc.)

## Inicio rápido

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Xera unha carteira nova
    let wallet = Wallet::generate();
    println!("Enderezo: {}", wallet.address());
    println!("Chave pública: {}", wallet.public_key_hex());

    // Asina unha mensaxe
    let sig = wallet.sign(b"ola rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"ola rustchain", &sig).unwrap();
    println!("Sinatura válida: {}", valid);

    // Conecta coa rede
    let node = NodeClient::new("https://rustchain.org");

    // Comproba a saúde
    let health = node.health().unwrap();
    println!("Nodo v{} (tempo activo {}s)", health.version, health.uptime_s);

    // Comproba o saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Comproba os multiplicadores de idade do hardware
    println!("Bonificación G4: {}x", CpuArch::G4.multiplier());
    println!("Bonificación G5: {}x", CpuArch::G5.multiplier());
}
```

## Multiplicadores de idade do hardware

| Arquitectura | Multiplicador | Exemplos |
|--------------|---------------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium temperán |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderna | 1.0x | x86_64 actual, aarch64 |

## Licenza

MIT - [Elyan Labs](https://rustchain.org)
