// ApplicationSettingsInteractor - アプリケーション設定操作のユースケース

use std::sync::Arc;

use javelin_domain::{
    masters::{
        ApplicationSettings, BackupRetentionDays, ClosingDay, CompanyCode, DateFormat,
        DecimalPlaces, FiscalYearStartMonth, Language,
    },
    repositories::ApplicationSettingsRepository,
};

use crate::error::ApplicationResult;

/// アプリケーション設定取得クエリ
#[derive(Debug, Clone)]
pub struct GetApplicationSettingsQuery;

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

/// アプリケーション設定Interactor
pub struct ApplicationSettingsInteractor<R>
where
    R: ApplicationSettingsRepository,
{
    repository: Arc<R>,
}

impl<R> ApplicationSettingsInteractor<R>
where
    R: ApplicationSettingsRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// アプリケーション設定を取得
    pub async fn get(
        &self,
        _query: GetApplicationSettingsQuery,
    ) -> ApplicationResult<ApplicationSettings> {
        self.repository
            .find()
            .await
            .map_err(|e| crate::error::ApplicationError::QueryExecutionFailed(e.to_string()))?
            .ok_or_else(|| {
                crate::error::ApplicationError::QueryExecutionFailed(
                    "アプリケーション設定が見つかりません".to_string(),
                )
            })
    }

    /// アプリケーション設定を更新
    pub async fn update(&self, request: UpdateApplicationSettingsRequest) -> ApplicationResult<()> {
        let default_company_code = request
            .default_company_code
            .map(|code| {
                CompanyCode::new(code)
                    .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))
            })
            .transpose()?;

        let language = Language::new(request.language)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let decimal_places = DecimalPlaces::new(request.decimal_places)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let date_format = DateFormat::new(request.date_format)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let fiscal_year_start_month = FiscalYearStartMonth::new(request.fiscal_year_start_month)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let closing_day = ClosingDay::new(request.closing_day)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;
        let backup_retention_days = BackupRetentionDays::new(request.backup_retention_days)
            .map_err(|e| crate::error::ApplicationError::ValidationError(e.to_string()))?;

        let settings = ApplicationSettings::new(
            default_company_code,
            language,
            decimal_places,
            date_format,
            fiscal_year_start_month,
            closing_day,
            request.auto_backup_enabled,
            backup_retention_days,
        );

        self.repository
            .save(&settings)
            .await
            .map_err(|e| crate::error::ApplicationError::UseCaseExecutionFailed(e.to_string()))
    }
}
