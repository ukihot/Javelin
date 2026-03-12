// ChartOfAccounts Repository - 勘定科目表リポジトリ

use super::entities::{AccountMaster, SubsidiaryAccountMaster};
use crate::common::RepositoryBase;

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
