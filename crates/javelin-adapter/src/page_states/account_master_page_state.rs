// AccountMasterPageState - 勘定科目マスタ画面の状態

use std::sync::Arc;

use javelin_application::dtos::request::LoadAccountMasterRequest;
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::{AccountMasterPresenter, AccountMasterViewModel},
    views::pages::AccountMasterPage,
};

/// 勘定科目マスタ画面の状態
pub struct AccountMasterPageState {
    /// Unique identifier for presenter registration
    id: Uuid,
    /// Reference to presenter registry
    registry: Arc<PresenterRegistry>,
    /// The page view
    page: AccountMasterPage,
    /// Data receiver channel
    data_rx: tokio::sync::mpsc::UnboundedReceiver<AccountMasterViewModel>,
    /// 現在のページ番号（0始まり）
    current_page: usize,
    /// 1ページあたりの表示件数
    items_per_page: usize,
    /// 選択中の行インデックス
    selected_index: usize,
    /// ローディング中かどうか
    is_loading: bool,
    /// データロード済みフラグ
    data_loaded: bool,
}

impl AccountMasterPageState {
    pub fn new(registry: Arc<PresenterRegistry>) -> Self {
        let id = Uuid::new_v4();

        // Create channel and presenter
        let (tx, rx) = AccountMasterPresenter::create_channel();
        let presenter = Arc::new(AccountMasterPresenter::new(tx));

        // Register presenter
        registry.register_account_master_presenter(id, presenter);

        Self {
            id,
            registry,
            page: AccountMasterPage::new(),
            data_rx: rx,
            current_page: 0,
            items_per_page: 10,
            selected_index: 0,
            is_loading: true,
            data_loaded: false,
        }
    }

    /// Poll for data updates from channel
    fn poll_data(&mut self) {
        while let Ok(view_model) = self.data_rx.try_recv() {
            self.page.set_data(view_model.accounts, self.current_page, self.selected_index);
            self.is_loading = false;
        }
    }

    /// 総ページ数を取得
    fn total_pages(&self) -> usize {
        let total_items = self.page.total_items();
        if total_items == 0 {
            1
        } else {
            total_items.div_ceil(self.items_per_page)
        }
    }

    /// 次のページへ移動
    fn next_page(&mut self) {
        if self.current_page + 1 < self.total_pages() {
            self.current_page += 1;
            self.selected_index = 0;
        }
    }

    /// 前のページへ移動
    fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.selected_index = 0;
        }
    }

    /// カーソルを上に移動
    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// カーソルを下に移動
    fn move_down(&mut self) {
        let page_items_len = self.page.current_page_items_len();
        if page_items_len > 0 && self.selected_index < page_items_len - 1 {
            self.selected_index += 1;
        }
    }
}

impl PageState for AccountMasterPageState {
    fn route(&self) -> Route {
        Route::AccountMaster
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        // 初回ロード
        if !self.data_loaded {
            self.data_loaded = true;
            let controller = Arc::clone(&controllers.account_master);
            let page_id = self.id;

            tokio::spawn(async move {
                let request = LoadAccountMasterRequest { filter: None, active_only: true };
                let _ = controller.handle_load_account_master(page_id, request).await;
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

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                    KeyCode::Left | KeyCode::Char('h') => self.prev_page(),
                    KeyCode::Right | KeyCode::Char('l') => self.next_page(),
                    _ => {}
                }
            }
        }
    }
}

impl Drop for AccountMasterPageState {
    fn drop(&mut self) {
        // Unregister presenter when page is destroyed
        self.registry.unregister_account_master_presenter(self.id);
    }
}
