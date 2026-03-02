// PreparationResultPageState - 締準備処理結果画面
// 責務: 締準備処理の結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
};

/// 締準備処理結果画面
pub struct PreparationResultPageState {}

impl PreparationResultPageState {
    pub fn new() -> Self {
        Self {}
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
                .split(area);

        // タイトル
        let title = Paragraph::new("締準備処理結果")
            .block(Block::default().borders(Borders::ALL).title("処理結果"));
        frame.render_widget(title, chunks[0]);

        // 結果テーブル
        let header = Row::new(vec!["項目", "結果", "詳細"])
            .style(Style::default().add_modifier(Modifier::BOLD));

        let rows = vec![
            Row::new(vec!["未承認仕訳チェック", "OK", "0件"]),
            Row::new(vec!["残高整合性検証", "OK", "差異なし"]),
            Row::new(vec!["期間ロック確認", "OK", "ロック可能"]),
            Row::new(vec!["必須項目チェック", "OK", "すべて入力済み"]),
        ];

        let table =
            Table::new(rows, [Constraint::Length(20), Constraint::Length(10), Constraint::Min(30)])
                .header(header)
                .block(Block::default().borders(Borders::ALL).title("検証結果"));

        frame.render_widget(table, chunks[1]);

        // フッター
        let footer = Paragraph::new("[Esc] 戻る")
            .block(Block::default().borders(Borders::ALL).title("操作"));
        frame.render_widget(footer, chunks[2]);
    }
}

impl PageState for PreparationResultPageState {
    fn route(&self) -> Route {
        Route::ClosingPreparationResult
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

impl Default for PreparationResultPageState {
    fn default() -> Self {
        Self::new()
    }
}
