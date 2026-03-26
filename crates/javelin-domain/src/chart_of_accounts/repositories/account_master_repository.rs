// AccountMaster Repository - 勘定科目マスタリポジトリ

use crate::{chart_of_accounts::entities::AccountMaster, common::RepositoryBase};

/// 勘定科目マスタリポジトリトレイト
///
/// AccountMaster集約を扱う専用リポジトリ。
/// RepositoryBaseを継承し、集約のロード/保存機能を提供。
///
/// # 責務
/// - AccountMaster集約の保存
/// - AccountMaster集約のロード
///
/// # インフラ層での実装
/// マスタデータのため、EventStoreではなくLMDBに直接保存
/// （イベントソーシング不要）
#[allow(async_fn_in_trait)]
pub trait AccountMasterRepository: RepositoryBase<AccountMaster> + Send + Sync {
    // 必要に応じて勘定科目マスタ固有のメソッドを追加可能
}
