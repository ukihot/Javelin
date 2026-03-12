// Company Repository - 会社リポジトリ

use super::entities::CompanyMaster;
use crate::common::RepositoryBase;

/// 会社マスタリポジトリトレイト
///
/// CompanyMaster集約を扱う専用リポジトリ。
/// RepositoryBaseを継承し、集約のロード/保存機能を提供。
///
/// # 責務
/// - CompanyMaster集約の保存
/// - CompanyMaster集約のロード
///
/// # インフラ層での実装
/// マスタデータのため、EventStoreではなくLMDBに直接保存
/// （イベントソーシング不要）
#[allow(async_fn_in_trait)]
pub trait CompanyMasterRepository: RepositoryBase<CompanyMaster> + Send + Sync {
    // 必要に応じて会社マスタ固有のメソッドを追加可能
}
