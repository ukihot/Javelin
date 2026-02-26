// ApplicationSettingsRepositoryImpl - アプリケーション設定リポジトリの実装

use std::{path::Path, sync::Arc};

use javelin_domain::{
    error::DomainResult,
    masters::{
        ApplicationSettings, BackupRetentionDays, ClosingDay, CompanyCode, DateFormat,
        DecimalPlaces, FiscalYearStartMonth, Language,
    },
    repositories::ApplicationSettingsRepository,
};
use lmdb::{Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StoredApplicationSettings {
    default_company_code: Option<String>,
    language: String,
    decimal_places: u8,
    date_format: String,
    fiscal_year_start_month: u8,
    closing_day: u8,
    auto_backup_enabled: bool,
    backup_retention_days: u32,
}

pub struct ApplicationSettingsRepositoryImpl {
    env: Arc<Environment>,
    db: Database,
}

impl ApplicationSettingsRepositoryImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        let env = Environment::new().set_max_dbs(1).set_map_size(10 * 1024 * 1024).open(path)?;

        let db = env.create_db(Some("application_settings"), DatabaseFlags::empty())?;

        let repository = Self { env: Arc::new(env), db };
        repository.initialize_defaults().await?;

        Ok(repository)
    }

    async fn initialize_defaults(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.find().await?.is_none() {
            let defaults = ApplicationSettings::new(
                Some(CompanyCode::new("0001").unwrap()),
                Language::new("ja").unwrap(),
                DecimalPlaces::new(2).unwrap(),
                DateFormat::new("YYYY-MM-DD").unwrap(),
                FiscalYearStartMonth::new(4).unwrap(),
                ClosingDay::new(31).unwrap(),
                true,
                BackupRetentionDays::new(90).unwrap(),
            );
            self.save(&defaults).await?;
        }
        Ok(())
    }

    fn to_stored(settings: &ApplicationSettings) -> StoredApplicationSettings {
        StoredApplicationSettings {
            default_company_code: settings.default_company_code().map(|c| c.value().to_string()),
            language: settings.language().value().to_string(),
            decimal_places: settings.decimal_places().value(),
            date_format: settings.date_format().value().to_string(),
            fiscal_year_start_month: settings.fiscal_year_start_month().value(),
            closing_day: settings.closing_day().value(),
            auto_backup_enabled: settings.auto_backup_enabled(),
            backup_retention_days: settings.backup_retention_days().value(),
        }
    }

    fn from_stored(stored: &StoredApplicationSettings) -> DomainResult<ApplicationSettings> {
        let default_company_code =
            stored.default_company_code.as_ref().map(CompanyCode::new).transpose()?;

        let language = Language::new(&stored.language)?;
        let decimal_places = DecimalPlaces::new(stored.decimal_places)?;
        let date_format = DateFormat::new(&stored.date_format)?;
        let fiscal_year_start_month = FiscalYearStartMonth::new(stored.fiscal_year_start_month)?;
        let closing_day = ClosingDay::new(stored.closing_day)?;
        let backup_retention_days = BackupRetentionDays::new(stored.backup_retention_days)?;

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

impl ApplicationSettingsRepository for ApplicationSettingsRepositoryImpl {
    async fn find(&self) -> DomainResult<Option<ApplicationSettings>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = "default";

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            match txn.get(db, &key) {
                Ok(value) => {
                    let stored: StoredApplicationSettings = serde_json::from_slice(value)?;
                    let settings = Self::from_stored(&stored)?;
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Some(settings))
                }
                Err(lmdb::Error::NotFound) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        Ok(result)
    }

    async fn save(&self, settings: &ApplicationSettings) -> DomainResult<()> {
        let stored = Self::to_stored(settings);
        let value = serde_json::to_vec(&stored)
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = "default";

        tokio::task::spawn_blocking(move || {
            let mut txn = env.begin_rw_txn()?;
            txn.put(db, &key, &value, lmdb::WriteFlags::empty())?;
            txn.commit()?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }
}
