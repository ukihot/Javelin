// ApplicationBuilder - アプリケーションのビルド
// 責務: 各セットアップモジュールを呼び出してApplicationを構築

use std::path::PathBuf;

use javelin_adapter::views::terminal_manager::TerminalManager;

use crate::{
    app::{Application, ApplicationConfig},
    app_error::{AppError::InitializationFailed, AppResult},
    app_setup::{setup_controllers, setup_infrastructure},
};

/// アプリケーションビルダー
pub struct ApplicationBuilder {
    data_dir: Option<PathBuf>,
    initial_route: Option<javelin_adapter::navigation::Route>,
}

impl ApplicationBuilder {
    /// 新規ビルダーを作成
    pub fn new() -> Self {
        Self { data_dir: None, initial_route: None }
    }

    /// Set initial route (used to select mode-specific top page)
    pub fn with_initial_route(mut self, route: javelin_adapter::navigation::Route) -> Self {
        self.initial_route = Some(route);
        self
    }

    /// データディレクトリを設定
    pub fn with_data_dir(mut self, path: PathBuf) -> Self {
        self.data_dir = Some(path);
        self
    }

    /// アプリケーションをビルド
    pub async fn build(self) -> AppResult<Application> {
        // データディレクトリの決定
        let data_dir = self.data_dir.unwrap_or_else(|| {
            let mut path = std::env::current_dir().expect("Failed to get current directory");
            path.push("data");
            path
        });

        println!("✓ Data directory: {}", data_dir.display());

        // インフラ層のセットアップ
        let infra = setup_infrastructure(&data_dir).await?;

        // コントローラのセットアップ
        let controller_components = setup_controllers(
            &data_dir,
            infra.event_store.clone(),
            infra.projection_db.clone(),
            infra.master_data_loader.clone(),
        )
        .await?;

        // TerminalManagerの作成
        let terminal_manager =
            TerminalManager::new().map_err(|e| InitializationFailed(Box::new(e)))?;

        // Applicationの構築
        let config = ApplicationConfig {
            controllers: controller_components.controllers,
            presenter_registry: controller_components.presenter_registry,
            terminal_manager,
            infra_error_receiver: infra.infra_error_receiver,
            initial_route: javelin_adapter::navigation::Route::Home,
        };

        let initial_route = self.initial_route.unwrap_or(javelin_adapter::navigation::Route::Home);

        let mut config = config;
        config.initial_route = initial_route;

        Ok(Application::new(config))
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
