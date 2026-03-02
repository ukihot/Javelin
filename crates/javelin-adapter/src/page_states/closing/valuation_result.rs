// ValuationResultPageState - 評価結果一覧画面
// 責務: IFRS評価結果の一覧表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 評価結果項目ViewModel
#[derive(Debug, Clone)]
pub struct ValuationResultItemViewModel {
    pub asset_code: String,
    pub asset_name: String,
    pub book_value: String,
    pub fair_value: String,
    pub difference: String,
    pub status: String,
}

impl MasterListItem for ValuationResultItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["資産コード", "資産名", "帳簿価額", "公正価値", "差額", "状態"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(10),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.asset_code.clone(),
            self.asset_name.clone(),
            self.book_value.clone(),
            self.fair_value.clone(),
            self.difference.clone(),
            self.status.clone(),
        ]
    }
}

/// 評価結果一覧画面
pub struct ValuationResultPageState {
    template: MasterListTemplate<ValuationResultItemViewModel>,
}

impl ValuationResultPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("IFRS評価結果一覧");
        Self { template }
    }

    fn load_data(&mut self, controllers: &Controllers) {
        let apply_ifrs_valuation = controllers.apply_ifrs_valuation.clone();

        // 非同期で評価結果を取得
        tokio::spawn(async move {
            // 評価結果の取得処理
            // 将来的にはプレゼンタ経由でデータを受信
            let _ = apply_ifrs_valuation;
        });

        // 現在は空のデータを表示（将来的にはプレゼンタ経由で受信）
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for ValuationResultPageState {
    fn route(&self) -> Route {
        Route::ValuationResult
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        // 初回データロード
        self.load_data(controllers);

        loop {
            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

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
                    KeyCode::Enter => {
                        // 詳細画面への遷移（将来実装）
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for ValuationResultPageState {
    fn default() -> Self {
        Self::new()
    }
}
