# microcred-rs - Claude Code ガイド

このドキュメントは、Claude CodeがこのRustプロジェクトを効率的に理解し、作業するための情報を提供します。

## プロジェクト概要

Rustで実装されたマイクロクレデンシャルシステムです。Ed25519暗号化を使用したデジタル署名により、安全な資格証明書の発行と検証を行います。

## アーキテクチャ

### コアモジュール構成

```
src/
├── lib.rs          # データモデル定義（Microcredential, Issuer, Subject, Skill, Evidence）
├── issuer.rs       # CredentialIssuer - 資格証明書の発行
├── verifier.rs     # CredentialVerifier - 資格証明書の検証
├── crypto.rs       # 暗号化処理（Ed25519署名）
└── main.rs         # デモプログラム
```

### 主要なデータ構造

#### Microcredential
資格証明書の中核となる構造体。以下を含む：
- `id`: 一意識別子（UUID）
- `issuer`: 発行者情報
- `subject`: 対象者情報
- `skill`: スキル情報（レベル付き）
- `evidence`: 証拠のリスト
- `issued_at`: 発行日時
- `expires_at`: 有効期限（オプション）
- `metadata`: カスタムメタデータ
- `signature`: Ed25519デジタル署名

#### SkillLevel
```rust
enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}
```

#### EvidenceType
```rust
enum EvidenceType {
    Project,
    Assessment,
    Portfolio,
    Certification,
    Other(String),
}
```

### 主要なコンポーネント

#### CredentialIssuer (src/issuer.rs)
- Ed25519鍵ペアを保持
- `issue_credential()`: 資格証明書に署名して発行
- `get_issuer_info()`: 公開鍵を含む発行者情報を取得

#### CredentialVerifier (src/verifier.rs)
- 信頼できる発行者のリストを管理
- `verify_credential()`: 署名の検証と有効期限チェック
- `add_trusted_issuer()`: 信頼できる発行者を追加

## 依存関係

主要なクレート：
- `serde` / `serde_json`: シリアライゼーション
- `ed25519-dalek`: Ed25519署名アルゴリズム
- `chrono`: 日時処理
- `uuid`: 一意識別子生成
- `sha2`: ハッシュ処理
- `rand`: 乱数生成

## 開発ガイド

### ビルドとテスト

```bash
# ビルド
cargo build

# テスト実行
cargo test

# デモプログラム実行
cargo run
```

### テストファイルの場所

- `src/lib.rs`: 統合テスト（資格証明書の作成、検証、有効期限）

### コーディング規約

1. **エラーハンドリング**: `Result<T, String>`を使用
2. **日時**: UTC時刻を使用（`chrono::Utc`）
3. **シリアライゼーション**: すべての主要な構造体に`Serialize`/`Deserialize`を実装
4. **暗号化**: Ed25519のみを使用（他のアルゴリズムへの変更は大規模な改修が必要）

### 新機能追加時の注意点

1. **署名の互換性**: `Microcredential`の構造を変更する場合、署名生成・検証ロジック（`crypto.rs`）も更新が必要
2. **有効期限チェック**: 検証時は必ず有効期限をチェックする
3. **信頼できる発行者**: 検証前に必ず信頼できる発行者をVerifierに追加する

### よくある操作

#### 新しいスキルレベルを追加
`src/lib.rs`の`SkillLevel` enumに追加

#### 新しい証拠タイプを追加
`src/lib.rs`の`EvidenceType` enumに追加

#### メタデータフィールドを追加
`Microcredential::add_metadata()`を使用、またはHashMapに直接追加

## セキュリティ考慮事項

- Ed25519秘密鍵は`CredentialIssuer`のみが保持
- 公開鍵は`Issuer`構造体に含まれ、検証に使用
- 署名検証に失敗した場合や有効期限切れの場合は、資格証明書を拒否

## 今後の拡張性

現在の設計で拡張可能な領域：
- 複数の署名アルゴリズムのサポート
- 失効リスト（CRL）の実装
- ブロックチェーンとの統合
- REST APIの追加
- データベース永続化層

## トラブルシューティング

### よくあるエラー

1. **署名検証失敗**: 信頼できる発行者が登録されているか確認
2. **有効期限エラー**: `expires_at`フィールドをチェック
3. **シリアライゼーションエラー**: すべての必須フィールドが設定されているか確認
