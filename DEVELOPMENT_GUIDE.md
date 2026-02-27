# 開発ガイド

Javelinの開発、実装、およびメンテナンスに関する実践的な情報を提供する。

---

## 第1章 プロジェクト構成

### 1.1 クレート構成

```
javelin/
├── crates/
│   ├── javelin/              # メインアプリケーション
│   ├── javelin-adapter/      # Adapter層
│   ├── javelin-application/  # Application層
│   ├── javelin-domain/       # Domain層
│   └── javelin-infrastructure/ # Infrastructure層
├── Cargo.toml
└── README.md
```

---

### 1.2 起動方法

```bash
# 通常モード
cargo run

# メンテナンスモード
cargo run -- --maintenance
```

---

### 1.3 開発ツール

```bash
# ビルド
cargo build

# テスト実行
cargo test

# 監視モード（bacon使用）
bacon

# フォーマット
cargo fmt

# Lint
cargo clippy
```

---

## 第2章 開発ワークフロー

### 2.1 新機能追加手順

1. Domain層: Entity/ValueObject定義
2. Domain層: DomainEvent定義
3. Application層: InputPort定義
4. Application層: Interactor実装
5. Application層: ProjectionBuilder実装
6. Adapter層: Controller実装
7. Adapter層: Presenter実装
8. Adapter層: View実装
9. Adapter層: PageState実装
10. テスト作成

---

### 2.2 画面追加手順

1. Route定義（navigation/route.rs）
2. PageState実装
3. View実装
4. PageStateResolver登録（app_resolver.rs）
5. ナビゲーション元からの遷移追加

---

## 第3章 Rust モダンプラクティス

### 3.1 Rust 1.93 / 2024 Edition 対応

本プロジェクトはRust 2024 Editionを前提とし、以下のモダンな書き方を推奨する。

#### (1) let-else による早期脱出

```rust
// 従来: ネストが深い
let user = match repo.find(id) {
    Some(u) => u,
    None => return Err(Error::NotFound),
};

// モダン: 意図が明確
let Some(user) = repo.find(id) else {
    return Err(Error::NotFound),
};
```

**利点:** ネストを減らし、失敗ケースを即座に処理。正常系が左寄せになる。

---

#### (2) if-let / while-let チェーン

```rust
// 従来: ネスト地獄
if let Some(x) = a {
    if let Ok(y) = parse(x) {
        if y > 10 {
            // ...
        }
    }
}

// モダン: 直列条件
if let Some(x) = a
    && let Ok(y) = parse(x)
    && y > 10
{
    // ...
}
```

---

#### (3) Option/Result の is_some_and / is_ok_and

```rust
// 従来: 冗長
if opt.map(|v| v > 5).unwrap_or(false) { ... }

// モダン: 簡潔
if opt.is_some_and(|v| v > 5) { ... }
if res.is_ok_and(|v| v > 5) { ... }
```

---

#### (4) std::path::Path の直接比較

```rust
// 従来: 不要なヒープ確保
if path == PathBuf::from("config.toml") { ... }

// モダン: Path::new で直接比較
if path == Path::new("config.toml") { ... }
```

---

#### (5) IntoIterator for arrays

```rust
// 従来: 参照が前提
for v in &[1, 2, 3] {
    println!("{v}");
}

// モダン: 配列が値として回る
for v in [1, 2, 3] {
    println!("{v}");
}
```

---

#### (6) std::sync::OnceLock / LazyLock

```rust
// 従来: once_cell クレート依存
use once_cell::sync::Lazy;
static CONFIG: Lazy<Config> = Lazy::new(load_config);

// モダン: 標準ライブラリ
use std::sync::OnceLock;
static CONFIG: OnceLock<Config> = OnceLock::new();

fn config() -> &'static Config {
    CONFIG.get_or_init(load_config)
}
```

---

#### (7) Error 実装に thiserror + anyhow

```rust
// モダン: thiserror で簡潔に
#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    
    #[error("parse error: {0}")]
    Parse(String),
}
```

**プロジェクト方針:** 
- Domain層: 独自エラー型（thiserror使用）
- Application層: ApplicationError（thiserror使用）
- Adapter層: anyhow::Result でラップ

---

#### (8) async fn in traits（安定化済）

```rust
// モダン: 標準サポート
trait Repository {
    async fn find(&self, id: Id) -> Result<Entity>;
}
```

**注意:** オブジェクト安全性には制約あり。

---

#### (9) let パターンによる構造分解強化

```rust
// モダン: let で直接分解
let Point { x, y, .. } = point;
// x, y がそのまま使える
```

---

### 3.2 補足的傾向（実務設計指針）

| 項目 | 従来 | モダン | 理由 |
|------------|------------|------------|------------|
| 並行制御 | Arc<Mutex<T>> | tokio::sync 系 / parking_lot | 用途に応じた選択 |
| メモリ確保 | Vec::new() | Vec::with_capacity(n) | リアロケーション削減 |
| エラー型 | Box<dyn Error> | 具体型 Result<T, E> | 型安全性とパフォーマンス |
| CI/CD | 手動チェック | clippy 強制 | cargo clippy -- -D warnings |
| 依存管理 | 多数の小クレート | 標準ライブラリ優先 | OnceLock, LazyLock 等の活用 |

---

### 3.3 プロジェクト固有のスタイルガイド

| 項目 | 方針 |
|------------|------------|
| Edition | Rust 2024 Edition を使用 |
| MSRV | Rust 1.93 以降 |
| 早期脱出 | let-else を積極活用 |
| エラーハンドリング | thiserror + 具体型 Result<T, E> |
| グローバル初期化 | OnceLock / LazyLock を使用 |
| 条件分岐 | if-let チェーンでネスト削減 |
| Option/Result | is_some_and / is_ok_and を優先 |
| unsafe | 最小限に抑え、必ず安全性コメントを付与 |
| lint | clippy::all, clippy::pedantic を有効化 |
| フォーマット | rustfmt のデフォルト設定を使用 |
| メモリ確保 | Vec::with_capacity を前提に設計 |
| 並行制御 | 用途に応じて tokio::sync / parking_lot を選択 |

---

### 3.4 アンチパターン防止

| アンチパターン | 問題 | 正しい実装 |
|------------|------------|------------|
| 文字列でEnum比較 | 型安全性の喪失 | FromStrトレイト使用 |
| DTO変換ユーティリティクラス | 責務の不明確化 | DTOにTryFrom実装 |
| ドメインロジックの再実装 | 一貫性喪失 | ドメインサービス呼び出し |
| 手動での借方貸方反転 | バグ混入リスク | JournalEntryService::create_reversal_lines使用 |
| format!("{:?}", status) | 表示形式の不安定性 | status.as_str()またはDisplay使用 |

---

## 第4章 トラブルシューティング

### 4.1 よくある問題

| 問題 | 原因 | 解決方法 |
|------------|------------|------------|
| コンパイルエラー | 依存関係違反 | ARCHITECTURE.mdの依存ルール確認 |
| テスト失敗 | イベント再生順序 | ProjectionBuilder実装確認 |
| 画面遷移不具合 | Route未登録 | PageStateResolver確認 |

---

## 第5章 今後の拡張

### 5.1 予定機能

- 固定資産管理の完全実装
- リース会計処理
- 財務諸表生成
- 管理会計レポート
- 判断ログ管理

---

### 5.2 技術的改善

- パフォーマンス最適化
- エラーハンドリング強化
- ログ機能拡充
- テストカバレッジ向上

---

## 第6章 設計の明示性とヒープ挙動の制御

現代Rustは「マクロで無理に回避する」より「標準が整備されたから素直に書く」方向に寄っている。

**中心思想:**
1. **設計の明示性**: let-else、if-let チェーンで意図を明確に
2. **ヒープ挙動の制御**: with_capacity、Path::new で不要な確保を回避
3. **標準ライブラリ優先**: OnceLock、async fn in traits で外部依存削減
4. **型安全性**: 具体型エラー、is_some_and で unwrap 回避

**プロジェクトへの適用:**
- Controller: let-else で入力検証
- Interactor: is_some_and / is_ok_and で条件チェック
- Repository: 将来的に async fn in traits を検討
- エラー処理: thiserror で統一
- グローバル設定: OnceLock で管理

---

*本ドキュメントはRust 1.93 / 2024 Editionを前提とする。*
