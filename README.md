# Javelin - 月次決算確報管理システム

Clean Architecture + Event Sourcing + CQRS による統合業務管理システム

---

## 概要

Javelinは、IFRS準拠の月次決算確報作成を支援するTUI（Terminal User Interface）アプリケーションです。
会計処理に加えて、売上管理・請求書発行機能を統合し、Event Sourcingによる完全な監査証跡と、CQRSによる高速な照会機能を提供します。

### 主な特徴

- **Clean Architecture**: 4層アーキテクチャによる保守性の高い設計
- **Event Sourcing**: イベントログを正本とした完全な履歴管理
- **CQRS**: Command/Query分離による最適化
- **TUI**: ratatuiによる快適なターミナル操作
- **IFRS準拠**: 国際財務報告基準に基づく会計処理
- **統合業務管理**: 会計・売上・請求を一元管理
- **Typst請求書**: 高品質なPDF請求書を自動生成

---

## クイックスタート

### 必要要件

- Rust 1.93 以降
- Cargo

### インストール

```bash
git clone https://github.com/your-org/javelin.git
cd javelin
cargo build --release
```

### 起動

```bash
# 通常モード
cargo run

# メンテナンスモード
cargo run -- --maintenance
```

### 請求書発行

Typstを使用した高品質なPDF請求書を生成できます。

```bash
# Typstのインストール（初回のみ）
# Windows: winget install --id Typst.Typst
# macOS: brew install typst
# Linux: cargo install --git https://github.com/typst/typst

# 請求書PDF生成
# TUIから「請求書印刷」メニューを選択
# 生成されたPDFは実行ディレクトリに保存されます
```

**請求書の特徴:**
- 適格請求書（インボイス制度）対応
- 登録番号・税率・税額を明記
- 銀行振込情報を自動挿入
- プロフェッショナルなレイアウト

---

## ドキュメント

### 業務要件

- **[BUSINESS_REQUIREMENTS.md](BUSINESS_REQUIREMENTS.md)** - 月次決算確報作成規程、業務フロー、統制要件

### 技術仕様

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - システムアーキテクチャ、設計原則、層別仕様

---

## プロジェクト構成

```
javelin/
├── crates/
│   ├── javelin/              # メインアプリケーション
│   ├── javelin-adapter/      # Adapter層（UI、Controller、Presenter）
│   ├── javelin-application/  # Application層（UseCase、Query、DTO）
│   ├── javelin-domain/       # Domain層（Entity、ValueObject、DomainService）
│   └── javelin-infrastructure/ # Infrastructure層（EventStore、Projection）
├── ARCHITECTURE.md           # アーキテクチャドキュメント
├── BUSINESS_REQUIREMENTS.md  # 業務要件定義
├── Cargo.toml
└── README.md
```

---

## 開発

### ビルド

```bash
cargo build
```

### テスト

```bash
cargo test
```

### 監視モード（bacon使用）

```bash
bacon
```

### フォーマット

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

---

## 主要機能

### 会計モジュール

- ✅ 仕訳入力・照会
- ✅ 勘定科目マスタ管理
- ✅ 補助科目マスタ管理
- ✅ 元帳照会
- ✅ 試算表生成
- 🚧 固定資産管理
- 🚧 リース会計処理
- 🚧 財務諸表生成
- 🚧 管理会計レポート

### 売上・請求モジュール

- ✅ 請求書データ管理
- ✅ Typst請求書PDF生成
- ✅ 適格請求書（インボイス）対応
- ✅ 銀行振込情報管理
- 🚧 売上計上処理
- 🚧 入金消込処理
- 🚧 請求書一覧・検索

### システム基盤

- ✅ ナビゲーションシステム
- ✅ Event Sourcing基盤
- ✅ Projection再構築
- 📋 判断ログ管理
- 📋 監査証跡機能
- 📋 期間管理

---

## 技術スタック

| カテゴリ | 技術 |
|----------|------|
| 言語 | Rust 1.93+ (Edition 2024) |
| UI | ratatui |
| 永続化 | LMDB (Event Store + Projections) |
| アーキテクチャ | Clean Architecture + Event Sourcing + CQRS |
| エラーハンドリング | thiserror + anyhow |
| 請求書生成 | Typst (PDF組版エンジン) |
| テスト | cargo test |

---

*本プロジェクトはIFRS準拠・個人事業主用月次決算確報作成規程に基づく。*
