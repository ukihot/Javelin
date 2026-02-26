# Javelin - 月次決算確報作成システム

Clean Architecture + Event Sourcing + CQRS による主計部業務バッチシステム

## プロジェクト構造

Cargo Workspaceを使用した多クレート構成:

```
javelin/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── javelin/                  # Main application (entry point)
│   │   ├── src/
│   │   │   ├── main.rs          # Application entry point
│   │   │   ├── app_builder.rs   # DI container
│   │   │   └── app_error.rs     # Top-level error
│   │   ├── tests/               # Integration tests
│   │   ├── .config/
│   │   │   └── nextest.toml     # Test configuration
│   │   └── bacon.toml           # Development workflow
│   │
│   ├── javelin-domain/          # Domain layer (no dependencies)
│   │   └── src/
│   │       ├── entity.rs
│   │       ├── value_object.rs
│   │       ├── event.rs
│   │       ├── repository_trait.rs
│   │       ├── service.rs
│   │       ├── error.rs
│   │       └── financial_close/
│   │
│   ├── javelin-application/     # Application layer (→ Domain)
│   │   └── src/
│   │       ├── input_port.rs
│   │       ├── interactor.rs
│   │       ├── query_service.rs
│   │       ├── projection_builder.rs
│   │       ├── output_port.rs
│   │       ├── dto.rs
│   │       └── error.rs
│   │
│   ├── javelin-infrastructure/  # Infrastructure layer (→ Domain)
│   │   └── src/
│   │       ├── event_store.rs
│   │       ├── projection_db.rs
│   │       ├── repository_impl.rs
│   │       └── error.rs
│   │
│   └── javelin-adapter/         # Adapter layer (→ Application)
│       └── src/
│           ├── controller.rs
│           ├── presenter.rs
│           ├── view.rs
│           ├── view_router.rs
│           └── error.rs
│
├── ARCHITECTURE.md              # Architecture documentation
└── financialCloseFinalReport.md # Business requirements
```

## 依存関係

```
javelin (main)
├── javelin-adapter
│   ├── javelin-application
│   │   └── javelin-domain
│   └── javelin-domain
├── javelin-infrastructure
│   └── javelin-domain
├── javelin-application
│   └── javelin-domain
└── javelin-domain (no dependencies)
```

## 開発環境

### 必要なツール

```bash
# Rust toolchain
rustup update

# nextest (高速テストランナー)
cargo install cargo-nextest

# bacon (バックグラウンドタスクランナー)
cargo install bacon
```

### ビルド

```bash
# Workspace全体をビルド
cargo build

# 特定のクレートをビルド
cargo build -p javelin-domain
cargo build -p javelin

# リリースビルド
cargo build --release
```

### テスト

各ファイルに`#[cfg(test)]`モジュールを配置する方式を採用。

```bash
# 全テスト実行
cargo nextest run

# 特定のクレートのみテスト
cargo nextest run -p javelin-domain
cargo nextest run -p javelin-application

# 層別テスト（パッケージ指定）
cargo nextest run -p javelin-domain
cargo nextest run -p javelin-infrastructure
```

**テストグループ:**
- Domain層: 高速、I/Oなし（並列度: 8）
- Application層: 中速、軽いI/O（並列度: 4）
- Infrastructure層: 低速、重いI/O（並列度: 2）

### 開発ワークフロー

```bash
# bacon起動（crates/javelin ディレクトリで）
cd crates/javelin
bacon

# または特定のジョブ
bacon test
bacon clippy
```

### 実行

```bash
# アプリケーション起動
cargo run -p javelin

# または
cd crates/javelin
cargo run
```

## アーキテクチャ

詳細は [ARCHITECTURE.md](ARCHITECTURE.md) を参照。

### 各層の責務

- **Domain層**: 業務ルール、Entity、ValueObject（外部依存なし）
- **Application層**: ユースケース、Query、Projection制御
- **Infrastructure層**: 永続化、EventStore、ProjectionDB
- **Adapter層**: UI、Controller、Presenter

### エラーハンドリング

各層で独立したエラー型:

- `DomainError` (D-xxxx) - javelin-domain
- `ApplicationError` (A-xxxx) - javelin-application
- `InfrastructureError` (I-xxxx) - javelin-infrastructure
- `AdapterError` (V-xxxx) - javelin-adapter
- `AppError` (APP-xxxx) - javelin

## ライセンス

See [LICENSE](LICENSE)
