// AccountMasterRepositoryImpl - 勘定科目マスタリポジトリの実装

use std::{path::Path, sync::Arc};

use javelin_domain::{
    error::DomainResult,
    masters::{AccountCode, AccountMaster, AccountName, AccountType},
    repositories::AccountMasterRepository,
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

pub struct AccountMasterRepositoryImpl {
    env: Arc<Environment>,
    db: Database,
}

impl AccountMasterRepositoryImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        let env = Environment::new().set_max_dbs(1).set_map_size(50 * 1024 * 1024).open(path)?;

        let db = env.create_db(Some("account_masters"), DatabaseFlags::empty())?;

        let repository = Self { env: Arc::new(env), db };
        repository.initialize_defaults().await?;

        Ok(repository)
    }

    async fn initialize_defaults(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let env = Arc::clone(&self.env);
        let db = self.db;

        let is_empty = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let mut cursor = txn.open_ro_cursor(db)?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(cursor.iter().next().is_none())
        })
        .await??;

        if is_empty {
            let defaults = vec![
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

            for account in defaults {
                self.save(&account).await?;
            }
        }

        Ok(())
    }

    fn to_stored(account: &AccountMaster) -> StoredAccountMaster {
        StoredAccountMaster {
            code: account.code().value().to_string(),
            name: account.name().value().to_string(),
            account_type: account.account_type(),
            is_active: account.is_active(),
        }
    }

    fn from_stored(stored: &StoredAccountMaster) -> DomainResult<AccountMaster> {
        let code = AccountCode::new(&stored.code)?;
        let name = AccountName::new(&stored.name)?;
        Ok(AccountMaster::new(code, name, stored.account_type, stored.is_active))
    }
}

impl AccountMasterRepository for AccountMasterRepositoryImpl {
    async fn find_by_code(&self, code: &AccountCode) -> DomainResult<Option<AccountMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = code.value().to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            match txn.get(db, &key) {
                Ok(value) => {
                    let stored: StoredAccountMaster = serde_json::from_slice(value)?;
                    let account = Self::from_stored(&stored)?;
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Some(account))
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

    async fn find_all(&self) -> DomainResult<Vec<AccountMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let mut cursor = txn.open_ro_cursor(db)?;
            let mut accounts = Vec::new();

            for (_key, value) in cursor.iter() {
                let stored: StoredAccountMaster = serde_json::from_slice(value)?;
                let account = Self::from_stored(&stored)?;
                accounts.push(account);
            }

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(accounts)
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        Ok(result)
    }

    async fn save(&self, account_master: &AccountMaster) -> DomainResult<()> {
        let stored = Self::to_stored(account_master);
        let value = serde_json::to_vec(&stored)
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = account_master.code().value().to_string();

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

    async fn delete(&self, code: &AccountCode) -> DomainResult<()> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = code.value().to_string();

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
