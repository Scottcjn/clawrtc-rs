[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org) RTCマイニング用のRustクライアント — ハードウェアアテステーション、Ed25519ウォレット、Proof-of-Antiquity（PoA）コンセンサス。

## 機能

- **Ed25519ウォレット** — RTCアドレスで生成、署名、検証
- **ノードクライアント** — ヘルスチェック、残高照会、マイナー一覧
- **ハードウェアアテステーション** — PoA証明を提出してRTCを獲得
- **エポック登録** — 報酬分配への登録
- **アーキテクチャ検出** — CPU倍率マッピング（G4=2.5倍、G5=2.0倍など）

## クイックスタート

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // 新しいウォレットを生成
    let wallet = Wallet::generate();
    println!("アドレス: {}", wallet.address());
    println!("公開鍵: {}", wallet.public_key_hex());

    // メッセージに署名
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("署名の有効性: {}", valid);

    // ネットワークに接続
    let node = NodeClient::new("https://rustchain.org");

    // ヘルスチェック
    let health = node.health().unwrap();
    println!("ノード v{}（アップタイム {}秒）", health.version, health.uptime_s);

    // 残高照会
    let balance = node.balance(&wallet.address()).unwrap();
    println!("残高: {} RTC", balance);

    // アンティーク（古硬体）倍率の確認
    println!("G4ボーナス: {}倍", CpuArch::G4.multiplier());
    println!("G5ボーナス: {}倍", CpuArch::G5.multiplier());
}
```

## アンティーク（古硬体）倍率

| アーキテクチャ | 倍率 | 例 |
|-------------|-----------|---------|
| PowerPC G4 | 2.5倍 | PowerBook G4、Power Mac G4 |
| PowerPC G5 | 2.0倍 | Power Mac G5、Xserve G5 |
| PowerPC G3 | 1.8倍 | iBook G3、Blue & White G3 |
| Pentium 4 | 1.5倍 | Dell Dimension、HP Pavilion |
| レトロ x86 | 1.4倍 | 486、386、初期Pentium |
| Core 2 Duo | 1.3倍 | MacBook 2006-2008 |
| Apple Silicon | 1.2倍 | M1、M2、M3 |
| モダン | 1.0倍 | 現行 x86_64、aarch64 |

## ライセンス

MIT — [Elyan Labs](https://rustchain.org)
