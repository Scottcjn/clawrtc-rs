[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Cliente Rust para mineração de RTC na [RustChain](https://rustchain.org), com
atestação de hardware, carteiras Ed25519 e consenso Proof-of-Antiquity (PoA).

## Recursos

- **Carteiras Ed25519** - gere, assine e verifique usando endereços RTC
- **Cliente do nó** - verificações de integridade, consultas de saldo e listagem de mineradores
- **Atestação de hardware** - envie provas PoA para ganhar RTC
- **Inscrição em épocas** - registre-se para a distribuição de recompensas
- **Detecção de arquitetura** - mapeamento de multiplicadores de CPU (G4=2.5x, G5=2.0x etc.)

## Início rápido

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Gera uma nova carteira
    let wallet = Wallet::generate();
    println!("Endereço: {}", wallet.address());
    println!("Chave pública: {}", wallet.public_key_hex());

    // Assina uma mensagem
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Assinatura válida: {}", valid);

    // Conecta-se à rede
    let node = NodeClient::new("https://rustchain.org");

    // Verifica a integridade
    let health = node.health().unwrap();
    println!("Nó v{} (tempo ativo: {}s)", health.version, health.uptime_s);

    // Consulta o saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Consulta os multiplicadores de antiguidade
    println!("Bônus G4: {}x", CpuArch::G4.multiplier());
    println!("Bônus G5: {}x", CpuArch::G5.multiplier());
}
```

## Multiplicadores de antiguidade

| Arquitetura | Multiplicador | Exemplos |
|-------------|---------------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| x86 retro | 1.4x | 486, 386, primeiros Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderna | 1.0x | x86_64 e aarch64 atuais |

## Licença

MIT - [Elyan Labs](https://rustchain.org)
