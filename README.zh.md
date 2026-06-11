[![BCOS 认证](https://img.shields.io/badge/BCOS-认证-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) RTC 挖矿的 Rust 客户端 —— 包含硬件认证、Ed25519 钱包和古物证明（PoA）共识。

## 功能

- **Ed25519 钱包** —— 生成、签名和验证 RTC 地址
- **节点客户端** —— 健康检查、余额查询、矿工列表
- **硬件认证** —— 提交 PoA 证明以赚取 RTC
- **纪元注册** —— 注册参与奖励分配
- **架构检测** —— CPU 倍数映射（G4=2.5x, G5=2.0x 等）

## 快速开始

`ust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // 生成新钱包
    let wallet = Wallet::generate();
    println!("地址: {}", wallet.address());
    println!("公钥: {}", wallet.public_key_hex());

    // 签名消息
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("签名有效: {}", valid);

    // 连接网络
    let node = NodeClient::new("https://rustchain.org");

    // 检查节点健康
    let health = node.health().unwrap();
    println!("节点 v{} (运行 {} 秒)", health.version, health.uptime_s);

    // 查询余额
    let balance = node.balance(&wallet.address()).unwrap();
    println!("余额: {} RTC", balance);

    // 查看古物倍数
    println!("G4 奖励: {}x", CpuArch::G4.multiplier());
    println!("G5 奖励: {}x", CpuArch::G5.multiplier());
}
`

## 古物倍数

| 架构 | 倍数 | 示例 |
|------|------|------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| 复古 x86 | 1.4x | 486, 386, 早期 Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| 现代 | 1.0x | 当前 x86_64, aarch64 |

## 许可证

MIT — [Elyan Labs](https://rustchain.org)