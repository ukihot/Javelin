// AssetDetailPageState - 資産詳細画面
// 責務: 固定資産の詳細情報表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
};

/// 資産詳細画面
pub struct AssetDetailPageState {
    asset_id: String,
}

impl AssetDetailPageState {
    pub fn new() -> Self {
        Self { asset_id: "ASSET-001".to_string() }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(area);

        let content = Paragraph::new(format!(
            "資産詳細画面\n\n資産ID: {}\n\n取得原価、減価償却累計額、帳簿価額などの詳細情報を表示",
            self.asset_id
        ))
        .block(Block::default().borders(Borders::ALL).title("資産詳細"));
        frame.render_widget(content, chunks[0]);

        let footer = Paragraph::new("[Esc] 戻る")
            .block(Block::default().borders(Borders::ALL).title("操作"));
        frame.render_widget(footer, chunks[1]);
    }
}

impl PageState for AssetDetailPageState {
    fn route(&self) -> Route {
        Route::AssetDetail
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
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

                if key.code == KeyCode::Esc {
                    return Ok(NavAction::Back);
                }
            }
        }
    }
}

impl Default for AssetDetailPageState {
    fn default() -> Self {
        Self::new()
    }
}
