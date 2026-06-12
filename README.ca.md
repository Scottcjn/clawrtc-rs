[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Client Rust per minar RTC a [RustChain](https://rustchain.org): attestació de maquinari, moneders Ed25519 i consens Proof-of-Antiquity (PoA).

## Funcions

- **Moneders Ed25519**: generació, signatura i verificació amb adreces RTC
- **Client de node**: comprovacions de salut, consultes de saldo i llistes de miners
- **Atestació de maquinari**: enviament de proves PoA per guanyar RTC
- **Registre d'època**: inscripció per a la distribució de recompenses
- **Detecció d'arquitectura**: mapatge de multiplicadors de CPU (G4=2.5x, G5=2.0x, etc.)

## Inici ràpid

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Genera un moneder nou
    let wallet = Wallet::generate();
    println!("Adreça: {}", wallet.address());
    println!("Clau pública: {}", wallet.public_key_hex());

    // Signa un missatge
    let sig = wallet.sign(b"hola rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hola rustchain", &sig).unwrap();
    println!("Signatura vàlida: {}", valid);

    // Connecta't a la xarxa
    let node = NodeClient::new("https://rustchain.org");

    // Comprova la salut
    let health = node.health().unwrap();
    println!("Node v{} (temps actiu {}s)", health.version, health.uptime_s);

    // Comprova el saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Comprova els multiplicadors d'edat del maquinari
    println!("Bonificació G4: {}x", CpuArch::G4.multiplier());
    println!("Bonificació G5: {}x", CpuArch::G5.multiplier());
}
```

## Multiplicadors d'edat del maquinari

| Arquitectura | Multiplicador | Exemples |
|--------------|---------------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium primerenc |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | x86_64 actual, aarch64 |

## Llicència

MIT - [Elyan Labs](https://rustchain.org)
