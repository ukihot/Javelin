// ApplicationSettings - アプリケーション設定操作リクエスト

/// アプリケーション設定取得リクエスト
#[derive(Debug, Clone)]
pub struct LoadApplicationSettingsRequest;

/// アプリケーション設定更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateApplicationSettingsRequest {
    pub default_company_code: Option<String>,
    pub language: String,
    pub decimal_places: u8,
    pub date_format: String,
    pub fiscal_year_start_month: u8,
    pub closing_day: u8,
    pub auto_backup_enabled: bool,
    pub backup_retention_days: u32,
}
