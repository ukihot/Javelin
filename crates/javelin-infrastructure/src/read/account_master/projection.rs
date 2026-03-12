// AccountMasterProjection - 勘定科目マスタProjection

use std::sync::Arc;

use javelin_domain::chart_of_accounts::{AccountCode, AccountMaster, AccountName, AccountType};
use serde::{Deserialize, Serialize};

use crate::read::infrastructure::db::ProjectionDb;

/// 勘定科目マスタProjection用の保存データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAccountMaster {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub is_active: bool,
}

/// 勘定科目マスタProjection
pub struct AccountMasterProjection {
    projection_db: Arc<ProjectionDb>,
}

impl AccountMasterProjection {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 勘定科目マスタを保存
    pub async fn save(
        &self,
        account: &AccountMaster,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stored = StoredAccountMaster {
            code: account.code().value().to_string(),
            name: account.name().value().to_string(),
            account_type: account.account_type(),
            is_active: account.is_active(),
        };

        let key = format!("account_master:{}", account.code().value());
        let value = serde_json::to_vec(&stored)?;

        // update_projection_batchを使用して保存
        self.projection_db
            .update_projection_batch("account_master", 1, vec![(key, value)], 0)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    /// 全勘定科目マスタを取得
    pub async fn get_all(
        &self,
    ) -> Result<Vec<AccountMaster>, Box<dyn std::error::Error + Send + Sync>> {
        // ProjectionDBから全件取得（プレフィックススキャン）
        let prefix = "account_master:";
        let all_data = self.projection_db.scan_prefix(prefix).await.map_err(|e| {
            Box::new(std::io::Error::other(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        let mut accounts = Vec::new();
        for (_key, value) in all_data {
            let stored: StoredAccountMaster = serde_json::from_slice(&value)?;
            // is_activeがtrueのもののみ返す
            if stored.is_active {
                let account = Self::from_stored(&stored)?;
                accounts.push(account);
            }
        }

        Ok(accounts)
    }

    /// コードで勘定科目マスタを取得
    pub async fn get_by_code(
        &self,
        code: &AccountCode,
    ) -> Result<Option<AccountMaster>, Box<dyn std::error::Error + Send + Sync>> {
        let key = format!("account_master:{}", code.value());
        let value = self.projection_db.get_projection(&key).await.map_err(|e| {
            Box::new(std::io::Error::other(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        match value {
            Some(v) => {
                let stored: StoredAccountMaster = serde_json::from_slice(&v)?;
                let account = Self::from_stored(&stored)?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }

    fn from_stored(
        stored: &StoredAccountMaster,
    ) -> Result<AccountMaster, Box<dyn std::error::Error + Send + Sync>> {
        let code = AccountCode::new(&stored.code)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let name = AccountName::new(&stored.name)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(AccountMaster::new(code, name, stored.account_type, stored.is_active))
    }
}
