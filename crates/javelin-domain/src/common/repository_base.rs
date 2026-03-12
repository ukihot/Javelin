// RepositoryBase - 集約の永続化における基底リポジトリトレイト

use crate::error::DomainResult;

/// RepositoryBase - 全リポジトリの基底トレイト
///
/// CQRS + Event Sourcingアーキテクチャにおいて、リポジトリは
/// 集約のロードと保存のみを担当する。
///
/// # 責務
/// - 集約の保存（内部でイベントを生成・永続化）
/// - 集約のロード（内部でイベントから復元）
///
/// # 禁止事項
/// - イベントの直接操作（インフラ層の責務）
/// - 検索機能（QueryServiceの責務）
#[allow(async_fn_in_trait)]
pub trait RepositoryBase<T>: Send + Sync {
    /// 集約を保存
    ///
    /// 集約の現在の状態を永続化する。
    /// インフラ層の実装では、集約からイベントを生成し、EventStoreに保存する。
    ///
    /// # Arguments
    /// * `aggregate` - 保存する集約
    ///
    /// # Returns
    /// 成功時は`Ok(())`、失敗時はエラー
    async fn save(&self, aggregate: &T) -> DomainResult<()>;

    /// 集約をロード
    ///
    /// 指定されたIDの集約を復元する。
    /// インフラ層の実装では、EventStoreからイベントを取得し、集約を復元する。
    ///
    /// # Arguments
    /// * `id` - 集約ID
    ///
    /// # Returns
    /// 集約が存在する場合は`Some(T)`、存在しない場合は`None`
    async fn load(&self, id: &str) -> DomainResult<Option<T>>;
}
