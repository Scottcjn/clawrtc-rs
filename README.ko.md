[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# clawrtc

[RustChain](https://rustchain.org)에서 RTC를 채굴하기 위한 Rust 클라이언트입니다. 하드웨어 증명, Ed25519 지갑, Proof-of-Antiquity(PoA) 합의를 제공합니다.

## 기능

- **Ed25519 지갑**: RTC 주소 생성, 서명, 검증
- **노드 클라이언트**: 상태 확인, 잔액 조회, 채굴자 목록 조회
- **하드웨어 증명**: RTC를 얻기 위한 PoA 증명 제출
- **에포크 등록**: 보상 분배 대상 등록
- **아키텍처 감지**: CPU 배율 매핑(G4=2.5x, G5=2.0x 등)

## 빠른 시작

```rust
use clawrtc::{NodeClient, Wallet, CpuArch};

fn main() {
    // 새 지갑 생성
    let wallet = Wallet::generate();
    println!("주소: {}", wallet.address());
    println!("공개 키: {}", wallet.public_key_hex());

    // 메시지 서명
    let sig = wallet.sign(b"hello rustchain");
    let valid = Wallet::verify(&wallet.public_key_hex(), b"hello rustchain", &sig).unwrap();
    println!("서명 유효: {}", valid);

    // 네트워크 연결
    let node = NodeClient::new("https://rustchain.org");

    // 상태 확인
    let health = node.health().unwrap();
    println!("노드 v{} (가동 시간 {}s)", health.version, health.uptime_s);

    // 잔액 확인
    let balance = node.balance(&wallet.address()).unwrap();
    println!("잔액: {} RTC", balance);

    // 장비 연식 배율 확인
    println!("G4 보너스: {}x", CpuArch::G4.multiplier());
    println!("G5 보너스: {}x", CpuArch::G5.multiplier());
}
```

## 장비 연식 배율

| 아키텍처 | 배율 | 예시 |
|----------|------|------|
| PowerPC G4 | 2.5x | PowerBook G4, Power Mac G4 |
| PowerPC G5 | 2.0x | Power Mac G5, Xserve G5 |
| PowerPC G3 | 1.8x | iBook G3, Blue & White G3 |
| Pentium 4 | 1.5x | Dell Dimension, HP Pavilion |
| Retro x86 | 1.4x | 486, 386, 초기 Pentium |
| Core 2 Duo | 1.3x | MacBook 2006-2008 |
| Apple Silicon | 1.2x | M1, M2, M3 |
| 현대식 | 1.0x | 현재 x86_64, aarch64 |

## 라이선스

MIT - [Elyan Labs](https://rustchain.org)
