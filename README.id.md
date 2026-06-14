[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Klien Rust untuk penambangan RTC [RustChain](https://rustchain.org) — atestasi perangkat keras, dompet Ed25519, dan konsensus Proof-of-Antiquity (PoA).

## Fitur

- **Dompet Ed25519** — membuat, menandatangani, dan memverifikasi dengan alamat RTC
- **Klien Node** — pemeriksaan kesehatan, kueri saldo, daftar penambang
- **Atestasi Perangkat Keras** — mengirim proof PoA untuk memperoleh RTC
- **Pendaftaran Epoch** — mendaftar untuk distribusi reward
- **Deteksi Arsitektur** — pemetaan multiplier CPU (G4=2.5x, G5=2.0x, dan seterusnya)

## Mulai Cepat

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Buat dompet baru
    let wallet = Wallet::generate();
    println!("Alamat: {}", wallet.address());
    println!("Kunci Publik: {}", wallet.public_key_hex());

    // Tandatangani pesan
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Tanda tangan valid: {}", valid);

    // Hubungkan ke jaringan
    let node = NodeClient::new("https://rustchain.org");

    // Periksa kesehatan node
    let health = node.health().unwrap();
    println!("Node v{} (uptime {}s)", health.version, health.uptime_s);

    // Periksa saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Periksa multiplier antiquity
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Multiplier Antiquity

| Arsitektur | Multiplier | Contoh |
|-------------|-----------|---------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium awal |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | x86_64 saat ini, aarch64 |

## Lisensi

MIT — [Elyan Labs](https://rustchain.org)
