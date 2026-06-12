[![BCOS Certificado](https://img.shields.io/badge/BCOS-Certificado-brightgreen?style=flat)](BCOS.md)

# clawrtc

Cliente en Rust para [RustChain](https://rustchain.org) minería de RTC — atestación de hardware, monederos Ed25519 y consenso Proof-of-Antiquity (PoA).

## Características

- **Monederos Ed25519** — genera, firma y verifica con direcciones RTC
- **Cliente de Nodo** — comprobaciones de salud, consultas de saldo, listados de mineros
- **Atestación de Hardware** — envía pruebas PoA para ganar RTC
- **Registro de Época** — regístrate para la distribución de recompensas
- **Detección de Arquitectura** — multiplicador de CPU por mapeo (G4=2.5x, G5=2.0x, etc.)

## Inicio Rápido

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Generar un monedero nuevo
    let wallet = Wallet::generate();
    println!("Dirección: {}", wallet.address());
    println!("Clave Pública: {}", wallet.public_key_hex());

    // Firmar un mensaje
    let sig = wallet.sign(b"hola rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hola rustchain", &sig).unwrap();
    println!("Firma válida: {}", valid);

    // Conectarse a la red
    let node = NodeClient::new("https://rustchain.org");

    // Comprobar salud
    let health = node.health().unwrap();
    println!("Nodo v{} (uptime {}s)", health.version, health.uptime_s);

    // Comprobar saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Comprobar multiplicadores de antigüedad
    println!("Bono G4: {}x", CpuArch::G4.multiplier());
    println!("Bono G5: {}x", CpuArch::G5.multiplier());
}
```

## Multiplicadores de Antigüedad

| Arquitectura | Multiplicador | Ejemplos |
|--------------|---------------|----------|
| PowerPC G4   | 2.5x          | PowerBook G4, Power Mac G4 |
| PowerPC G5   | 2.0x          | Power Mac G5, Xserve G5 |
| PowerPC G3   | 1.8x          | iBook G3, Blue & White G3 |
| Pentium 4    | 1.5x          | Dell Dimension, HP Pavilion |
| Retro x86    | 1.4x          | 486, 386, Pentium temprano |
| Core 2 Duo   | 1.3x          | MacBook 2006-2008 |
| Apple Silicon | 1.2x         | M1, M2, M3 |
| Moderna      | 1.0x          | x86_64 actual, aarch64 |

## Licencia

MIT — [Elyan Labs](https://rustchain.org)
