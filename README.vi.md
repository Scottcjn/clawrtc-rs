[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

Ứng dụng khách Rust để đào RTC trên [RustChain](https://rustchain.org): chứng thực phần cứng, ví Ed25519 và đồng thuận Proof-of-Antiquity (PoA).

## Tính năng

- **Ví Ed25519**: tạo, ký và xác minh bằng địa chỉ RTC
- **Ứng dụng khách node**: kiểm tra trạng thái, truy vấn số dư và danh sách thợ đào
- **Chứng thực phần cứng**: gửi bằng chứng PoA để kiếm RTC
- **Đăng ký epoch**: đăng ký nhận phân phối phần thưởng
- **Phát hiện kiến trúc**: ánh xạ hệ số CPU (G4=2.5x, G5=2.0x, v.v.)

## Bắt đầu nhanh

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // Tạo ví mới
    let wallet = Wallet::generate();
    println!("Địa chỉ: {}", wallet.address());
    println!("Khóa công khai: {}", wallet.public_key_hex());

    // Ký một thông điệp
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("Chữ ký hợp lệ: {}", valid);

    // Kết nối mạng
    let node = NodeClient::new("https://rustchain.org");

    // Kiểm tra trạng thái
    let health = node.health().unwrap();
    println!("Node v{} (thời gian chạy {}s)", health.version, health.uptime_s);

    // Kiểm tra số dư
    let balance = node.balance(&wallet.address()).unwrap();
    println!("Số dư: {} RTC", balance);

    // Kiểm tra hệ số tuổi phần cứng
    println!("Thưởng G4: {}x", CpuArch::G4.multiplier());
    println!("Thưởng G5: {}x", CpuArch::G5.multiplier());
}
```

## Hệ số tuổi phần cứng

| Kiến trúc | Hệ số | Ví dụ |
|-----------|------|-------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, Pentium đời đầu |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| Hiện đại | 1.0x | x86_64 hiện tại, aarch64 |

## Giấy phép

MIT - [Elyan Labs](https://rustchain.org)
