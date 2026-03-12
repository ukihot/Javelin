# リポジトリ命名修正完了

## 実施内容

すべてのインタラクタで `event_repository` を適切な具体的なリポジトリ名に変更し、`append_events()` を `save()` に統一しました。

## 修正対象

### 1. フィールド名の変更

| 変更前 | 変更後 |
|--------|--------|
| `event_repository: Arc<R>` | `journal_entry_repository: Arc<R>` |

### 2. メソッドの変更

| 変更前 | 変更後 | 用途 |
|--------|--------|------|
| `append_events()` | `save()` | 集約の永続化 |
| `get_events()` | `load()` | 集約の復元 |
| `events()` | `uncommitted_events()` | イベント取得 |

## 修正したインタラクタ一覧

### Command処理（既存集約の状態変更）

1. `ApproveJournalEntryInteractor`
   - `load()` → `approve()` → `save()`

2. `RejectJournalEntryInteractor`
   - `load()` → `reject()` → `save()`

3. `CorrectJournalEntryInteractor`
   - `load()` → `correct()` → `save()`

4. `SubmitForApprovalInteractor`
   - `load()` → `submit_for_approval()` → `save()`

5. `ReverseJournalEntryInteractor`
   - `load()` → `reverse()` → `save()`

6. `UpdateDraftJournalEntryInteractor`
   - `load()` → 更新処理 → `save()`
   - TODO: 集約に `update_lines()`, `update_transaction_date()`, `update_voucher_number()` メソッドが必要

7. `DeleteDraftJournalEntryInteractor`
   - `load()` → 削除処理 → `save()`
   - TODO: 集約に `delete()` メソッドが必要

### Command処理（新規集約の作成）

8. `RegisterJournalEntryInteractor`
   - `JournalEntry::new()` → `save()`

9. `CancelJournalEntryInteractor`
   - `JournalEntry::new()` → `save()`

10. `CreateReversalEntryInteractor`
    - `JournalEntry::new()` → `save()`

11. `CreateReplacementEntryInteractor`
    - `JournalEntry::new()` → `save()`

12. `CreateReclassificationEntryInteractor`
    - `JournalEntry::new()` → `save()`

13. `CreateAdditionalEntryInteractor`
    - `JournalEntry::new()` → `save()`

## 正しいパターン

### 既存集約の状態変更

```rust
pub struct ApproveJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,  // ✅ 具体的なリポジトリ名
    output_port: Arc<O>,
}

impl ApproveJournalEntryInteractor {
    async fn execute(&self, request: Request) -> Result<()> {
        // 1. load() で集約を復元
        let mut journal = self.journal_entry_repository
            .load(&id).await?
            .ok_or(NotFound)?;
        
        // 2. ビジネスロジック実行
        journal.approve(params)?;
        
        // 3. save() で永続化
        self.journal_entry_repository
            .save(&journal).await?;
        
        Ok(())
    }
}
```

### 新規集約の作成

```rust
pub struct RegisterJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    journal_entry_repository: Arc<R>,  // ✅ 具体的なリポジトリ名
    output_port: Arc<O>,
}

impl RegisterJournalEntryInteractor {
    async fn execute(&self, request: Request) -> Result<()> {
        // 1. 新規集約を作成
        let journal = JournalEntry::new(params)?;
        
        // 2. save() で永続化
        self.journal_entry_repository
            .save(&journal).await?;
        
        Ok(())
    }
}
```

## 誤ったパターン（修正前）

```rust
// ❌ 間違い
pub struct ApproveJournalEntryInteractor<R: JournalEntryRepository, O: JournalEntryOutputPort> {
    event_repository: Arc<R>,  // ❌ 抽象的すぎる名前
    output_port: Arc<O>,
}

impl ApproveJournalEntryInteractor {
    async fn execute(&self, request: Request) -> Result<()> {
        // ❌ 手動でイベントを取得
        let events = self.event_repository.get_events(&id).await?;
        
        // ❌ 手動で集約を復元
        let mut journal = JournalEntry::from_events(events)?;
        
        journal.approve(params)?;
        
        // ❌ 手動でイベントを取得して保存
        let events = journal.events();
        self.event_repository.append_events(&id, events).await?;
        
        Ok(())
    }
}
```

## 命名規則

### リポジトリフィールド名

| 集約 | フィールド名 |
|------|------------|
| `JournalEntry` | `journal_entry_repository` |
| `Account` | `account_repository` |
| `Company` | `company_repository` |
| `Lease` | `lease_repository` |

### 一般的なパターン

```
{集約名の小文字スネークケース}_repository
```

例:
- `JournalEntry` → `journal_entry_repository`
- `FixedAsset` → `fixed_asset_repository`
- `LeaseContract` → `lease_contract_repository`

## メリット

1. **明確な責務**
   - フィールド名から扱う集約が明確
   - `event_repository` では何のイベントか不明

2. **型安全性**
   - `JournalEntryRepository` トレイトを実装したリポジトリのみ受け入れ
   - 間違ったリポジトリを渡すとコンパイルエラー

3. **一貫性**
   - すべてのインタラクタで同じ命名規則
   - コードレビューが容易

4. **保守性**
   - リファクタリング時に影響範囲が明確
   - IDE のリネーム機能が効果的に使える

## TODO

以下の集約メソッドを実装する必要があります：

### JournalEntry 集約

1. `update_lines(lines: Vec<JournalEntryLine>) -> DomainResult<()>`
   - 明細を更新
   - DraftUpdatedイベントを生成

2. `update_transaction_date(date: TransactionDate) -> DomainResult<()>`
   - 取引日付を更新
   - DraftUpdatedイベントを生成

3. `update_voucher_number(voucher: VoucherNumber) -> DomainResult<()>`
   - 証憑番号を更新
   - DraftUpdatedイベントを生成

4. `delete(user_id: UserId) -> DomainResult<()>`
   - 下書きを削除
   - Deletedイベントを生成

これらのメソッドが実装されれば、`UpdateDraftJournalEntryInteractor` と `DeleteDraftJournalEntryInteractor` が完全に動作します。
