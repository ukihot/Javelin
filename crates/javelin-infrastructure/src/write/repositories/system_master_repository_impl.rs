// SystemMasterRepositoryImpl - システムマスタリポジトリの実装
// 責務: LMDBを使用したシステムマスタの永続化

use std::{path::Path, sync::Arc};

use javelin_domain::{
    entity::{Entity, EntityId},
    error::DomainResult,
    repositories::SystemMasterRepository,
    system_masters::{
        AccountCode, AccountMaster, AccountName, AccountType, BackupRetentionDays, ClosingDay,
        CompanyCode, CompanyMaster, CompanyName, DateFormat, DecimalPlaces, FiscalYearStartMonth,
        Language, SystemMaster, SystemMasterId, SystemMasterService, SystemSettings, UserSettings,
    },
};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StoredAccountMaster {
    code: String,
    name: String,
    account_type: AccountType,
    is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredCompanyMaster {
    code: String,
    name: String,
    is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredUserSettings {
    default_company_code: Option<String>,
    language: String,
    decimal_places: u8,
    date_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredSystemSettings {
    fiscal_year_start_month: u8,
    closing_day: u8,
    auto_backup_enabled: bool,
    backup_retention_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredSystemMaster {
    id: String,
    version: u64,
    account_masters: Vec<StoredAccountMaster>,
    company_masters: Vec<StoredCompanyMaster>,
    user_settings: StoredUserSettings,
    system_settings: StoredSystemSettings,
}

/// システムマスタリポジトリの実装
pub struct SystemMasterRepositoryImpl {
    env: Arc<Environment>,
    db: Database,
}

impl SystemMasterRepositoryImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // ディレクトリが存在しない場合は作成
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        // LMDB環境を作成
        let env = Environment::new()
            .set_max_dbs(2)
            .set_map_size(100 * 1024 * 1024) // 100MB
            .open(path)?;

        // データベース作成
        let db = env.create_db(Some("system_masters"), DatabaseFlags::empty())?;

        let repository = Self { env: Arc::new(env), db };

        // 初回起動時にデフォルトデータを投入
        repository.initialize_if_empty().await?;

        Ok(repository)
    }

    /// データベースが空の場合、デフォルトデータを投入
    async fn initialize_if_empty(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let env = Arc::clone(&self.env);
        let db = self.db;

        let is_empty = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let mut cursor = txn.open_ro_cursor(db)?;
            let is_empty = cursor.iter().next().is_none();
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(is_empty)
        })
        .await??;

        if is_empty {
            let default_system_master = SystemMasterService::create_default_system_master();
            self.save(&default_system_master).await?;
        }

        Ok(())
    }

    /// ドメインオブジェクトから保存用データに変換
    fn to_stored(system_master: &SystemMaster) -> StoredSystemMaster {
        let account_masters = system_master
            .account_masters()
            .iter()
            .map(|am| StoredAccountMaster {
                code: am.code().value().to_string(),
                name: am.name().value().to_string(),
                account_type: am.account_type(),
                is_active: am.is_active(),
            })
            .collect();

        let company_masters = system_master
            .company_masters()
            .iter()
            .map(|cm| StoredCompanyMaster {
                code: cm.code().value().to_string(),
                name: cm.name().value().to_string(),
                is_active: cm.is_active(),
            })
            .collect();

        let user_settings = system_master.user_settings();
        let stored_user_settings = StoredUserSettings {
            default_company_code: user_settings
                .default_company_code()
                .map(|c| c.value().to_string()),
            language: user_settings.language().value().to_string(),
            decimal_places: user_settings.decimal_places().value(),
            date_format: user_settings.date_format().value().to_string(),
        };

        let system_settings = system_master.system_settings();
        let stored_system_settings = StoredSystemSettings {
            fiscal_year_start_month: system_settings.fiscal_year_start_month().value(),
            closing_day: system_settings.closing_day().value(),
            auto_backup_enabled: system_settings.auto_backup_enabled(),
            backup_retention_days: system_settings.backup_retention_days().value(),
        };

        StoredSystemMaster {
            id: system_master.id().value().to_string(),
            version: system_master.version(),
            account_masters,
            company_masters,
            user_settings: stored_user_settings,
            system_settings: stored_system_settings,
        }
    }

    /// 保存用データからドメインオブジェクトに変換
    #[allow(dead_code)]
    fn from_stored(stored: &StoredSystemMaster) -> DomainResult<SystemMaster> {
        let account_masters = stored
            .account_masters
            .iter()
            .map(|am| {
                let code = AccountCode::new(&am.code)?;
                let name = AccountName::new(&am.name)?;
                Ok(AccountMaster::new(code, name, am.account_type, am.is_active))
            })
            .collect::<DomainResult<Vec<_>>>()?;

        let company_masters = stored
            .company_masters
            .iter()
            .map(|cm| {
                let code = CompanyCode::new(&cm.code)?;
                let name = CompanyName::new(&cm.name)?;
                Ok(CompanyMaster::new(code, name, cm.is_active))
            })
            .collect::<DomainResult<Vec<_>>>()?;

        let user_settings = {
            let default_company_code = stored
                .user_settings
                .default_company_code
                .as_ref()
                .map(CompanyCode::new)
                .transpose()?;

            let language = Language::new(&stored.user_settings.language)?;
            let decimal_places = DecimalPlaces::new(stored.user_settings.decimal_places)?;
            let date_format = DateFormat::new(&stored.user_settings.date_format)?;

            UserSettings::new(default_company_code, language, decimal_places, date_format)
        };

        let system_settings = {
            let fiscal_year_start_month =
                FiscalYearStartMonth::new(stored.system_settings.fiscal_year_start_month)?;
            let closing_day = ClosingDay::new(stored.system_settings.closing_day)?;
            let backup_retention_days =
                BackupRetentionDays::new(stored.system_settings.backup_retention_days)?;

            SystemSettings::new(
                fiscal_year_start_month,
                closing_day,
                stored.system_settings.auto_backup_enabled,
                backup_retention_days,
            )
        };

        let system_master = SystemMaster::new(
            SystemMasterId::new(&stored.id),
            account_masters,
            company_masters,
            user_settings,
            system_settings,
        );

        Ok(system_master)
    }
}
impl SystemMasterRepository for SystemMasterRepositoryImpl {
    async fn save(&self, system_master: &SystemMaster) -> DomainResult<()> {
        let stored = Self::to_stored(system_master);
        let value = serde_json::to_vec(&stored)
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = system_master.id().value().to_string();

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

    async fn delete(&self, id: &SystemMasterId) -> DomainResult<()> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = id.value().to_string();

        tokio::task::spawn_blocking(move || {
            let mut txn = env.begin_rw_txn()?;
            txn.del(db, &key, None)?;
            txn.commit()?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }
}
