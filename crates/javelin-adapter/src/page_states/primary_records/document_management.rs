// DocumentManagementPageState - 証憑管理画面
// 責務: 証憑ファイルの管理

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// 証憑項目ViewModel
#[derive(Debug, Clone)]
pub struct DocumentItemViewModel {
    pub document_id: String,
    pub voucher_number: String,
    pub document_type: String,
    pub file_name: String,
    pub upload_date: String,
    pub file_size: String,
}

impl MasterListItem for DocumentItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["証憑ID", "伝票番号", "種類", "ファイル名", "登録日", "サイズ"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Min(25),
            Constraint::Length(12),
            Constraint::Length(10),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.document_id.clone(),
            self.voucher_number.clone(),
            self.document_type.clone(),
            self.file_name.clone(),
            self.upload_date.clone(),
            self.file_size.clone(),
        ]
    }
}

/// 証憑管理画面
pub struct DocumentManagementPageState {
    template: MasterListTemplate<DocumentItemViewModel>,
}

impl DocumentManagementPageState {
    pub fn new() -> Self {
        let template = MasterListTemplate::new("証憑管理");
        Self { template }
    }

    fn load_data(&mut self, _controllers: &Controllers) {
        // TODO: 実際のコントローラを使ってデータを取得
        self.template.set_data(vec![], 0, 0);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for DocumentManagementPageState {
    fn route(&self) -> Route {
        Route::DocumentManagement
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
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
                        // 証憑表示（将来実装）
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for DocumentManagementPageState {
    fn default() -> Self {
        Self::new()
    }
}
