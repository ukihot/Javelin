# Command/Query 分離パターン完了報告

## 実施内容

### 1. Command インタラクタの修正（既存集約の状態変更）

以下のインタラクタを `load/save` パターンに統一しました：

| インタラクタ | 操作 | 修正内容 |
|------------|------|---------|
| `ApproveJournalEntryInteractor` | 承認 | `load()` → `approve()` → `save()` |
| `RejectJournalEntryInteractor` | 差戻し | `load()` → `reject()` → `save()` |
| `CorrectJournalEntryInteractor` | 修正 | `load()` → `correct()` → `save()` |
| `SubmitForApprovalInteractor` | 承認申請 | `load()` → `submit_for_approval()` → `save()` |
| `ReverseJournalEntryInteractor` | 取消 | `load()` → `reverse()` → `save()` |

### 2. Query インタラクタの明確化

以下のインタラクタに、Query 処理であることを明記するコメントを追加：

- `LoadAccountMasterInteractor`
- `LoadSubsidiaryAccountMasterInteractor`
- `LoadCompanyMasterInteractor`
- `LoadApplicationSettingsInteractor`

これらは正しく `QueryService` を使用しており、`Repository.load()` は使用していません。

### 3. ドキュメント作成

- `LOAD_VS_QUERY_PATTERN.md` - load と QueryService の使い分けガイド
- `LOAD_PATTERN_FIXES.md` - 修正内容の詳細
- `COMMAND_QUERY_SEPARATION_COMPLETE.md` - 本ドキュメント

## 正しいパターン

### Command 処理（状態変更）

```rust
// ✅ 正しい
async fn execute(&self, request: Request) -> Result<()> {
    // 1. load() で集約を復元（イベントストリームから）
    let mut aggregate = self.repository.load(&id).await?
        .ok_or(NotFound)?;
    
    // 2. ビジネスロジック実行（集約内部でイベント生成）
    aggregate.do_something(params)?;
    
    // 3. save() で永続化（インフラ層で uncommitted_events を保存）
    self.repository.save(&aggregate).await?;
    
    Ok(())
}
```

### Query 処理（検索・参照）

```rust
// ✅ 正しい
async fn execute(&self, request: Request) -> Result<Response> {
    // QueryService を使用（Repository は使わない）
    let data = self.query_service.get(params).await?;
    
    Ok(Response { data })
}
```

### 新規集約作成

```rust
// ✅ 正しい
async fn execute(&self, request: Request) -> Result<()> {
    // 1. 新規集約を作成（コンストラクタ内でイベント生成）
    let aggregate = Aggregate::new(params)?;
    
    // 2. save() で永続化
    self.repository.save(&aggregate).await?;
    
    Ok(())
}
```

## アーキテクチャの責務分離

### ドメイン層

```rust
// RepositoryBase トレイト
pub trait RepositoryBase<T> {
    async fn save(&self, aggregate: &T) -> DomainResult<()>;
    async fn load(&self, id: &str) -> DomainResult<Option<T>>;
}

// 集約エンティティ
impl JournalEntry {
    // ビジネスロジック実行時にイベントを生成
    pub fn approve(&mut self, ...) -> DomainResult<()> {
        // 状態変更
        self.status = JournalStatus::Posted;
        
        // イベント生成
        self.uncommitted_events.push(JournalEntryEvent::Approved { ... });
        
        Ok(())
    }
    
    // 未コミットイベントを取得
    pub fn uncommitted_events(&self) -> &[JournalEntryEvent] {
        &self.uncommitted_events
    }
}
```

### アプリケーション層

```rust
// Command インタラクタ
impl ApproveJournalEntryInteractor {
    async fn execute(&self, request: Request) -> Result<()> {
        let mut journal = self.repository.load(&id).await?.ok_or(NotFound)?;
        journal.approve(params)?;
        self.repository.save(&journal).await?;
        Ok(())
    }
}

// Query インタラクタ
impl LoadAccountMasterInteractor {
    async fn execute(&self, request: Request) -> Result<Response> {
        let accounts = self.query_service.get_all().await?;
        Ok(Response { accounts })
    }
}
```

### インフラ層

```rust
// Repository 実装
impl JournalEntryRepository for EventSourcedJournalEntryRepository {
    async fn save(&self, aggregate: &JournalEntry) -> DomainResult<()> {
        // 集約から未コミットイベントを取得
        let events = aggregate.uncommitted_events();
        
        // イベントストアに保存
        self.event_store.append_events(aggregate.id(), events).await?;
        
        Ok(())
    }
    
    async fn load(&self, id: &str) -> DomainResult<Option<JournalEntry>> {
        // イベントストアからイベントを取得
        let events = self.event_store.get_events(id).await?;
        
        if events.is_empty() {
            return Ok(None);
        }
        
        // イベントから集約を復元
        let aggregate = JournalEntry::from_events(events)?;
        
        Ok(Some(aggregate))
    }
}
```

## 判断基準

| 操作 | 使用するもの | 理由 |
|------|------------|------|
| 既存集約の状態変更 | `Repository.load()` + `save()` | 現在状態が必要 |
| 新規集約作成 | `Aggregate::new()` + `Repository.save()` | 新規作成 |
| 検索・参照 | `QueryService` | 読み取り専用 |
| マスタデータ参照 | `QueryService` | 状態変更しない |
| 集計・分析 | `QueryService` | 複数件、集計 |

## 重要な原則

1. **load は 1 Aggregate だけ**
   - `load(id)` で 1 件のみ
   - `load_all()` や `find_by_x()` は存在しない
   - 理由: Aggregate = 一貫性境界

2. **イベント操作はインフラ層の責務**
   - アプリケーション層は `get_events()` や `append_events()` を使わない
   - `load()` と `save()` のみを使用

3. **Query は Repository を使わない**
   - `QueryService` を使用
   - Projection DB から直接読み取り

4. **イベント生成は集約内部**
   - インタラクタでイベントを生成しない
   - 集約のメソッド内で `uncommitted_events` に追加

## 次のステップ

以下のインタラクタは新規作成系なので、現在の実装で問題ありませんが、
`events()` メソッドが存在しない場合は `uncommitted_events()` を使用するように修正が必要です：

- `RegisterJournalEntryInteractor`
- `CancelJournalEntryInteractor`
- `CreateReversalEntryInteractor`
- `CreateReplacementEntryInteractor`
- `CreateReclassificationEntryInteractor`
- `CreateAdditionalEntryInteractor`

これらは `JournalEntry::new()` → `repository.save()` のパターンが正しいです。
