// ApplicationSettingsRepository - アプリケーション設定リポジトリトレイト

use crate::{error::DomainResult, masters::ApplicationSettings};

/// アプリケーション設定リポジトリトレイト
///
/// ApplicationSettings（アプリケーション全設定）を扱うリポジトリ。
/// イベントソーシングではなく、CRUD操作で扱われる。
#[allow(async_fn_in_trait)]
pub trait ApplicationSettingsRepository: Send + Sync {
    /// アプリケーション設定を取得
    async fn find(&self) -> DomainResult<Option<ApplicationSettings>>;

    /// アプリケーション設定を保存
    async fn save(&self, settings: &ApplicationSettings) -> DomainResult<()>;
}
