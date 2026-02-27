# システムアーキテクチャ

Javelinの技術アーキテクチャ、設計原則、および実装方針を定義する。

---

# 第1章 アーキテクチャ総則

Javelinは以下の4層で構成される。

| 層 | 役割 | 依存方向 | クレート |
|------|------------|------------|------------|
| Adapter層 | 入出力変換・UI | → Application | javelin-adapter |
| Application層 | ユースケース / Query / Projection制御 | → Domain | javelin-application |
| Domain層 | 業務ルール / 集約整合性保証 | 依存なし | javelin-domain |
| Infrastructure層 | 永続化 / 外部技術実装 | → Domain | javelin-infrastructure |

依存関係は常に内側へ向かう。

---

## 1.1 設計原則

- CommandとQueryを完全分離する
- 永続データはイベントログを正本とする
- 読み取りモデルはイベントから再構築可能とする
- ドメイン層は読み取り最適化構造を認識しない
- 単一ユーザー・ローカル完結を前提とする

---

## 1.2 CQRS原則

| 区分 | 役割 | 実装層 |
|------------|------------|------------|
| Command | 状態変更 | Application + Domain |
| Query | 読み取り専用 | Application |

---

## 1.3 イベントソーシング原則

| 項目 | 内容 |
|------------|------------|
| 正本データ | Event Store |
| 更新方式 | 追記専用 |
| 状態復元 | イベント再生 |
| Projection | 派生データとして生成 |
| 再構築 | 常時可能 |

---

## 1.4 永続ストレージ構成

| 用途 | DB |
|------------|------------|
| 書き込み | Event Store LMDB |
| 読み取り | Projection LMDB（複数可） |

---

## 1.5 Projection運用方針

- 検索用途ごとに独立ReadModelを許容する
- Projectionはキャッシュではなく派生構造である
- 新Projectionは既存イベントから生成可能
- Projection数に制限を設けない

---

## 1.6 同期原則

| 項目 | 内容 |
|------------|------------|
| 同期契機 | Event永続化直後 |
| 実行方式 | 同一プロセス内Projection更新 |
| 非同期更新 | 原則不要 |

---

# 第2章 Adapter層

## 2.1 目的

外部入出力を内部DTOへ変換し、ユースケースへ接続する。
TUI（Terminal User Interface）による対話的操作を提供する。
業務ロジック・判断・仕様を保持しない。

---

## 2.2 構成要素

| 要素 | 役割 | 責務 | 禁止事項 |
|------------|------------|------------|------------|
| Controller | 外部入力受付 | DTO変換、InputPort呼び出し | 業務判断 |
| Presenter | 出力整形 | OutputPort実装、Viewモデル生成 | 業務判断 |
| View | 表示専用 | ratatuiによる描画 | 状態管理・ロジック保持 |
| PageState | 画面状態管理 | ナビゲーション制御、ライフサイクル管理 | 業務ロジック |
| NavigationStack | 画面遷移管理 | 画面履歴管理、Back遷移 | 画面内ロジック |

---

## 2.3 Controller仕様

| 項目 | 内容 |
|------------|------------|
| 入力元 | TUIイベント（キーボード入力） |
| 出力先 | InputPort |
| 処理内容 | 外部データ → DTO変換 |
| 状態保持 | 不可 |
| 実装例 | JournalEntryController, AccountMasterController |

---

## 2.4 Presenter仕様

| 項目 | 内容 |
|------------|------------|
| 入力元 | Interactor |
| 出力先 | View |
| 処理内容 | DTO → ViewModel変換 |
| 状態保持 | 不可 |
| 実装例 | JournalEntryPresenter, AccountMasterPresenter |

---

## 2.5 View仕様

| 項目 | 内容 |
|------------|------------|
| 表示技術 | ratatui |
| 更新契機 | Presenter通知 |
| 業務知識 | 完全禁止 |
| 実装例 | JournalEntryPage, AccountMasterPage, MenuPage |

---

## 2.6 Navigation System

### 2.6.1 PageState

各画面の状態とライフサイクルを管理する。

| メソッド | 役割 |
|------------|------------|
| route() | 画面識別子を返す |
| run() | イベントループ実行 |
| on_pause() | 画面から離れる際の処理 |
| on_resume() | 画面に戻る際の処理 |

### 2.6.2 NavigationStack

画面遷移履歴を管理し、Back遷移を実現する。

| 操作 | 説明 |
|------------|------------|
| push() | 新しい画面をスタックに追加 |
| pop() | 現在の画面を削除し、前の画面に戻る |
| current() | 現在の画面を取得 |

### 2.6.3 Route

画面識別子。各画面に一意のRouteが割り当てられる。

**主要Route:**
- Home: ダッシュボード
- PrimaryRecordsMenu: 原始記録登録メニュー
- JournalEntry: 仕訳入力
- LedgerMenu: 元帳管理メニュー
- ClosingMenu: 月次決算メニュー
- FinancialStatementsMenu: 財務諸表メニュー

---

# 第3章 Application層

## 3.1 目的

ユースケース実装およびCommand・Queryの調整を担う。
ドメインを組み合わせるが業務ルールを保持しない。

---

## 3.2 構成要素

| 要素 | 役割 | 責務 |
|------------|------------|------------|
| InputPort | Command境界 | ユースケース定義 |
| Interactor | Command実装 | ドメイン操作調整 |
| QueryService | Query処理 | Projection検索 |
| OutputPort | 出力抽象 | Presenter連携 |
| ProjectionBuilder | ReadModel生成 | Event → Projection変換 |
| DTO | データ転送 | ドメイン隠蔽 |

---

## 3.3 Command処理原則

- Interactorはイベント生成を行う
- RepositoryTraitはイベント永続のみを担当
- 状態更新はイベント再生により行う

---

## 3.4 Query処理原則

| 項目 | 内容 |
|------------|------------|
| 処理主体 | QueryService |
| 参照対象 | Projection |
| Repository利用 | 禁止 |
| 検索ロジック | Application層で管理 |

---

## 3.5 ProjectionBuilder仕様

| 項目 | 内容 |
|------------|------------|
| 入力 | Event Stream |
| 出力 | Projection DB |
| 再構築 | 全イベント再生で生成可能 |
| 実装例 | AccountMasterProjectionBuilder, JournalEntryProjectionBuilder |

---

## 3.6 Interactor仕様

| 項目 | 内容 |
|------------|------------|
| 実装対象 | InputPort |
| 利用対象 | Entity / ValueObject / DomainService / RepositoryTrait |
| 判断基準 | ドメイン委譲 |
| 表示更新 | OutputPort経由のみ |
| DTO変換 | TryFromトレイトを使用 |

---

## 3.7 DTO仕様

| 項目 | 内容 |
|------------|------------|
| 構成 | プリミティブ型 |
| 目的 | ドメイン隔離 |
| 変換 | TryFromトレイトで実装 |
| 責務 | アダプター層とアプリケーション層の境界 |

---

## 3.8 DTO変換原則

DTOはアプリケーション層の産物であり、以下の役割を持つ：

1. **レスポンスDTO**: アダプター層へ返すプリミティブデータ
   - アプリケーション層内部ではエンティティ/値オブジェクトを使用
   - 返却時のみDTOに詰め替え

2. **リクエストDTO**: アダプター層から受け取るプリミティブデータ
   - INPUT PORTへ渡せる唯一のプロトコル
   - TryFromトレイトでドメインオブジェクトへ変換

**変換実装:**
```rust
impl TryFrom<&JournalEntryLineDto> for JournalEntryLine {
    type Error = ApplicationError;
    fn try_from(dto: &JournalEntryLineDto) -> Result<Self, Self::Error> {
        // DTO → ドメインオブジェクト変換
    }
}
```

**重要原則:**
- ドメインDTOは存在しない
- DTOはアプリケーション層管轄
- 変換ロジックはDTOに実装（ユーティリティクラス不要）

---

# 第4章 Domain層

## 4.1 目的

業務ルールおよび整合性を保証する。

---

## 4.2 構成要素

| 要素 | 役割 | 特徴 |
|------------|------------|------------|
| Entity | 業務主体 | ID識別 |
| ValueObject | 値意味保持 | 不変 |
| RepositoryTrait | Event永続抽象 | Append操作 |
| DomainService | 横断処理 | 集約間ロジック |

---

## 4.3 Entity仕様

| 項目 | 内容 |
|------------|------------|
| 識別 | ID必須 |
| 変更 | プロパティ変更可 |
| 同一性 | ID基準 |
| フィールド型 | ValueObject中心に構成 |
| ロジック | 業務整合性維持 |

---

## 4.4 ValueObject仕様

| 項目 | 内容 |
|------------|------------|
| 識別 | 値一致 |
| 不変性 | 必須 |
| 検証 | コンストラクタ内実施 |
| 演算 | 業務意味単位で提供 |
| 標準トレイト | FromStr, Display実装推奨 |

---

## 4.4.1 ValueObject標準トレイト実装

Enumや構造体の値オブジェクトには、Rustの標準トレイトを実装することで、コードの一貫性と可読性を向上させる。

| トレイト | 用途 | 実装例 |
|------------|------------|------------|
| FromStr | 文字列からの変換 | `"Debit".parse::<DebitCredit>()` |
| Display | 文字列への変換 | `format!("{}", transaction_date)` |
| as_str() | 文字列表現取得 | `debit_credit.as_str()` |

**実装済み値オブジェクト:**
- DebitCredit (借方/貸方)
- Currency (通貨)
- TaxType (税区分)
- TransactionDate (取引日付)
- JournalStatus (仕訳ステータス)

---

## 4.5 RepositoryTrait仕様

| 項目 | 内容 |
|------------|------------|
| 役割 | Event Store抽象 |
| 必須操作 | append / loadStream |
| Query機能 | 禁止 |

---

## 4.6 DomainService仕様

| 項目 | 内容 |
|------------|------------|
| 対象 | 複数Entity横断処理 |
| 使用条件 | Entityへ属させると不自然な処理 |
| 制約 | Entity貧血防止 |
| 実装例 | JournalEntryService (借貸バランス検証、反転仕訳生成) |

---

## 4.6.1 JournalEntryService実装機能

| メソッド | 役割 | 使用箇所 |
|------------|------------|------------|
| validate_balance | 借貸バランス検証 | 全仕訳登録処理 |
| create_reversal_lines | 反転仕訳明細生成 | 取消・反対仕訳登録 |
| validate_correction | 修正仕訳検証 | 修正仕訳登録 |

**設計原則:**
- 複数の明細にまたがる処理はドメインサービスで実装
- アプリケーション層で同様のロジックを実装しない
- ドメイン知識はドメイン層に集約

---

# 第5章 Infrastructure層

## 5.1 目的

永続化および外部技術依存を実装する。

---

## 5.2 構成要素

| 要素 | 役割 |
|------------|------------|
| EventStore実装 | LMDBイベント保存 |
| ProjectionDB | ReadModel保存 |
| Repository実装 | RepositoryTrait具象化 |

---

## 5.3 Event Store仕様

| 項目 | 内容 |
|------------|------------|
| 保存方式 | 追記専用 |
| トランザクション | ACID準拠 |
| バージョン管理 | ストリーム単位 |

---

## 5.4 Projection DB仕様

| 項目 | 内容 |
|------------|------------|
| 保存内容 | Query最適化構造 |
| 再構築 | Event再生 |
| 独立性 | Projection単位で管理 |

---

# 第6章 依存関係ルール

| 層 | 依存可能対象 |
|------------|------------|
| Adapter | Application |
| Application | Domain |
| Infrastructure | Domain |
| Domain | 依存禁止 |

---

# 第7章 状態遷移原則

| 原則 | 内容 |
|------------|------------|
| Command入力 | Controller → InputPort |
| Command処理 | Interactor → Domain → Event生成 |
| 永続化 | RepositoryTrait → EventStore |
| Projection更新 | ProjectionBuilder |
| Query処理 | QueryService → Projection |
| 出力処理 | OutputPort ⇠ Presenter → View |

---

# 第8章 禁止事項一覧

| 項目 | 禁止内容 |
|------------|------------|
| Adapter層 | 業務判断 |
| Application層 | 業務ルール定義、ドメインロジックの重複実装 |
| Domain層 | UI依存 / Query最適化 |
| RepositoryTrait | 検索処理 |
| Infrastructure層 | 業務判断 |
| 全層 | Enumの文字列直接比較（FromStr使用）、ユーティリティクラスによるDTO変換 |

---

### 8.1 アンチパターン防止

以下のアンチパターンを避けること:

### 8.1.1 アプリケーション固有のアンチパターン

| アンチパターン | 問題 | 正しい実装 |
|------------|------------|------------|
| 文字列でEnum比較 | 型安全性の喪失 | FromStrトレイト使用 |
| DTO変換ユーティリティクラス | 責務の不明確化 | DTOにTryFrom実装 |
| ドメインロジックの再実装 | 一貫性喪失 | ドメインサービス呼び出し |
| 手動での借方貸方反転 | バグ混入リスク | JournalEntryService::create_reversal_lines使用 |
| format!("{:?}", status) | 表示形式の不安定性 | status.as_str()またはDisplay使用 |

### 8.1.2 Rust モダンプラクティス（Rust 1.93 / 2024 Edition対応）

本プロジェクトはRust 2024 Editionを前提とし、以下のモダンな書き方を推奨する。

#### (1) Rust 2024 Edition のデフォルトパターン

| 従来の書き方 | モダンな書き方 | 利点 |
|------------|------------|------------|
| `Box<dyn Fn() -> Box<dyn Future>>` | `-> impl Future` | 戻り位置impl Traitで簡潔化 |
| 手動キャプチャ指定 | 2024 Editionの自動キャプチャ | クロージャのキャプチャ方法が改善 |

#### (2) async クロージャと非同期ジェネレーター

```rust
// 従来: 冗長な Future 生成
let f = || async move { /* ... */ };

// モダン: async クロージャ（2024 Edition）
let f = async move |x| { /* ... */ };
```

**適用箇所:** 非同期処理を返すコールバックやイテレータ

#### (3) トレイトオブジェクトのアップキャスト

```rust
// 従来: 手動キャスト
let obj: Box<dyn SubTrait> = /* ... */;
let super_obj: Box<dyn SuperTrait> = unsafe { /* ... */ };

// モダン: 安全な暗黙的アップキャスト
let obj: Box<dyn SubTrait> = /* ... */;
let super_obj: Box<dyn SuperTrait> = obj; // 自動変換
```

**適用箇所:** PageStateやPresenterの抽象化

#### (4) 低レベルメモリ操作 API の標準化

```rust
// 従来: 手動分解
let ptr = vec.as_mut_ptr();
let len = vec.len();
let cap = vec.capacity();
std::mem::forget(vec);

// モダン: into_raw_parts
let (ptr, len, cap) = vec.into_raw_parts();
```

**適用箇所:** FFI連携、パフォーマンスクリティカルなコード

#### (5) 新しいコレクション API

```rust
// 従来: 手動条件チェック
if let Some(front) = deque.front() {
    if condition(front) {
        deque.pop_front();
    }
}

// モダン: pop_front_if
deque.pop_front_if(|x| condition(x));
```

**適用箇所:** NavigationStack、イベントキュー処理

#### (6) デフォルト lint の強化と安全性指向

```rust
// アンチパターン: null ポインタ逆参照（コンパイルエラー）
let ptr: *const i32 = std::ptr::null();
unsafe { *ptr } // deref_nullptr lint でエラー

// 正しい実装: Option で安全に扱う
let value: Option<&i32> = None;
if let Some(v) = value { /* ... */ }
```

**プロジェクト方針:** 
- `deref_nullptr` などの lint をデフォルトで有効化
- 定数内の可変性チェックを厳格化
- unsafe コードは最小限に抑え、必ず安全性コメントを付与

#### (7) ビルドスクリプトの文脈指定

```rust
// build.rs での条件分岐
fn main() {
    // 従来: 環境変数の手動チェック
    let debug = std::env::var("PROFILE").unwrap() == "debug";
    
    // モダン: CARGO_CFG_DEBUG_ASSERTIONS
    if std::env::var("CARGO_CFG_DEBUG_ASSERTIONS").is_ok() {
        println!("cargo:rustc-cfg=debug_mode");
    }
}
```

#### (8) cargo tree による依存関係管理

```bash
# 依存グラフのカスタマイズフォーマット
cargo tree --format "{p} {f}"

# 重複依存の検出
cargo tree --duplicates

# 特定クレートへの依存パス
cargo tree --invert javelin-domain
```

**プロジェクト方針:** 定期的に依存関係を整理し、不要な依存を削減

#### (9) 型システム改善の積極活用

```rust
// 戻り位置 impl Trait の活用
fn create_presenter() -> impl Presenter {
    ConcretePresenter::new()
}

// ジェネリック関連型の明示
trait Repository {
    type Item;
    type Error;
    fn load(&self) -> Result<Self::Item, Self::Error>;
}
```

**プロジェクト方針:** 
- 戻り位置 impl Trait を積極的に使用
- trait 解決の改善を活用し、ジェネリック境界を簡潔に記述

---

### 8.1.3 Rust 1.93 モダンプラクティス

以下は単なる糖衣構文ではなく、設計や安全性の前提が変わっている書き方。

#### (1) let-else による早期脱出

```rust
// 従来: ネストが深い
let user = match repo.find(id) {
    Some(u) => u,
    None => return Err(Error::NotFound),
};

// モダン: 意図が明確
let Some(user) = repo.find(id) else {
    return Err(Error::NotFound);
};
```

**利点:** ネストを減らし、失敗ケースを即座に処理。正常系が左寄せになる。

**適用箇所:** Controller、Interactor の入力検証

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

**利点:** 複数条件を平坦に記述。可読性が大幅に向上。

**適用箇所:** 複雑な条件分岐、バリデーション処理

#### (3) Option/Result の is_some_and / is_ok_and

```rust
// 従来: 冗長
if opt.map(|v| v > 5).unwrap_or(false) { ... }
if res.is_ok() && res.unwrap() > 5 { ... }

// モダン: 簡潔
if opt.is_some_and(|v| v > 5) { ... }
if res.is_ok_and(|v| v > 5) { ... }
```

**利点:** 存在確認と条件判定を1行で。unwrap の危険性を回避。

**適用箇所:** Option/Result の条件チェック全般

#### (4) std::path::Path の直接比較

```rust
// 従来: 不要なヒープ確保
use std::path::PathBuf;
if path == PathBuf::from("config.toml") { ... }

// モダン: Path::new で直接比較
use std::path::Path;
if path == Path::new("config.toml") { ... }
```

**利点:** ヒープ確保を避け、パフォーマンス向上。

**適用箇所:** ファイルパス比較、設定ファイル読み込み

#### (5) IntoIterator for arrays（配列を move で回す）

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

**利点:** 所有権の意図が明確。Copy 型なら move でも問題なし。

**適用箇所:** 定数配列のイテレーション

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

**利点:** 外部依存を削減。グローバル初期化の標準解。

**適用箇所:** アプリケーション設定、グローバルリソース

#### (7) Error 実装に thiserror + anyhow 前提設計

```rust
// 従来: 手書き impl Error
enum AppError {
    Io(std::io::Error),
    Parse(String),
}
impl std::fmt::Display for AppError { /* ... */ }
impl std::error::Error for AppError { /* ... */ }

// モダン: thiserror で簡潔に
#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    
    #[error("parse error: {0}")]
    Parse(String),
}
```

**利点:** ボイラープレートを削減。#[from] で自動変換。

**プロジェクト方針:** 
- Domain層: 独自エラー型（thiserror使用）
- Application層: ApplicationError（thiserror使用）
- Adapter層: anyhow::Result でラップ

#### (8) async fn in traits（安定化済）

```rust
// 従来: async-trait マクロ必須
#[async_trait]
trait Repository {
    async fn find(&self, id: Id) -> Result<Entity>;
}

// モダン: 標準サポート
trait Repository {
    async fn find(&self, id: Id) -> Result<Entity>;
}
```

**注意:** オブジェクト安全性には制約あり。dyn Trait として使う場合は検討が必要。

**適用箇所:** 将来的な非同期Repository実装時に検討

#### (9) cfg の細粒度化

```rust
// 従来: ブロック単位
#[cfg(target_arch = "x86_64")]
{
    asm!("nop");
    asm!("mfence");
}

// モダン: 文単位
asm!(
    "nop",
    #[cfg(target_arch = "x86_64")]
    "mfence",
);
```

**利点:** 条件分岐が細かく制御可能。低レベル実装が整理しやすい。

**適用箇所:** プラットフォーム固有の最適化

#### (10) let パターンによる構造分解強化

```rust
// 従来: match で分解
match point {
    Point { x, y, .. } => {
        // x, y を使用
    }
}

// モダン: let で直接分解
let Point { x, y, .. } = point;
// x, y がそのまま使える
```

**利点:** match を書かずに束縛。.. の活用で保守性向上。

**適用箇所:** 構造体の部分的な値取得

---

### 8.1.4 補足的傾向（実務設計指針）

| 項目 | 従来 | モダン | 理由 |
|------------|------------|------------|------------|
| 並行制御 | Arc<Mutex<T>> | tokio::sync 系 / parking_lot | 用途に応じた選択。tokio::sync::RwLock は非同期対応 |
| メモリ確保 | Vec::new() | Vec::with_capacity(n) | 事前確保でリアロケーション削減 |
| エラー型 | Box<dyn Error> | 具体型 Result<T, E> | 型安全性とパフォーマンス |
| CI/CD | 手動チェック | clippy 強制 | cargo clippy -- -D warnings |
| 依存管理 | 多数の小クレート | 標準ライブラリ優先 | OnceLock, LazyLock 等の活用 |

---

### 8.1.5 プロジェクト固有の Rust スタイルガイド

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

### 8.1.6 設計の明示性とヒープ挙動の制御

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

# 第9章 テスト戦略

| 層 | テスト種別 |
|------------|------------|
| Domain | 単体テスト |
| Application Command | ユースケーステスト |
| Application Query | Projection検索テスト |
| Infrastructure | 永続統合テスト |
| ProjectionBuilder | 再構築テスト |
| Navigation | ナビゲーションフローテスト |

---

# 第10章 不変設計原則

| 原則 | 説明 |
|------------|------------|
| Event正本 | イベントのみが真実 |
| Projection派生 | 再生成可能 |
| CQRS分離 | Command / Query独立 |
| DTO境界 | ドメイン完全隠蔽 |
| Trait駆動 | 実装依存排除 |
| Append Only | 更新禁止 |

---

# 第11章 命名規則

| 要素 | 命名 |
|------------|------------|
| InputPort | XxxUseCase |
| Interactor | XxxInteractor |
| QueryService | XxxQueryService |
| ProjectionBuilder | XxxProjectionBuilder |
| OutputPort | XxxOutputPort |
| Controller | XxxController |
| Presenter | XxxPresenter |
| Repository | XxxEventRepository |
| PageState | XxxPageState |

---

*本ドキュメントはClean Architecture + Event Sourcing + CQRSを前提とする。*
