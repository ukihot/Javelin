// FixedAssetsMenuPageState - C-01: Fixed Assets & Lease Menu

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct FixedAssetsMenuPageState {
    page: MenuPage,
}

impl FixedAssetsMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Fixed Asset List", "固定資産一覧"),
            ("Asset Registration", "資産登録実行"),
            ("Depreciation Execution", "減価償却計算実行"),
            ("Lease Contract List", "リース契約一覧"),
            ("ROU Asset List", "使用権資産台帳"),
        ];

        Self { page: MenuPage::new("C. Fixed Assets & Lease", &menu_items) }
    }
}

impl Default for FixedAssetsMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for FixedAssetsMenuPageState {
    fn route(&self) -> Route {
        Route::FixedAssetsMenu
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    KeyCode::Enter => {
                        let route = match self.page.selected_index() {
                            0 => Route::FixedAssetList,
                            1 => Route::AssetRegistration,
                            2 => Route::DepreciationExecution,
                            3 => Route::LeaseContractList,
                            4 => Route::RouAssetList,
                            _ => continue,
                        };
                        return Ok(NavAction::Go(route));
                    }
                    _ => {}
                }
            }
        }
    }
}
