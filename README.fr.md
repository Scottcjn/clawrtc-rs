[![Certifié BCOS](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Client Rust pour le minage RTC de [RustChain](https://rustchain.org) — attestation matérielle, portefeuilles Ed25519 et consensus Preuve d'Antiquité (PoA).

## Fonctionnalités

- **Portefeuilles Ed25519** — génération, signature et vérification avec adresses RTC
- **Client de nœud** — vérifications d'état, requêtes de solde, liste des mineurs
- **Attestation matérielle** — soumission de preuves PoA pour gagner des RTC
- **Inscription aux epochs** — enregistrement pour la distribution des récompenses
- **Détection d'architecture** — table de multiplicateurs CPU (G4=2.5x, G5=2.0x, etc.)

## Démarrage rapide

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Générer un nouveau portefeuille
    let wallet = Wallet::generate();
    println!("Adresse : {}", wallet.address());
    println!("Clé publique : {}", wallet.public_key_hex());

    // Signer un message
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Signature valide : {}", valid);

    // Connexion au réseau
    let node = NodeClient::new("https://rustchain.org");

    // Vérification de l'état du nœud
    let health = node.health().unwrap();
    println!("Nœud v{} (uptime {}s)", health.version, health.uptime_s);

    // Consultation du solde
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Solde : {} RTC", balance);

    // Consultation des multiplicateurs d'antiquité
    println!("Bonus G4 : {}x", CpuArch::G4.multiplier());
    println!("Bonus G5 : {}x", CpuArch::G5.multiplier());
}
```

## Multiplicateurs d'antiquité

| Architecture | Multiplicateur | Exemples |
|-------------|---------------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| x86 rétro | 1.4x | 486, 386, premiers Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Moderne | 1.0x | x86_64, aarch64 actuels |

## Licence

MIT — [Elyan Labs](https://rustchain.org)