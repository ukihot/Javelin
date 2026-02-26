// ApplicationSettingsPresenter実装
// アプリケーション設定の出力を整形してビューに渡す

use javelin_application::{
    dtos::response::LoadApplicationSettingsResponse, output_ports::ApplicationSettingsOutputPort,
};
use tokio::sync::mpsc;

/// アプリケーション設定ViewModel
#[derive(Debug, Clone)]
pub struct ApplicationSettingsViewModel {
    pub default_company_code: Option<String>,
    pub language: String,
    pub language_label: String,
    pub decimal_places: u8,
    pub date_format: String,
    pub fiscal_year_start_month: u8,
    pub fiscal_year_start_month_label: String,
    pub closing_day: u8,
    pub auto_backup_enabled: bool,
    pub auto_backup_label: String,
    pub backup_retention_days: u32,
}

/// アプリケーション設定Presenter
#[derive(Clone)]
pub struct ApplicationSettingsPresenter {
    sender: mpsc::UnboundedSender<ApplicationSettingsViewModel>,
}

impl ApplicationSettingsPresenter {
    pub fn new(sender: mpsc::UnboundedSender<ApplicationSettingsViewModel>) -> Self {
        Self { sender }
    }

    /// チャネルを作成
    pub fn create_channel() -> (
        mpsc::UnboundedSender<ApplicationSettingsViewModel>,
        mpsc::UnboundedReceiver<ApplicationSettingsViewModel>,
    ) {
        mpsc::unbounded_channel()
    }

    fn format_language_label(language: &str) -> String {
        match language {
            "ja" => "日本語",
            "en" => "English",
            _ => language,
        }
        .to_string()
    }

    fn format_month_label(month: u8) -> String {
        format!("{}月", month)
    }

    fn format_backup_label(enabled: bool) -> String {
        if enabled {
            "有効".to_string()
        } else {
            "無効".to_string()
        }
    }
}

#[allow(async_fn_in_trait)]
impl ApplicationSettingsOutputPort for ApplicationSettingsPresenter {
    async fn present_application_settings(&self, response: &LoadApplicationSettingsResponse) {
        let view_model = ApplicationSettingsViewModel {
            default_company_code: response.user_options.default_company_code.clone(),
            language: response.user_options.language.clone(),
            language_label: Self::format_language_label(&response.user_options.language),
            decimal_places: response.user_options.decimal_places,
            date_format: response.user_options.date_format.clone(),
            fiscal_year_start_month: response.system_settings.fiscal_year_start_month,
            fiscal_year_start_month_label: Self::format_month_label(
                response.system_settings.fiscal_year_start_month,
            ),
            closing_day: response.system_settings.closing_day,
            auto_backup_enabled: response.system_settings.auto_backup_enabled,
            auto_backup_label: Self::format_backup_label(
                response.system_settings.auto_backup_enabled,
            ),
            backup_retention_days: response.system_settings.backup_retention_days,
        };

        let _ = self.sender.send(view_model);
    }

    async fn notify_error(&self, error_message: String) {
        eprintln!("[ApplicationSettings Error] {}", error_message);
    }
}
