// JournalDetailPageState - 仕訳詳細画面のページ状態管理
// Presenterのライフサイクルを管理し、画面の状態を保持する

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::JournalEntryPresenter,
    views::pages::JournalDetailPage,
};

/// 仕訳詳細ページ状態
pub struct JournalDetailPageState {
    /// ページインスタンスID
    id: Uuid,
    /// Presenterレジストリ
    registry: Arc<PresenterRegistry>,
    /// 仕訳詳細ページビュー
    page: JournalDetailPage,
    /// 仕訳エントリープレゼンター
    #[allow(dead_code)]
    journal_entry_presenter: Arc<JournalEntryPresenter>,
    /// 表示する仕訳ID
    entry_id: String,
}

impl JournalDetailPageState {
    /// 新しい仕訳詳細ページ状態を作成
    ///
    /// # Arguments
    /// * `registry` - Presenterレジストリ
    /// * `entry_id` - 表示する仕訳ID
    pub fn new(registry: Arc<PresenterRegistry>, entry_id: String) -> Self {
        let id = Uuid::new_v4();

        // チャネルを作成
        let (list_tx, _list_rx) = tokio::sync::mpsc::unbounded_channel();
        let (detail_tx, detail_rx) = tokio::sync::mpsc::unbounded_channel();
        let (result_tx, _result_rx) = tokio::sync::mpsc::unbounded_channel();
        let (progress_tx, progress_rx) = tokio::sync::mpsc::unbounded_channel();

        // JournalEntryPresenterを作成
        let journal_entry_presenter =
            Arc::new(JournalEntryPresenter::new(list_tx, detail_tx, result_tx, progress_tx));

        // PresenterRegistryに登録
        registry.register_journal_entry_presenter(id, Arc::clone(&journal_entry_presenter));

        // Pageを作成
        let mut page = JournalDetailPage::new();
        page.set_detail_receiver(detail_rx);
        page.set_progress_receiver(progress_rx);

        Self { id, registry, page, journal_entry_presenter, entry_id }
    }
}

impl PageState for JournalDetailPageState {
    fn route(&self) -> Route {
        Route::JournalDetail
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        // 初回ロード
        if self.page.needs_initial_load() {
            self.page.mark_loaded();

            let controller: Arc<crate::navigation::controllers::JournalDetailControllerType> =
                Arc::clone(&controllers.journal_detail);
            let page_id = self.id;
            let entry_id = self.entry_id.clone();

            tokio::spawn(async move {
                let _ = controller.handle_get_journal_entry_detail(page_id, entry_id).await;
            });
        }

        loop {
            // 非同期結果をポーリング
            self.page.poll_detail_data();
            self.page.poll_progress_messages();

            // 描画
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // イベント処理
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
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

impl Drop for JournalDetailPageState {
    fn drop(&mut self) {
        // PresenterRegistryから登録解除
        self.registry.unregister_journal_entry_presenter(self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_returns_journal_detail() {
        let registry = Arc::new(PresenterRegistry::new());
        let state = JournalDetailPageState::new(Arc::clone(&registry), "entry-123".to_string());
        assert_eq!(state.route(), Route::JournalDetail);
    }

    #[test]
    fn test_presenter_registration() {
        let registry = Arc::new(PresenterRegistry::new());
        let count_before = registry.total_count();

        let state = JournalDetailPageState::new(Arc::clone(&registry), "entry-123".to_string());

        assert_eq!(registry.total_count(), count_before + 1);
        assert!(registry.get_journal_entry_presenter(state.id).is_some());
    }

    #[test]
    fn test_presenter_unregistration_on_drop() {
        let registry = Arc::new(PresenterRegistry::new());
        let count_before = registry.total_count();

        {
            let state = JournalDetailPageState::new(Arc::clone(&registry), "entry-123".to_string());
            let state_id = state.id;

            assert_eq!(registry.total_count(), count_before + 1);
            assert!(registry.get_journal_entry_presenter(state_id).is_some());
        }

        assert_eq!(registry.total_count(), count_before);
    }
}
