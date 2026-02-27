// アプリケーション設定マスタQueryService trait

use javelin_domain::masters::ApplicationSettings;

use crate::error::ApplicationResult;

/// アプリケーション設定マスタQueryService
///
/// CQRS原則: 読み取り専用のQueryService
/// ProjectionDBからアプリケーション設定マスタデータを取得する
#[allow(async_fn_in_trait)]
pub trait ApplicationSettingsMasterQueryService: Send + Sync {
    /// アプリケーション設定マスタを取得
    async fn get(&self) -> ApplicationResult<Option<ApplicationSettings>>;
}
