// SystemMasterService - システムマスタサービス

use super::{
    account_master::{AccountCode, AccountMaster, AccountName, AccountType},
    company_master::{CompanyCode, CompanyMaster, CompanyName},
    system_master::{SystemMaster, SystemMasterId},
    system_settings::{BackupRetentionDays, ClosingDay, FiscalYearStartMonth, SystemSettings},
    user_settings::{DateFormat, DecimalPlaces, Language, UserSettings},
};
use crate::{
    entity::EntityId, error::DomainResult,
    repositories::system_master_repository::SystemMasterRepository, value_object::ValueObject,
};

/// システムマスタサービス
pub struct SystemMasterService;

impl SystemMasterService {
    /// デフォルトのシステムマスタを作成
    pub fn create_default_system_master() -> SystemMaster {
        let account_masters = vec![
            AccountMaster::new(
                AccountCode::new("1000").unwrap(),
                AccountName::new("現金").unwrap(),
                AccountType::Asset,
                true,
            ),
            AccountMaster::new(
                AccountCode::new("1100").unwrap(),
                AccountName::new("普通預金").unwrap(),
                AccountType::Asset,
                true,
            ),
            AccountMaster::new(
                AccountCode::new("2000").unwrap(),
                AccountName::new("買掛金").unwrap(),
                AccountType::Liability,
                true,
            ),
            AccountMaster::new(
                AccountCode::new("3000").unwrap(),
                AccountName::new("資本金").unwrap(),
                AccountType::Equity,
                true,
            ),
            AccountMaster::new(
                AccountCode::new("4000").unwrap(),
                AccountName::new("売上高").unwrap(),
                AccountType::Revenue,
                true,
            ),
            AccountMaster::new(
                AccountCode::new("5000").unwrap(),
                AccountName::new("売上原価").unwrap(),
                AccountType::Expense,
                true,
            ),
        ];

        let company_masters = vec![
            CompanyMaster::new(
                CompanyCode::new("0001").unwrap(),
                CompanyName::new("本社").unwrap(),
                true,
            ),
            CompanyMaster::new(
                CompanyCode::new("0002").unwrap(),
                CompanyName::new("支社A").unwrap(),
                true,
            ),
        ];

        let user_settings = UserSettings::new(
            Some(CompanyCode::new("0001").unwrap()),
            Language::new("ja").unwrap(),
            DecimalPlaces::new(2).unwrap(),
            DateFormat::new("YYYY-MM-DD").unwrap(),
        );

        let system_settings = SystemSettings::new(
            FiscalYearStartMonth::new(4).unwrap(), // 4月開始
            ClosingDay::new(31).unwrap(),          // 月末締め
            true,                                  // 自動バックアップ有効
            BackupRetentionDays::new(90).unwrap(), // 90日保持
        );

        SystemMaster::new(
            SystemMasterId::new("default"),
            account_masters,
            company_masters,
            user_settings,
            system_settings,
        )
    }

    /// 勘定科目マスタを検証
    pub fn validate_account_master(account_master: &AccountMaster) -> DomainResult<()> {
        account_master.code().validate()?;
        account_master.name().validate()?;
        Ok(())
    }

    /// 会社マスタを検証
    pub fn validate_company_master(company_master: &CompanyMaster) -> DomainResult<()> {
        company_master.code().validate()?;
        company_master.name().validate()?;
        Ok(())
    }

    /// ユーザ設定を検証
    pub fn validate_user_settings(user_settings: &UserSettings) -> DomainResult<()> {
        if let Some(company_code) = user_settings.default_company_code() {
            company_code.validate()?;
        }
        user_settings.language().validate()?;
        user_settings.decimal_places().validate()?;
        user_settings.date_format().validate()?;
        Ok(())
    }

    /// システム設定を検証
    pub fn validate_system_settings(system_settings: &SystemSettings) -> DomainResult<()> {
        system_settings.fiscal_year_start_month().validate()?;
        system_settings.closing_day().validate()?;
        system_settings.backup_retention_days().validate()?;
        Ok(())
    }

    /// システムマスタを検証
    pub fn validate_system_master(system_master: &SystemMaster) -> DomainResult<()> {
        system_master.validate()
    }

    /// リポジトリを使用してデフォルトのシステムマスタを取得または作成
    pub async fn get_or_create_default_system_master<R: SystemMasterRepository>(
        repository: &R,
    ) -> DomainResult<SystemMaster> {
        if let Some(system_master) = repository.find_default().await? {
            Ok(system_master)
        } else {
            let default_system_master = Self::create_default_system_master();
            repository.save(&default_system_master).await?;
            Ok(default_system_master)
        }
    }

    /// 勘定科目マスタを追加
    pub async fn add_account_master<R: SystemMasterRepository>(
        repository: &R,
        system_master_id: &SystemMasterId,
        account_master: AccountMaster,
    ) -> DomainResult<()> {
        let mut system_master =
            repository.find_by_id(system_master_id).await?.ok_or_else(|| {
                crate::error::DomainError::NotFound(format!(
                    "システムマスタが見つかりません: {}",
                    system_master_id.value()
                ))
            })?;

        Self::validate_account_master(&account_master)?;

        // 重複チェック
        if system_master.find_account_master(account_master.code().value()).is_some() {
            return Err(crate::error::DomainError::ValidationError(format!(
                "勘定科目コードが既に存在します: {}",
                account_master.code().value()
            )));
        }

        system_master.add_account_master(account_master);
        system_master.validate()?;

        repository.save(&system_master).await
    }

    /// 会社マスタを追加
    pub async fn add_company_master<R: SystemMasterRepository>(
        repository: &R,
        system_master_id: &SystemMasterId,
        company_master: CompanyMaster,
    ) -> DomainResult<()> {
        let mut system_master =
            repository.find_by_id(system_master_id).await?.ok_or_else(|| {
                crate::error::DomainError::NotFound(format!(
                    "システムマスタが見つかりません: {}",
                    system_master_id.value()
                ))
            })?;

        Self::validate_company_master(&company_master)?;

        // 重複チェック
        if system_master.find_company_master(company_master.code().value()).is_some() {
            return Err(crate::error::DomainError::ValidationError(format!(
                "会社コードが既に存在します: {}",
                company_master.code().value()
            )));
        }

        let mut company_masters = system_master.company_masters().to_vec();
        company_masters.push(company_master);
        system_master.update_company_masters(company_masters);
        system_master.validate()?;

        repository.save(&system_master).await
    }
}
