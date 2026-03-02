// LeaseContractDetailPageState - リース契約詳細画面
// 責務: リース契約の詳細情報表示

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

/// リース契約詳細画面
pub struct LeaseContractDetailPageState {
    contract_id: String,
}

impl LeaseContractDetailPageState {
    pub fn new() -> Self {
        Self { contract_id: "LEASE-001".to_string() }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).split(area);

        let content = Paragraph::new(format!(
            "リース契約詳細画面\n\n契約ID: {}\n\n契約条件、支払スケジュール、使用権資産、リース負債の詳細情報を表示",
            self.contract_id
        ))
        .block(Block::default().borders(Borders::ALL).title("リース契約詳細"));
        frame.render_widget(content, chunks[0]);

        let footer = Paragraph::new("[Esc] 戻る | [S] スケジュール表示")
            .block(Block::default().borders(Borders::ALL).title("操作"));
        frame.render_widget(footer, chunks[1]);
    }
}

impl PageState for LeaseContractDetailPageState {
    fn route(&self) -> Route {
        Route::LeaseContractDetail
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

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        return Ok(NavAction::Go(Route::LeaseSchedule));
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for LeaseContractDetailPageState {
    fn default() -> Self {
        Self::new()
    }
}
