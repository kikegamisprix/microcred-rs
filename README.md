# microcred-rs

Rustで実装されたマイクロクレデンシャルシステム

## 機能

- デジタル署名による安全な資格証明書の発行・検証
- スキル、証拠、メタデータを含む包括的なデータモデル
- Ed25519暗号化による偽造防止

## 使用方法

```bash
# デモの実行
cargo run

# テストの実行
cargo test
```

## 基本的な使い方

```rust
use microcred_rs::{issuer::CredentialIssuer, verifier::CredentialVerifier};

// 発行者を作成
let issuer = CredentialIssuer::new(
    "大学名".to_string(),
    "https://university.edu".to_string(),
);

// 資格証明書を発行
let credential = issuer.issue_credential(subject, skill, evidence, None)?;

// 検証者を作成して検証
let mut verifier = CredentialVerifier::new();
verifier.add_trusted_issuer(issuer.get_issuer_info().clone());
let is_valid = verifier.verify_credential(&credential)?;
```