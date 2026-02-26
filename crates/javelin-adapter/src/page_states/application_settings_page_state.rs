// ApplicationSettingsPageState - アプリケーション設定画面の状態

use std::sync::Arc;

use javelin_application::dtos::request::LoadApplicationSettingsRequest;
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::{ApplicationSettingsPresenter, ApplicationSettingsViewModel},
    views::pages::ApplicationSettingsPage,
};

/// アプリケーション設定画面の状態
pub struct ApplicationSettingsPageState {
    /// Unique identifier for presenter registration
    id: Uuid,
    /// Reference to presenter registry
    registry: Arc<PresenterRegistry>,
    /// The page view
    page: ApplicationSettingsPage,
    /// Data receiver channel
    data_rx: tokio::sync::mpsc::UnboundedReceiver<ApplicationSettingsViewModel>,
    /// ローディング中かどうか
    is_loading: bool,
    /// データロード済みフラグ
    data_loaded: bool,
}

impl ApplicationSettingsPageState {
    pub fn new(registry: Arc<PresenterRegistry>) -> Self {
        let id = Uuid::new_v4();

        // Create channel and presenter
        let (tx, rx) = ApplicationSettingsPresenter::create_channel();
        let presenter = Arc::new(ApplicationSettingsPresenter::new(tx));

        // Register presenter
        registry.register_application_settings_presenter(id, presenter);

        Self {
            id,
            registry,
            page: ApplicationSettingsPage::new(),
            data_rx: rx,
            is_loading: true,
            data_loaded: false,
        }
    }

    /// Poll for data updates from channel
    fn poll_data(&mut self) {
        while let Ok(view_model) = self.data_rx.try_recv() {
            self.page.set_data(view_model);
            self.is_loading = false;
        }
    }
}

impl PageState for ApplicationSettingsPageState {
    fn route(&self) -> Route {
        Route::ApplicationSettings
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        // 初回ロード
        if !self.data_loaded {
            self.data_loaded = true;
            let controller = Arc::clone(&controllers.application_settings);
            let page_id = self.id;

            tokio::spawn(async move {
                let request = LoadApplicationSettingsRequest;
                let _ = controller.handle_load_application_settings(page_id, request).await;
            });
        }

        loop {
            // Poll for data updates
            self.poll_data();

            // Render
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events
            if crossterm::event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let crossterm::event::Event::Key(key) =
                    crossterm::event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                use crossterm::event::{KeyCode, KeyEventKind};

                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Esc {
                    return Ok(NavAction::Back);
                }
            }
        }
    }
}

impl Drop for ApplicationSettingsPageState {
    fn drop(&mut self) {
        // Unregister presenter when page is destroyed
        self.registry.unregister_application_settings_presenter(self.id);
    }
}
