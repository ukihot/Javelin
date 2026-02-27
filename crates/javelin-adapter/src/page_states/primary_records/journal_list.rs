// JournalListPageState - 仕訳検索・一覧画面
// Route::JournalListに対応

use std::sync::Arc;

use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    page_states::StubPageState,
};

/// 仕訳検索・一覧画面のPageState
///
/// 現在はStubPageStateを使用した仮実装
pub struct JournalListPageState {
    stub: StubPageState,
    _presenter_registry: Arc<PresenterRegistry>,
}

impl JournalListPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self {
            stub: StubPageState::new(
                Route::JournalList,
                "Journal Entry Search / List",
                "仕訳検索・一覧画面",
            ),
            _presenter_registry: presenter_registry,
        }
    }
}

impl PageState for JournalListPageState {
    fn route(&self) -> Route {
        self.stub.route()
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        self.stub.run(terminal, controllers)
    }
}
