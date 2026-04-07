// CompanyMaster Repository - 会社マスタリポジトリ（後方互換用エイリアス）

use crate::{common::RepositoryBase, company::entities::Organization};

/// 会社マスタリポジトリトレイト（後方互換エイリアス）
///
/// OrganizationRepositoryへの移行を推奨。
#[allow(async_fn_in_trait)]
pub trait CompanyMasterRepository: RepositoryBase<Organization> + Send + Sync {
    // OrganizationRepositoryに統合予定
}
