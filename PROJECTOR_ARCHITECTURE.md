# Projector アーキテクチャ - CQRS Read Model 構築

## 概要

CQRS（Command Query Responsibility Segregation）パターンにおいて、Read Model（Projection）の構築を担当するProjectorを実装しました。

## アーキテクチャ

### イベントフロー

```
EventStore (Write Model)
    ↓ イベント保存
    ↓ 通知コールバック
ProjectionBuilder
    ↓ イベント振り分け
ProjectorRegistry
    ↓ 各Projectorへ配信
┌─────────────────────────────────────┐
│  JournalEntryProjector              │ → ProjectionDB (仕訳一覧)
│  AccountMasterProjector             │ → ProjectionDB (勘定科目マスタ)
│  LedgerProjector                    │ → ProjectionDB (元帳)
│  TrialBalanceProjector              │ → ProjectionDB (試算表)
└─────────────────────────────────────┘
    ↓ Read Model更新
QueryService (Application Layer)
    ↓ 検索・照会（ドメイン層を経由しない）
Application Layer (Interactor)
```

## 実装されたProjector

### 1. JournalEntryProjector
**責務**: 仕訳一覧Projectionの更新

**購読イベント**:
- `DraftCreated` - 下書き作成
- `SubmittedForApproval` - 承認申請
- `Approved` - 承認
- `Rejected` - 却下
- `Updated` - 更新
- `Deleted` - 削除
- `Corrected` - 訂正
- `Reversed` - 取消

**Projection構造**:
```rust
StoredJournalEntry {
    entry_id: String,
    entry_number: Option<String>,
    status: String,
    transaction_date: String,
    voucher_number: String,
    description: String,
    total_debit: f64,
    total_credit: f64,
    created_by: String,
    created_at: String,
    updated_by: Option<String>,
    updated_at: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    lines: Vec<StoredJournalEntryLine>,
}
```

**キー形式**: `journal_entry:{entry_id}`

### 2. AccountMasterProjector
**責務**: 勘定科目マスタProjectionの更新

**購読イベント**:
- `AccountMasterCreated` - 勘定科目作成
- `AccountMasterUpdated` - 勘定科目更新
- `AccountMasterDeleted` - 勘定科目削除（論理削除）

**Projection構造**:
```json
{
    "code": "1000",
    "name": "現金",
    "account_type": "Asset",
    "is_active": true
}
```

**キー形式**: `account_master:{code}`

### 3. LedgerProjector
**責務**: 元帳Projectionの更新

**購読イベント**:
- `Approved` - 承認済み仕訳のみを元帳に転記

**Projection構造**:
```rust
StoredLedgerData {
    account_name: String,
    opening_balance: f64,
    entries: Vec<StoredLedgerEntry>,
}
```

**キー形式**: `ledger:{account_code}:{year}:{month}`

**特徴**:
- 承認済み仕訳のみが元帳に転記される
- 勘定科目ごと・年月ごとに集計
- 借方・貸方の金額を記録

### 4. TrialBalanceProjector
**責務**: 試算表Projectionの更新

**購読イベント**:
- `Approved` - 承認済み仕訳のみを試算表に反映

**Projection構造**:
```rust
StoredTrialBalanceData {
    entries: Vec<StoredTrialBalanceEntry>,
}

StoredTrialBalanceEntry {
    account_code: String,
    account_name: String,
    debit_amount: f64,
    credit_amount: f64,
}
```

**キー形式**: `trial_balance:{year}:{month}`

**特徴**:
- 勘定科目ごとの借方・貸方合計を集計
- 年月ごとに試算表を生成

## Projectorトレイト

すべてのProjectorは共通のトレイトを実装します：

```rust
pub trait Projector: Send + Sync {
    /// このProjectorが処理対象とするイベントタイプのリスト
    fn event_types(&self) -> Vec<&'static str>;

    /// イベントを処理してProjectionを更新
    async fn project(&self, event: &StoredEvent) -> InfrastructureResult<()>;

    /// このProjectorが指定されたイベントを処理すべきか判定
    fn should_process(&self, event: &StoredEvent) -> bool {
        self.event_types().contains(&event.event_type.as_str())
    }
}
```

## ProjectorRegistry

すべてのProjectorを管理し、イベントを適切なProjectorに振り分けます。

**静的ディスパッチ（ジェネリクス）を使用**してゼロコスト抽象化を実現しています。

```rust
pub struct ProjectorRegistry<J, A, L, T>
where
    J: Projector,
    A: Projector,
    L: Projector,
    T: Projector,
{
    journal_entry_projector: Arc<J>,
    account_master_projector: Arc<A>,
    ledger_projector: Arc<L>,
    trial_balance_projector: Arc<T>,
}

impl<J, A, L, T> ProjectorRegistry<J, A, L, T>
where
    J: Projector,
    A: Projector,
    L: Projector,
    T: Projector,
{
    pub async fn process_event(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        // 各Projectorに対してイベントを処理（静的ディスパッチ）
        if self.journal_entry_projector.should_process(event) {
            self.journal_entry_projector.project(event).await?;
        }

        if self.account_master_projector.should_process(event) {
            self.account_master_projector.project(event).await?;
        }

        if self.ledger_projector.should_process(event) {
            self.ledger_projector.project(event).await?;
        }

        if self.trial_balance_projector.should_process(event) {
            self.trial_balance_projector.project(event).await?;
        }

        Ok(())
    }
}
```

### 静的ディスパッチの利点

1. **ゼロコスト抽象化**: コンパイル時に型が確定し、実行時のオーバーヘッドなし
2. **インライン化**: コンパイラが積極的にインライン化を行える
3. **型安全性**: コンパイル時に型チェックが完了
4. **パフォーマンス**: 動的ディスパッチ（vtable lookup）のコストを回避

## ProjectionBuilderImpl の役割

ProjectionBuilderImplは以下の責務を持ちます：

1. **イベント通知の受信**: EventStoreからイベント通知を受け取る
2. **ProjectorRegistryへの委譲**: イベントをProjectorRegistryに渡す
3. **エラーハンドリング**: Projection更新エラーを処理
4. **再試行キュー**: 失敗したイベントを再試行キューに追加
5. **Projection再構築**: 全イベントを再処理してProjectionを再構築

```rust
pub struct ProjectionBuilderImpl<J, A, L, T>
where
    J: Projector,
    A: Projector,
    L: Projector,
    T: Projector,
{
    projection_db: Arc<ProjectionDb>,
    event_store: Arc<EventStore>,
    projector_registry: Arc<ProjectorRegistry<J, A, L, T>>,
    retry_queue: Arc<Mutex<VecDeque<RetryQueueEntry>>>,
    error_sender: Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>,
}
```

**具体的な型**:
```rust
type ConcreteProjectionBuilder = ProjectionBuilderImpl<
    JournalEntryProjector,
    AccountMasterProjector,
    LedgerProjector,
    TrialBalanceProjector,
>;
```

## イベント購読の仕組み

### 1. EventStoreへのコールバック登録

```rust
// app_setup.rs
let notification_handler =
    projection_builder.clone().create_event_notification_handler(infra_error_sender);
event_store.set_notification_callback(notification_handler);
```

### 2. イベント保存時の自動通知

```rust
// EventStore::append()
// イベント保存後に通知コールバックを呼び出す
if let Some(callback) = callback_opt {
    for event in stored_events {
        tokio::spawn(async move {
            callback(event).await;
        });
    }
}
```

### 3. Projectionの自動更新

```rust
// ProjectionBuilderImpl::process_event_internal()
async fn process_event_internal(&self, event: &StoredEvent) -> ApplicationResult<()> {
    // ProjectorRegistryを使用してイベントを処理
    self.projector_registry.process_event(event).await?;
    Ok(())
}
```

## CQRS原則の遵守

### Command側（Write Model）
- **EventStore**: イベントを永続化
- **Repository**: 集約をイベントから再構築
- **Aggregate**: ビジネスロジックを実行してイベントを生成

### Query側（Read Model）
- **Projector**: イベントを購読してRead Modelを更新
- **ProjectionDB**: 読み取り専用データベース
- **QueryService**: Read Modelから検索・照会（アプリケーション層のインターフェース）

### 分離の利点
1. **スケーラビリティ**: Read/Writeを独立してスケール可能
2. **最適化**: 各側で最適なデータ構造を選択可能
3. **柔軟性**: 新しいRead Modelを追加しても既存に影響なし
4. **パフォーマンス**: 読み取りクエリがドメイン層を経由せず高速
5. **シンプルさ**: QueryServiceはドメインロジックを含まず、単純なデータ取得のみ

## エラーハンドリングと再試行

### 再試行キュー
Projection更新に失敗したイベントは再試行キューに追加されます：

```rust
fn add_to_retry_queue(&self, event: StoredEvent, error: String) {
    let mut queue = self.retry_queue.lock().unwrap();
    queue.push_back(RetryQueueEntry { 
        event, 
        retry_count: 0, 
        last_error: error 
    });
}
```

### 再試行処理
最大3回まで再試行し、それでも失敗した場合はログに記録：

```rust
pub async fn process_retry_queue(&self) -> ApplicationResult<()> {
    const MAX_RETRIES: u32 = 3;
    // 再試行ロジック
}
```

## Projection再構築

全イベントを再処理してProjectionを再構築：

```rust
async fn rebuild_all_projections(&self) -> ApplicationResult<()> {
    // EventStoreから全イベントを取得（シーケンス0から）
    let events = self.event_store.get_all_events(0).await?;

    // 各イベントを順次処理
    for event in events.iter() {
        self.process_event_internal(event).await?;
    }

    Ok(())
}
```

## ファイル構成

```
crates/javelin-infrastructure/src/read/
├── projectors.rs                          # Projectorモジュール定義
├── projectors/
│   ├── journal_entry_projector.rs         # 仕訳一覧Projector
│   ├── account_master_projector.rs        # 勘定科目マスタProjector
│   ├── ledger_projector.rs                # 元帳Projector
│   ├── trial_balance_projector.rs         # 試算表Projector
│   └── registry.rs                        # ProjectorRegistry
└── infrastructure/
    ├── builder.rs                         # ProjectionBuilderImpl
    ├── db.rs                              # ProjectionDB
    └── traits.rs                          # Projectionトレイト
```

## 今後の拡張

新しいRead Modelを追加する場合：

1. 新しいProjectorを作成（`Projector`トレイトを実装）
2. `ProjectorRegistry::new()`に追加
3. 必要に応じてQueryServiceを実装

例：
```rust
// 新しいProjectorの追加
pub struct CashFlowProjector {
    projection_db: Arc<ProjectionDb>,
}

impl Projector for CashFlowProjector {
    fn event_types(&self) -> Vec<&'static str> {
        vec!["Approved"]
    }

    async fn project(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        // キャッシュフローProjectionを更新
        Ok(())
    }
}
```
