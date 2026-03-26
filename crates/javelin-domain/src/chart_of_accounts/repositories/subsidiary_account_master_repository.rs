// SubsidiaryAccountMaster Repository - 補助科目マスタリポジトリ

use crate::{chart_of_accounts::entities::SubsidiaryAccountMaster, common::RepositoryBase};

/// 補助科目マスタリポジトリトレイト
///
/// SubsidiaryAccountMaster集約を扱う専用リポジトリ。
/// RepositoryBaseを継承し、集約のロード/保存機能を提供。
///
/// # 責務
/// - SubsidiaryAccountMaster集約の保存
/// - SubsidiaryAccountMaster集約のロード
///
/// # インフラ層での実装
/// マスタデータのため、EventStoreではなくLMDBに直接保存
/// （イベントソーシング不要）
#[allow(async_fn_in_trait)]
pub trait SubsidiaryAccountMasterRepository:
    RepositoryBase<SubsidiaryAccountMaster> + Send + Sync
{
    // 必要に応じて補助科目マスタ固有のメソッドを追加可能
}
