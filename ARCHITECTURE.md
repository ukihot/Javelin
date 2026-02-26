# 1. アーキテクチャ総則

Javelinは以下の4層で構成される。

| 層 | 役割 | 依存方向 |
|------|------------|------------|
| Adapter層 | 入出力変換 | → Application |
| Application層 | ユースケース / Query / Projection制御 | → Domain |
| Domain層 | 業務ルール / 集約整合性保証 | 依存なし |
| Infrastructure層 | 永続化 / 外部技術実装 | → Domain |

依存関係は常に内側へ向かう。

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

# 2. Adapter層

## 2.1 目的

外部入出力を内部DTOへ変換し、ユースケースへ接続する。
業務ロジック・判断・仕様を保持しない。

---

## 2.2 構成要素

| 要素 | 役割 | 責務 | 禁止事項 |
|------------|------------|------------|------------|
| Controller | 外部入力受付 | DTO変換、InputPort呼び出し | 業務判断 |
| Presenter | 出力整形 | OutputPort実装、Viewモデル生成 | 業務判断 |
| View | 表示専用 | ratatuiによる描画 | 状態管理・ロジック保持 |

---

## 2.3 Controller仕様

| 項目 | 内容 |
|------------|------------|
| 入力元 | CLI入力 / TUIイベント |
| 出力先 | InputPort |
| 処理内容 | 外部データ → DTO変換 |
| 状態保持 | 不可 |

---

## 2.4 Presenter仕様

| 項目 | 内容 |
|------------|------------|
| 入力元 | Interactor |
| 出力先 | View |
| 処理内容 | DTO → ViewModel変換 |
| 状態保持 | 不可 |

---

## 2.5 View仕様

| 項目 | 内容 |
|------------|------------|
| 表示技術 | ratatui |
| 更新契機 | Presenter通知 |
| 業務知識 | 完全禁止 |

---

# 3. Application層

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

# 4. Domain層

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

# 5. Infrastructure層

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

# 6. 依存関係ルール

| 層 | 依存可能対象 |
|------------|------------|
| Adapter | Application |
| Application | Domain |
| Infrastructure | Domain |
| Domain | 依存禁止 |

---

# 7. 状態遷移原則

| 原則 | 内容 |
|------------|------------|
| Command入力 | Controller → InputPort |
| Command処理 | Interactor → Domain → Event生成 |
| 永続化 | RepositoryTrait → EventStore |
| Projection更新 | ProjectionBuilder |
| Query処理 | QueryService → Projection |
| 出力処理 | OutputPort ⇠ Presenter → View |

---

# 8. 禁止事項一覧

| 項目 | 禁止内容 |
|------------|------------|
| Adapter層 | 業務判断 |
| Application層 | 業務ルール定義、ドメインロジックの重複実装 |
| Domain層 | UI依存 / Query最適化 |
| RepositoryTrait | 検索処理 |
| Infrastructure層 | 業務判断 |
| 全層 | Enumの文字列直接比較（FromStr使用）、ユーティリティクラスによるDTO変換 |

---

# 8.1 アンチパターン防止

以下のアンチパターンを避けること:

| アンチパターン | 問題 | 正しい実装 |
|------------|------------|------------|
| 文字列でEnum比較 | 型安全性の喪失 | FromStrトレイト使用 |
| DTO変換ユーティリティクラス | 責務の不明確化 | DTOにTryFrom実装 |
| ドメインロジックの再実装 | 一貫性喪失 | ドメインサービス呼び出し |
| 手動での借方貸方反転 | バグ混入リスク | JournalEntryService::create_reversal_lines使用 |
| format!("{:?}", status) | 表示形式の不安定性 | status.as_str()またはDisplay使用 |

---

# 9. テスト戦略

| 層 | テスト種別 |
|------------|------------|
| Domain | 単体テスト |
| Application Command | ユースケーステスト |
| Application Query | Projection検索テスト |
| Infrastructure | 永続統合テスト |
| ProjectionBuilder | 再構築テスト |

---

# 10. 不変設計原則

| 原則 | 説明 |
|------------|------------|
| Event正本 | イベントのみが真実 |
| Projection派生 | 再生成可能 |
| CQRS分離 | Command / Query独立 |
| DTO境界 | ドメイン完全隠蔽 |
| Trait駆動 | 実装依存排除 |
| Append Only | 更新禁止 |

---

# 11. 命名規則

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

# 11. 命名規則

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
| Converter | ~~DtoConverter~~ (削除: TryFromトレイト使用) |
