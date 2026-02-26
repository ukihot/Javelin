// SettingsTemplate - 設定画面の汎用テンプレート
// 責務: キー・バリュー形式の設定表示の共通レイアウト

use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

/// 設定画面のデータ項目
pub trait SettingsItem {
    /// 設定項目のキー・バリューペアを返す
    fn to_key_value_pairs(&self) -> Vec<(String, String)>;
}

/// 設定画面の状態
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

/// 設定画面のテンプレート
pub struct SettingsTemplate<T: SettingsItem> {
    /// タイトル
    title: String,
    /// 設定データ
    data: Option<T>,
    /// ローディング状態
    loading_state: LoadingState,
    /// フッターメッセージ
    footer: String,
}

impl<T: SettingsItem> SettingsTemplate<T> {
    /// 新しいテンプレートを作成
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            data: None,
            loading_state: LoadingState::Loading,
            footer: "[Esc] 戻る".to_string(),
        }
    }

    /// フッターメッセージを設定
    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = footer.into();
        self
    }

    /// データを設定
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
        self.loading_state = LoadingState::Loaded;
    }

    /// ローディング状態を設定
    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    /// エラー状態を設定
    pub fn set_error(&mut self, error: String) {
        self.loading_state = LoadingState::Error(error);
    }

    /// 画面を描画
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // ローディング中
        if self.loading_state == LoadingState::Loading {
            let loading = Paragraph::new("読み込み中...")
                .block(Block::default().borders(Borders::ALL).title(self.title.as_str()));
            frame.render_widget(loading, area);
            return;
        }

        // エラー表示
        if let LoadingState::Error(error) = &self.loading_state {
            let error_widget = Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("エラー"));
            frame.render_widget(error_widget, area);
            return;
        }

        // データ表示
        if let Some(ref data) = self.data {
            let pairs = data.to_key_value_pairs();
            let mut text = String::new();

            for (key, value) in pairs {
                text.push_str(&format!("{}: {}\n", key, value));
            }

            text.push_str(&format!("\n{}", self.footer));

            let widget = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title(self.title.as_str()));

            frame.render_widget(widget, area);
        }
    }
}
