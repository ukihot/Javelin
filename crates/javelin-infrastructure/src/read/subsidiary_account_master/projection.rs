// SubsidiaryAccountMasterProjection - 補助科目マスタProjection

use std::sync::Arc;

use javelin_domain::chart_of_accounts::{
    AccountCode, SubsidiaryAccountCode, SubsidiaryAccountMaster, SubsidiaryAccountName,
};
use serde::{Deserialize, Serialize};

use crate::read::infrastructure::db::ProjectionDb;

/// 補助科目マスタProjection用の保存データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSubsidiaryAccountMaster {
    pub code: String,
    pub name: String,
    pub parent_account_code: String,
    pub is_active: bool,
}

/// 補助科目マスタProjection
pub struct SubsidiaryAccountMasterProjection {
    projection_db: Arc<ProjectionDb>,
}

impl SubsidiaryAccountMasterProjection {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 補助科目マスタを保存
    pub async fn save(
        &self,
        account: &SubsidiaryAccountMaster,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stored = StoredSubsidiaryAccountMaster {
            code: account.code().value().to_string(),
            name: account.name().value().to_string(),
            parent_account_code: account.parent_account_code().value().to_string(),
            is_active: account.is_active(),
        };

        let key = format!("subsidiary_account_master:{}", account.code().value());
        let value = serde_json::to_vec(&stored)?;

        self.projection_db
            .update_projection_batch("subsidiary_account_master", 1, vec![(key, value)], 0)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    /// 全補助科目マスタを取得
    pub async fn get_all(
        &self,
    ) -> Result<Vec<SubsidiaryAccountMaster>, Box<dyn std::error::Error + Send + Sync>> {
        // ProjectionDBから全件取得（プレフィックススキャン）
        let prefix = "subsidiary_account_master:";
        let all_data = self.projection_db.scan_prefix(prefix).await.map_err(|e| {
            Box::new(std::io::Error::other(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        let mut accounts = Vec::new();
        for (_key, value) in all_data {
            let stored: StoredSubsidiaryAccountMaster = serde_json::from_slice(&value)?;
            let account = Self::from_stored(&stored)?;
            accounts.push(account);
        }

        Ok(accounts)
    }

    /// コードで補助科目マスタを取得
    pub async fn get_by_code(
        &self,
        code: &SubsidiaryAccountCode,
    ) -> Result<Option<SubsidiaryAccountMaster>, Box<dyn std::error::Error + Send + Sync>> {
        let key = format!("subsidiary_account_master:{}", code.value());
        let value = self.projection_db.get_projection(&key).await.map_err(|e| {
            Box::new(std::io::Error::other(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        match value {
            Some(v) => {
                let stored: StoredSubsidiaryAccountMaster = serde_json::from_slice(&v)?;
                let account = Self::from_stored(&stored)?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }

    /// 親勘定科目コードで補助科目マスタを取得
    pub async fn get_by_parent_account(
        &self,
        _parent_code: &AccountCode,
    ) -> Result<Vec<SubsidiaryAccountMaster>, Box<dyn std::error::Error + Send + Sync>> {
        // 全件取得してフィルタリング（将来的にはインデックスを使用）
        let all = self.get_all().await?;
        Ok(all)
    }

    fn from_stored(
        stored: &StoredSubsidiaryAccountMaster,
    ) -> Result<SubsidiaryAccountMaster, Box<dyn std::error::Error + Send + Sync>> {
        let code = SubsidiaryAccountCode::new(&stored.code)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let name = SubsidiaryAccountName::new(&stored.name)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let parent_account_code = AccountCode::new(&stored.parent_account_code)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(SubsidiaryAccountMaster::new(code, name, parent_account_code, stored.is_active))
    }
}
