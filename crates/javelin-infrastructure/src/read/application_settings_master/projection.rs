// ApplicationSettingsMasterProjection - アプリケーション設定マスタProjection

use std::sync::Arc;

use javelin_domain::masters::{
    ApplicationSettings, BackupRetentionDays, ClosingDay, CompanyCode, DateFormat, DecimalPlaces,
    FiscalYearStartMonth, Language,
};
use serde::{Deserialize, Serialize};

use crate::read::infrastructure::db::ProjectionDb;

/// アプリケーション設定マスタProjection用の保存データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredApplicationSettings {
    pub default_company_code: Option<String>,
    pub language: String,
    pub decimal_places: u8,
    pub date_format: String,
    pub fiscal_year_start_month: u8,
    pub closing_day: u8,
    pub auto_backup_enabled: bool,
    pub backup_retention_days: u32,
}

/// アプリケーション設定マスタProjection
pub struct ApplicationSettingsMasterProjection {
    projection_db: Arc<ProjectionDb>,
}

impl ApplicationSettingsMasterProjection {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// アプリケーション設定マスタを保存
    pub async fn save(
        &self,
        settings: &ApplicationSettings,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stored = StoredApplicationSettings {
            default_company_code: settings.default_company_code().map(|c| c.value().to_string()),
            language: settings.language().value().to_string(),
            decimal_places: settings.decimal_places().value(),
            date_format: settings.date_format().value().to_string(),
            fiscal_year_start_month: settings.fiscal_year_start_month().value(),
            closing_day: settings.closing_day().value(),
            auto_backup_enabled: settings.auto_backup_enabled(),
            backup_retention_days: settings.backup_retention_days().value(),
        };

        let key = "application_settings:default".to_string();
        let value = serde_json::to_vec(&stored)?;

        self.projection_db
            .update_projection_batch("application_settings", 1, vec![(key, value)], 0)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    /// アプリケーション設定マスタを取得
    pub async fn get(
        &self,
    ) -> Result<Option<ApplicationSettings>, Box<dyn std::error::Error + Send + Sync>> {
        let key = "application_settings:default";
        let value = self.projection_db.get_projection(key).await.map_err(|e| {
            Box::new(std::io::Error::other(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        match value {
            Some(v) => {
                let stored: StoredApplicationSettings = serde_json::from_slice(&v)?;
                let settings = Self::from_stored(&stored)?;
                Ok(Some(settings))
            }
            None => Ok(None),
        }
    }

    fn from_stored(
        stored: &StoredApplicationSettings,
    ) -> Result<ApplicationSettings, Box<dyn std::error::Error + Send + Sync>> {
        let default_company_code =
            stored.default_company_code.as_ref().map(CompanyCode::new).transpose().map_err(
                |e: javelin_domain::error::DomainError| {
                    Box::new(e) as Box<dyn std::error::Error + Send + Sync>
                },
            )?;

        let language = Language::new(&stored.language)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let decimal_places = DecimalPlaces::new(stored.decimal_places)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let date_format = DateFormat::new(&stored.date_format)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let fiscal_year_start_month = FiscalYearStartMonth::new(stored.fiscal_year_start_month)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let closing_day = ClosingDay::new(stored.closing_day)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let backup_retention_days = BackupRetentionDays::new(stored.backup_retention_days)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(ApplicationSettings::new(
            default_company_code,
            language,
            decimal_places,
            date_format,
            fiscal_year_start_month,
            closing_day,
            stored.auto_backup_enabled,
            backup_retention_days,
        ))
    }
}
