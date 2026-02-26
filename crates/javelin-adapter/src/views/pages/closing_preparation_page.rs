// ClosingPreparationPage - 締準備実行履歴画面
// 責務: 締準備処理の実行履歴表示

use ratatui::Frame;

use crate::views::layouts::templates::{BatchHistoryItem, BatchHistoryTemplate};

pub struct ClosingPreparationPage {
    template: BatchHistoryTemplate,
}

impl ClosingPreparationPage {
    pub fn new() -> Self {
        let template = BatchHistoryTemplate::new("締準備処理 - 実行履歴");
        Self { template }
    }

    pub fn set_history(&mut self, history: Vec<BatchHistoryItem>) {
        self.template.set_history(history);
    }

    pub fn set_loading(&mut self) {
        self.template.set_loading();
    }

    pub fn set_error(&mut self, error: String) {
        self.template.set_error(error);
    }

    pub fn add_info(&mut self, message: impl Into<String>) {
        self.template.add_info(message);
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.template.add_error(message);
    }

    pub fn select_next(&mut self) {
        self.template.select_next();
    }

    pub fn select_previous(&mut self) {
        self.template.select_previous();
    }

    pub fn tick(&mut self) {
        self.template.tick();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Default for ClosingPreparationPage {
    fn default() -> Self {
        Self::new()
    }
}
