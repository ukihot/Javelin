// MasterDataLoaderの実装 - 各マスタリポジトリを使用
// 責務: 各マスタリポジトリからマスタデータをロード

use std::{path::Path, sync::Arc};

use javelin_application::{
    error::ApplicationResult,
    query_service::{
        AccountMaster, CompanyMaster, MasterData, MasterDataLoaderService, SystemSettings,
        UserOptions,
    },
};

use crate::repositories::{
    AccountMasterRepositoryImpl, ApplicationSettingsRepositoryImpl, CompanyMasterRepositoryImpl,
};

/// マスタデータローダーの実装
pub struct MasterDataLoaderImpl {
    account_repository: Arc<AccountMasterRepositoryImpl>,
    company_repository: Arc<CompanyMasterRepositoryImpl>,
    settings_repository: Arc<ApplicationSettingsRepositoryImpl>,
}

impl MasterDataLoaderImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let account_path = path.join("accounts");
        let company_path = path.join("companies");
        let settings_path = path.join("settings");

        let account_repository = AccountMasterRepositoryImpl::new(&account_path).await?;
        let company_repository = CompanyMasterRepositoryImpl::new(&company_path).await?;
        let settings_repository = ApplicationSettingsRepositoryImpl::new(&settings_path).await?;

        Ok(Self {
            account_repository: Arc::new(account_repository),
            company_repository: Arc::new(company_repository),
            settings_repository: Arc::new(settings_repository),
        })
    }

    /// 各リポジトリからマスタデータをロード
    async fn load_from_repositories(&self) -> ApplicationResult<MasterData> {
        use javelin_domain::repositories::{
            AccountMasterRepository, ApplicationSettingsRepository, CompanyMasterRepository,
        };

        let account_masters = self.account_repository.find_all().await.map_err(|e| {
            javelin_application::error::ApplicationError::QueryExecutionFailed(e.to_string())
        })?;

        let company_masters = self.company_repository.find_all().await.map_err(|e| {
            javelin_application::error::ApplicationError::QueryExecutionFailed(e.to_string())
        })?;

        let settings = self
            .settings_repository
            .find()
            .await
            .map_err(|e| {
                javelin_application::error::ApplicationError::QueryExecutionFailed(e.to_string())
            })?
            .ok_or_else(|| {
                javelin_application::error::ApplicationError::QueryExecutionFailed(
                    "Application settings not found".to_string(),
                )
            })?;

        let accounts = account_masters.iter().map(AccountMaster::from).collect();
        let companies = company_masters.iter().map(CompanyMaster::from).collect();
        let user_options = UserOptions::from(&settings);
        let system_settings = SystemSettings::from(&settings);

        Ok(MasterData { accounts, companies, user_options, system_settings })
    }
}

impl MasterDataLoaderService for MasterDataLoaderImpl {
    async fn load_master_data(&self) -> ApplicationResult<MasterData> {
        self.load_from_repositories().await
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_load_master_data() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let master_db_path = temp_dir.path().join("master_data");

        let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();

        let result = loader.load_master_data().await;
        assert!(result.is_ok(), "Master data loading should succeed");

        let master_data = result.unwrap();
        assert!(!master_data.accounts.is_empty(), "Should have accounts");
        assert!(!master_data.companies.is_empty(), "Should have companies");
        assert_eq!(master_data.user_options.language, "ja", "Default language should be ja");
        assert_eq!(
            master_data.system_settings.fiscal_year_start_month, 4,
            "Default fiscal year start month should be 4"
        );
    }

    #[tokio::test]
    async fn test_default_master_data_structure() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let master_db_path = temp_dir.path().join("master_data");

        let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();
        let master_data = loader.load_master_data().await.unwrap();

        // 勘定科目の検証
        assert_eq!(master_data.accounts.len(), 6);
        assert!(master_data.accounts.iter().any(|a| a.code == "1000" && a.name == "現金"));

        // 会社マスタの検証
        assert_eq!(master_data.companies.len(), 2);
        assert!(master_data.companies.iter().any(|c| c.code == "0001" && c.name == "本社"));

        // ユーザ設定の検証
        assert_eq!(master_data.user_options.decimal_places, 2);
        assert_eq!(master_data.user_options.date_format, "YYYY-MM-DD");

        // システム設定の検証
        assert_eq!(master_data.system_settings.fiscal_year_start_month, 4);
        assert_eq!(master_data.system_settings.closing_day, 31);
        assert!(master_data.system_settings.auto_backup_enabled);
        assert_eq!(master_data.system_settings.backup_retention_days, 90);
    }

    #[tokio::test]
    async fn test_persistence() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let master_db_path = temp_dir.path().join("master_data");

        // 最初のローダーでデータを保存
        {
            let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();
            let data = loader.load_master_data().await.unwrap();
            assert_eq!(data.accounts.len(), 6);
        }

        // 新しいローダーで同じデータを読み込めることを確認
        {
            let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();
            let data = loader.load_master_data().await.unwrap();
            assert_eq!(data.accounts.len(), 6);
            assert_eq!(data.companies.len(), 2);
            assert_eq!(data.system_settings.closing_day, 31);
        }
    }
}
