[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Klien Rust untuk menambang RTC di [RustChain](https://rustchain.org): atestasi perangkat keras, dompet Ed25519, dan konsensus Proof-of-Antiquity (PoA).

## Fitur

- **Dompet Ed25519**: membuat, menandatangani, dan memverifikasi dengan alamat RTC
- **Klien node**: pemeriksaan kesehatan, kueri saldo, dan daftar penambang
- **Atestasi perangkat keras**: mengirim bukti PoA untuk memperoleh RTC
- **Pendaftaran epoch**: mendaftar untuk distribusi hadiah
- **Deteksi arsitektur**: pemetaan pengali CPU (G4=2.5x, G5=2.0x, dan seterusnya)

## Mulai cepat

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Buat dompet baru
    let wallet = Wallet::generate();
    println!("Alamat: {}", wallet.address());
    println!("Kunci publik: {}", wallet.public_key_hex());

    // Tanda tangani pesan
    let sig = wallet.sign(b"halo rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"halo rustchain", &sig).unwrap();
    println!("Tanda tangan valid: {}", valid);

    // Terhubung ke jaringan
    let node = NodeClient::new("https://rustchain.org");

    // Periksa kesehatan
    let health = node.health().unwrap();
    println!("Node v{} (waktu aktif {}s)", health.version, health.uptime_s);

    // Periksa saldo
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Saldo: {} RTC", balance);

    // Periksa pengali usia perangkat keras
    println!("Bonus G4: {}x", CpuArch::G4.multiplier());
    println!("Bonus G5: {}x", CpuArch::G5.multiplier());
}
```

## Pengali usia perangkat keras

| Arsitektur | Pengali | Contoh |
|------------|---------|--------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium awal |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | x86_64 saat ini, aarch64 |

## Lisensi

MIT - [Elyan Labs](https://rustchain.org)
