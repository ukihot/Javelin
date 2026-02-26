// ApplicationSettingsPage - アプリケーション設定画面のビューコンポーネント

use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::presenter::ApplicationSettingsViewModel;

#[derive(Debug, Clone, PartialEq)]
enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

pub struct ApplicationSettingsPage {
    view_model: Option<ApplicationSettingsViewModel>,
    loading_state: LoadingState,
}

impl ApplicationSettingsPage {
    pub fn new() -> Self {
        Self { view_model: None, loading_state: LoadingState::Loading }
    }

    pub fn set_data(&mut self, view_model: ApplicationSettingsViewModel) {
        self.view_model = Some(view_model);
        self.loading_state = LoadingState::Loaded;
    }

    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    pub fn set_error(&mut self, error: String) {
        self.loading_state = LoadingState::Error(error);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if self.loading_state == LoadingState::Loading {
            let loading = Paragraph::new("読み込み中...")
                .block(Block::default().borders(Borders::ALL).title("アプリケーション設定"));
            frame.render_widget(loading, area);
            return;
        }

        if let LoadingState::Error(error) = &self.loading_state {
            let error_widget = Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("エラー"));
            frame.render_widget(error_widget, area);
            return;
        }

        if let Some(vm) = &self.view_model {
            let text = format!(
                "デフォルト会社コード: {}\n\
                 言語: {}\n\
                 小数点以下桁数: {}\n\
                 日付フォーマット: {}\n\
                 会計年度開始月: {}\n\
                 締日: {}日\n\
                 自動バックアップ: {}\n\
                 バックアップ保持日数: {}日\n\n\
                 [Esc] 戻る",
                vm.default_company_code.as_deref().unwrap_or("未設定"),
                vm.language_label,
                vm.decimal_places,
                vm.date_format,
                vm.fiscal_year_start_month_label,
                vm.closing_day,
                vm.auto_backup_label,
                vm.backup_retention_days
            );

            let widget = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("アプリケーション設定"));

            frame.render_widget(widget, area);
        }
    }
}

impl Default for ApplicationSettingsPage {
    fn default() -> Self {
        Self::new()
    }
}
