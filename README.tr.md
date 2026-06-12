[![BCOS Sertifikalı](https://img.shields.io/badge/BCOS-Sertifikal%C4%B1-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) RTC madenciliği için Rust istemcisi: donanım doğrulaması, Ed25519 cüzdanları ve Proof-of-Antiquity (PoA) mutabakatı.

## Özellikler

- **Ed25519 cüzdanları**: RTC adresleriyle oluşturma, imzalama ve doğrulama
- **Düğüm istemcisi**: sağlık kontrolleri, bakiye sorguları, madenci listeleri
- **Donanım doğrulaması**: RTC kazanmak için PoA kanıtları gönderme
- **Epok kaydı**: ödül dağıtımına kayıt olma
- **Mimari algılama**: CPU çarpanı eşlemesi (G4=2.5x, G5=2.0x vb.)

## Hızlı başlangıç

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Yeni bir cüzdan oluştur
    let wallet = Wallet::generate();
    println!("Adres: {}", wallet.address());
    println!("Açık anahtar: {}", wallet.public_key_hex());

    // Bir mesaj imzala
    let sig = wallet.sign(b"merhaba rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"merhaba rustchain", &sig).unwrap();
    println!("İmza geçerli: {}", valid);

    // Ağa bağlan
    let node = NodeClient::new("https://rustchain.org");

    // Sağlığı kontrol et
    let health = node.health().unwrap();
    println!("Düğüm v{} (çalışma süresi {}s)", health.version, health.uptime_s);

    // Bakiyeyi kontrol et
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Bakiye: {} RTC", balance);

    // Antiquity çarpanlarını kontrol et
    println!("G4 bonusu: {}x", CpuArch::G4.multiplier());
    println!("G5 bonusu: {}x", CpuArch::G5.multiplier());
}
```

## Antiquity çarpanları

| Mimari | Çarpan | Örnekler |
|--------|--------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, erken Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Modern | 1.0x | Güncel x86_64, aarch64 |

## Lisans

MIT - [Elyan Labs](https://rustchain.org)
