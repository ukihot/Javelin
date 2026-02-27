// ApplicationSettingsRepository - アプリケーション設定リポジトリトレイト

use crate::{error::DomainResult, masters::ApplicationSettings};

/// アプリケーション設定リポジトリトレイト
///
/// CQRS原則: Repositoryはイベント永続化のみを担当
/// 読み取りはQueryServiceを使用すること
#[allow(async_fn_in_trait)]
pub trait ApplicationSettingsRepository: Send + Sync {
    /// アプリケーション設定を保存
    async fn save(&self, settings: &ApplicationSettings) -> DomainResult<()>;
}
