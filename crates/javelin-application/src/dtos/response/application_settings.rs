// ApplicationSettings - アプリケーション設定操作レスポンス

use serde::{Deserialize, Serialize};

/// アプリケーション設定取得レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadApplicationSettingsResponse {
    pub user_options: UserOptionsDto,
    pub system_settings: SystemSettingsDto,
}

/// ユーザ設定DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptionsDto {
    pub default_company_code: Option<String>,
    pub language: String,
    pub decimal_places: u8,
    pub date_format: String,
}

/// システム設定DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettingsDto {
    pub fiscal_year_start_month: u8,
    pub closing_day: u8,
    pub auto_backup_enabled: bool,
    pub backup_retention_days: u32,
}

/// アプリケーション設定更新レスポンス
#[derive(Debug, Clone)]
pub struct UpdateApplicationSettingsResponse {
    pub message: String,
}
