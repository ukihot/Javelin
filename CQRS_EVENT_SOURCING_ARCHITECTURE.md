# CQRS + Event Sourcing Architecture

## Overview

このプロジェクトは、純粋なCQRS（Command Query Responsibility Segregation）とEvent Sourcingパターンを採用しています。

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                     Presentation Layer                       │
│                    (javelin-adapter)                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│                  (javelin-application)                       │
│                                                              │
│  ┌──────────────────┐         ┌──────────────────┐         │
│  │   Interactor     │         │  QueryService    │         │
│  │  (Use Cases)     │         │   Interface      │         │
│  │                  │         │  (Read Model)    │         │
│  └──────────────────┘         └──────────────────┘         │
│          │                             │                     │
│          │                             │ (直接アクセス)      │
└──────────┼─────────────────────────────┼─────────────────────┘
           │                             │
           ▼                             │
┌──────────────────────┐                 │
│   Domain Layer       │                 │
│  (javelin-domain)    │                 │
│                      │                 │
│  ┌────────────────┐ │                 │
│  │  Repository    │ │                 │
│  │  Interface     │ │                 │
│  └────────────────┘ │                 │
│  ┌────────────────┐ │                 │
│  │   Aggregate    │ │                 │
│  │   Entities     │ │                 │
│  └────────────────┘ │                 │
│  ┌────────────────┐ │                 │
│  │ Domain Events  │ │                 │
│  └────────────────┘ │                 │
└──────────────────────┘                 │
           │                             │
           ▼                             ▼
┌──────────────────────┐      ┌──────────────────────┐
│ Infrastructure Layer │      │ Infrastructure Layer │
│ (javelin-infra)      │      │ (javelin-infra)      │
│                      │      │                      │
│  WRITE SIDE          │      │  READ SIDE           │
│  ┌────────────────┐ │      │  ┌────────────────┐ │
│  │ RepositoryImpl │ │      │  │QueryServiceImpl│ │
│  └────────────────┘ │      │  └────────────────┘ │
│          │           │      │          │          │
│          ▼           │      │          ▼          │
│  ┌────────────────┐ │      │  ┌────────────────┐ │
│  │  EventStore    │ │      │  │ ProjectionDB   │ │
│  │    (LMDB)      │ │      │  │    (LMDB)      │ │
│  └────────────────┘ │      │  └────────────────┘ │
│          │           │      │          ▲          │
│          │           │      │          │          │
│          └───────────┼──────┼──────────┘          │
│                      │      │  Event Stream       │
└──────────────────────┘      └──────────────────────┘
```

## Core Principles

### 1. Command Side (Write Side)

**責務**: 集約の状態変更とイベントの永続化

#### Repository Interface (Domain Layer)
```rust
// crates/javelin-domain/src/repositories/journal_entry_repository.rs
pub trait JournalEntryRepository: RepositoryBase<Event = JournalEntryEvent> {
    // 集約固有のメソッドを追加可能
}

pub trait RepositoryBase {
    type Event: DomainEvent;
    
    async fn append(&self, event: Self::Event) -> DomainResult<()>;
    async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>;
    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>>;
    async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>>;
    async fn get_latest_sequence(&self) -> DomainResult<u64>;
}
```

#### Repository Implementation (Infrastructure Layer)
```rust
// crates/javelin-infrastructure/src/write/repositories/journal_entry_repository_impl.rs
pub struct JournalEntryRepositoryImpl {
    event_store: Arc<EventStore>,
}

impl JournalEntryRepository for JournalEntryRepositoryImpl {}

impl RepositoryBase for JournalEntryRepositoryImpl {
    type Event = JournalEntryEvent;
    
    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        // EventStoreにイベントを保存
        self.event_store.append_event(...).await
    }
    
    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>> {
        // EventStoreからイベントを取得
        self.event_store.get_events(aggregate_id).await
    }
}
```

#### EventStore (Infrastructure Layer)
```rust
// crates/javelin-infrastructure/src/write/event_store/store.rs
pub struct EventStore {
    env: Arc<Environment>,
    events_db: Database,
    meta_db: Database,
}

impl EventStore {
    // イベントの永続化
    pub async fn append_event(...) -> InfrastructureResult<Sequence>;
    pub async fn append<T>(...) -> InfrastructureResult<u64>;
    
    // イベントの取得
    pub async fn get_events(&self, aggregate_id: &str) -> InfrastructureResult<Vec<StoredEvent>>;
    pub async fn get_all_events(&self, from_sequence: u64) -> InfrastructureResult<Vec<StoredEvent>>;
    
    // イベント通知
    pub fn set_notification_callback(&self, callback: EventNotificationCallback);
}
```

### 2. Query Side (Read Side)

**責務**: 読み取り最適化されたデータの提供

#### QueryService Interface (Application Layer)

QueryServiceインターフェースはアプリケーション層に配置されます。
ドメイン層を経由せず、ProjectionDBから直接読み取るため高速です。

**重要**: クエリ（読み取り）操作にはInteractorを使用しません。
Controller層がQueryServiceを直接呼び出します。

```rust
// crates/javelin-application/src/query_service/journal_entry_query_service.rs
pub trait JournalEntryQueryService: Send + Sync {
    async fn find_by_id(&self, id: &str) -> ApplicationResult<Option<JournalEntryDto>>;
    async fn find_all(&self) -> ApplicationResult<Vec<JournalEntryDto>>;
    async fn search(&self, criteria: SearchCriteria) -> ApplicationResult<Vec<JournalEntryDto>>;
}
```

#### QueryService Implementation (Infrastructure Layer)
```rust
// crates/javelin-infrastructure/src/read/journal_entry/query_service_impl.rs
pub struct JournalEntryQueryServiceImpl {
    projection_db: Arc<ProjectionDb>,
}

impl JournalEntryQueryService for JournalEntryQueryServiceImpl {
    async fn find_by_id(&self, id: &str) -> ApplicationResult<Option<JournalEntryDto>> {
        // ProjectionDBから直接読み取り（ドメイン層を経由しない）
        self.projection_db.get(id).await
    }
}
```

**CQRS の利点**:
- **高速な読み取り**: ドメイン層を経由せず、ProjectionDBから直接読み取り
- **最適化されたデータ構造**: 読み取り専用に最適化されたスキーマ
- **スケーラビリティ**: Read/Writeを独立してスケール可能
- **シンプルな実装**: クエリにInteractorが不要で、Controller→QueryServiceの直接呼び出し

#### ProjectionDB (Infrastructure Layer)
```rust
// crates/javelin-infrastructure/src/read/infrastructure/projection_db.rs
pub struct ProjectionDb {
    env: Arc<Environment>,
    db: Database,
}

impl ProjectionDb {
    pub async fn get<T>(&self, key: &str) -> InfrastructureResult<Option<T>>;
    pub async fn put<T>(&self, key: &str, value: &T) -> InfrastructureResult<()>;
    pub async fn delete(&self, key: &str) -> InfrastructureResult<()>;
}
```

### 3. Event Flow

```
Command Flow (書き込み):
1. Command → Controller
2. Controller → Interactor
3. Interactor → Repository.save(aggregate)
4. Repository → EventStore.append(events)
5. EventStore → Notification Callback
6. Callback → ProjectorRegistry
7. ProjectorRegistry → 各Projector
8. Projector → ProjectionDB.update()

Query Flow (読み取り):
1. Query → Controller
2. Controller → QueryService (Interactorを経由しない)
3. QueryService → ProjectionDB.get()
4. ProjectionDB → Response
```

**重要な設計原則**:
- **Command側**: Controller → Interactor → Domain → Repository → EventStore
- **Query側**: Controller → QueryService → ProjectionDB（Interactorなし）
- マスタデータは「ディメンション」としてProjectionDBに存在
- アプリケーション層に「master_data」という概念は存在しない

## Aggregate Types

### Event-Sourced Aggregates

これらの集約はイベントソーシングを使用し、すべての状態変更がイベントとして記録されます。

- **JournalEntry** (仕訳伝票)
  - Repository: `JournalEntryRepository`
  - Events: `JournalEntryEvent`
  - EventStore: 使用
  
- **Closing** (月次決算)
  - Repository: `ClosingRepository`
  - Events: `ClosingEvent`
  - EventStore: 使用

### Master Data Aggregates

これらの集約はマスタデータであり、イベントソーシングは使用しません。
LMDBに直接保存されます。

- **AccountMaster** (勘定科目マスタ)
  - Repository: `AccountMasterRepository`
  - Storage: LMDB直接
  - Events: オプション（監査用）
  
- **CompanyMaster** (会社マスタ)
  - Repository: `CompanyMasterRepository`
  - Storage: LMDB直接
  - Events: オプション（監査用）
  
- **SubsidiaryAccountMaster** (補助科目マスタ)
  - Repository: `SubsidiaryAccountMasterRepository`
  - Storage: LMDB直接
  - Events: オプション（監査用）

## Directory Structure

```
crates/
├── javelin-domain/              # ドメイン層
│   ├── src/
│   │   ├── repositories/        # リポジトリインターフェース（Command側のみ）
│   │   │   ├── repository_base.rs
│   │   │   ├── journal_entry_repository.rs
│   │   │   ├── closing_repository.rs
│   │   │   ├── account_master_repository.rs
│   │   │   └── company_master_repository.rs
│   │   ├── financial_close/
│   │   │   └── journal_entry/
│   │   │       ├── entities.rs  # 集約エンティティ
│   │   │       └── events.rs    # ドメインイベント
│   │   └── masters/
│   │       ├── account_master.rs
│   │       └── events.rs
│   
├── javelin-application/         # アプリケーション層
│   ├── src/
│   │   ├── interactor/          # ユースケース（Command側のみ）
│   │   │   ├── journal_entry/
│   │   │   └── closing/
│   │   └── query_service/       # QueryServiceインターフェース（Query側）
│   │       ├── journal_entry_query_service.rs
│   │       ├── account_master_query_service.rs
│   │       ├── company_master_query_service.rs
│   │       └── subsidiary_account_master_query_service.rs
│   
└── javelin-infrastructure/      # インフラ層
    ├── src/
    │   ├── write/               # Command側
    │   │   ├── event_store/     # イベントストア
    │   │   │   ├── store.rs
    │   │   │   ├── event_stream.rs
    │   │   │   └── snapshot_db.rs
    │   │   └── repositories/    # リポジトリ実装
    │   │       ├── journal_entry_repository_impl.rs
    │   │       ├── closing_repository_impl.rs
    │   │       ├── account_master_repository_impl.rs
    │   │       └── company_master_repository_impl.rs
    │   │
    │   └── read/                # Query側
    │       ├── infrastructure/
    │       │   ├── projection_db.rs
    │       │   └── builder.rs   # ProjectionBuilder
    │       ├── projectors/      # Projector（イベント購読）
    │       │   ├── journal_entry_projector.rs
    │       │   ├── account_master_projector.rs
    │       │   ├── ledger_projector.rs
    │       │   ├── trial_balance_projector.rs
    │       │   └── registry.rs  # ProjectorRegistry
    │       ├── journal_entry/
    │       │   ├── query_service_impl.rs
    │       │   └── projection.rs
    │       ├── account_master/
    │       │   ├── query_service_impl.rs
    │       │   └── projection.rs
    │       ├── company_master/
    │       │   ├── query_service_impl.rs
    │       │   └── projection.rs
    │       └── subsidiary_account_master/
    │           ├── query_service_impl.rs
    │           └── projection.rs
```

## Key Design Decisions

### 1. Repository vs EventStore の分離

**決定**: RepositoryとEventStoreは別のコンポーネント

**理由**:
- Repository: ドメイン層のインターフェースを実装し、集約のロード/保存を担当
- EventStore: イベントの永続化と取得のみを担当
- 責務の分離により、テスタビリティと保守性が向上

### 2. Master Data の扱い

**決定**: マスタデータはイベントソーシングを使用しない

**理由**:
- マスタデータは頻繁に変更されない
- 履歴管理が不要な場合が多い
- LMDB直接アクセスでパフォーマンスが向上
- 必要に応じて監査用イベントを発行可能

### 3. Projection の更新

**決定**: EventStoreのnotification callbackを使用

**理由**:
- イベント保存と同時にProjectionを更新
- 非同期処理により書き込みパフォーマンスを維持
- 疎結合な設計

### 4. Query Service の配置

**決定**: QueryServiceインターフェースはApplication層、実装はInfrastructure層

**理由**:
- **高速な読み取り**: ドメイン層を経由せず、ProjectionDBから直接読み取り
- **CQRS原則**: Command側（ドメイン層）とQuery側（アプリケーション層）を完全に分離
- **依存関係の逆転**: アプリケーション層がインターフェースを定義し、インフラ層が実装
- **テスト容易性**: テスト時にモック実装を使用可能
- **パフォーマンス**: ドメインロジックを経由しないため、読み取りが高速

## Testing Strategy

### Unit Tests
- Domain層: 集約のビジネスロジック
- Application層: Interactorのユースケース

### Integration Tests
- Repository実装とEventStoreの統合
- QueryService実装とProjectionDBの統合
- Event → Projection の更新フロー

### End-to-End Tests
- Command → Event → Projection → Query の完全なフロー

## Performance Considerations

### Write Side
- イベントの一括追記: `append_events()`
- 楽観的ロック: `ExpectedVersion`
- 非同期イベント通知

### Read Side
- Projection の事前計算
- インデックスの最適化
- キャッシング戦略

## Future Enhancements

1. **Snapshot機能**: 大量のイベントがある集約の復元を高速化
2. **Event Replay**: Projectionの再構築機能
3. **Event Versioning**: イベントスキーマの進化対応
4. **Saga Pattern**: 複数集約にまたがるトランザクション
5. **Event Sourcing for Master Data**: 必要に応じてマスタデータもイベントソーシング化
