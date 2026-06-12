[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

ไคลเอนต์ Rust สำหรับขุด RTC บน [RustChain](https://rustchain.org): การรับรองฮาร์ดแวร์ กระเป๋า Ed25519 และฉันทามติ Proof-of-Antiquity (PoA)

## คุณสมบัติ

- **กระเป๋า Ed25519**: สร้าง ลงนาม และตรวจสอบด้วยที่อยู่ RTC
- **ไคลเอนต์โหนด**: ตรวจสุขภาพ สอบถามยอดคงเหลือ และรายการนักขุด
- **การรับรองฮาร์ดแวร์**: ส่งหลักฐาน PoA เพื่อรับ RTC
- **ลงทะเบียนอีพ็อก**: ลงทะเบียนเพื่อรับการแจกจ่ายรางวัล
- **ตรวจจับสถาปัตยกรรม**: แมปตัวคูณ CPU (G4=2.5x, G5=2.0x เป็นต้น)

## เริ่มต้นอย่างรวดเร็ว

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // สร้างกระเป๋าใหม่
    let wallet = Wallet::generate();
    println!("ที่อยู่: {}", wallet.address());
    println!("กุญแจสาธารณะ: {}", wallet.public_key_hex());

    // ลงนามข้อความ
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("ลายเซ็นถูกต้อง: {}", valid);

    // เชื่อมต่อเครือข่าย
    let node = NodeClient::new("https://rustchain.org");

    // ตรวจสุขภาพ
    let health = node.health().unwrap();
    println!("โหนด v{} (เวลาทำงาน {}s)", health.version, health.uptime_s);

    // ตรวจยอดคงเหลือ
    let balance = node.balance(&wallet.address()).unwrap();
    println!("ยอดคงเหลือ: {} RTC", balance);

    // ตรวจตัวคูณอายุฮาร์ดแวร์
    println!("โบนัส G4: {}x", CpuArch::G4.multiplier());
    println!("โบนัส G5: {}x", CpuArch::G5.multiplier());
}
```

## ตัวคูณอายุฮาร์ดแวร์

| สถาปัตยกรรม | ตัวคูณ | ตัวอย่าง |
|--------------|--------|----------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium ยุคแรก |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| สมัยใหม่ | 1.0x | x86_64 ปัจจุบัน, aarch64 |

## ใบอนุญาต

MIT - [Elyan Labs](https://rustchain.org)
