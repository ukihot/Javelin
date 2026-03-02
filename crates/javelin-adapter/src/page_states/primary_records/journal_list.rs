// JournalListPageState - 仕訳検索・一覧画面
// Route::JournalListに対応

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::SearchPresenter,
    views::pages::JournalListPage,
};

/// 仕訳検索・一覧画面のPageState
pub struct JournalListPageState {
    page_id: Uuid,
    page: JournalListPage,
    presenter_registry: Arc<PresenterRegistry>,
}

impl JournalListPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();

        // SearchPresenterのチャネルを作成
        let (presenter, channels) = SearchPresenter::create_channels();

        // PresenterRegistryに登録
        presenter_registry.register_search_presenter(page_id, Arc::new(presenter));

        // Pageを作成（チャネルを渡す）
        let page = JournalListPage::new(page_id, channels);

        Self { page_id, page, presenter_registry }
    }
}

impl Drop for JournalListPageState {
    fn drop(&mut self) {
        // PresenterRegistryから登録解除
        self.presenter_registry.unregister_search_presenter(self.page_id);
    }
}

impl PageState for JournalListPageState {
    fn route(&self) -> Route {
        Route::JournalList
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // Tick animation
            self.page.tick();

            // Poll for async data updates
            self.page.poll_search_results();

            // Check if search needs to be triggered
            if self.page.has_pending_search() {
                self.page.clear_pending_search();

                let controller = controllers.search.clone();
                let page_id = self.page_id;

                tokio::spawn(async move {
                    use javelin_application::dtos::request::SearchCriteriaDto;

                    // 全件検索（フィルタなし）
                    let criteria = SearchCriteriaDto {
                        from_date: None,
                        to_date: None,
                        account_code: None,
                        min_amount: None,
                        max_amount: None,
                        description: None,
                        debit_credit: None,
                        limit: Some(100),
                        offset: None,
                    };

                    let _ = controller.handle_search(page_id, criteria).await;
                });
            }

            // Render the page
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events with timeout for animation updates
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Char('s') => {
                        // 検索実行
                        self.page.trigger_search();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.page.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.page.select_previous();
                    }
                    _ => {}
                }
            }
        }
    }
}
