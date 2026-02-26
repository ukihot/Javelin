// SubsidiaryAccountMasterRepositoryImpl - 補助科目マスタリポジトリ実装

use std::{path::Path, sync::Arc};

use javelin_domain::{
    error::DomainResult,
    masters::{AccountCode, SubsidiaryAccountCode, SubsidiaryAccountMaster, SubsidiaryAccountName},
    repositories::SubsidiaryAccountMasterRepository,
};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StoredSubsidiaryAccountMaster {
    code: String,
    name: String,
    parent_account_code: String,
    is_active: bool,
}

pub struct SubsidiaryAccountMasterRepositoryImpl {
    env: Arc<Environment>,
    db: Database,
}

impl SubsidiaryAccountMasterRepositoryImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        let env = Environment::new().set_max_dbs(1).set_map_size(50 * 1024 * 1024).open(path)?;

        let db = env.create_db(Some("subsidiary_account_masters"), DatabaseFlags::empty())?;

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
                SubsidiaryAccountMaster::new(
                    SubsidiaryAccountCode::new("1100-001").unwrap(),
                    SubsidiaryAccountName::new("普通預金A銀行").unwrap(),
                    AccountCode::new("1100").unwrap(),
                    true,
                ),
                SubsidiaryAccountMaster::new(
                    SubsidiaryAccountCode::new("1100-002").unwrap(),
                    SubsidiaryAccountName::new("普通預金B銀行").unwrap(),
                    AccountCode::new("1100").unwrap(),
                    true,
                ),
                SubsidiaryAccountMaster::new(
                    SubsidiaryAccountCode::new("2000-001").unwrap(),
                    SubsidiaryAccountName::new("買掛金X社").unwrap(),
                    AccountCode::new("2000").unwrap(),
                    true,
                ),
            ];

            for account in defaults {
                self.save(&account).await?;
            }
        }

        Ok(())
    }

    fn to_stored(account: &SubsidiaryAccountMaster) -> StoredSubsidiaryAccountMaster {
        StoredSubsidiaryAccountMaster {
            code: account.code().value().to_string(),
            name: account.name().value().to_string(),
            parent_account_code: account.parent_account_code().value().to_string(),
            is_active: account.is_active(),
        }
    }

    fn from_stored(
        stored: &StoredSubsidiaryAccountMaster,
    ) -> DomainResult<SubsidiaryAccountMaster> {
        let code = SubsidiaryAccountCode::new(&stored.code)?;
        let name = SubsidiaryAccountName::new(&stored.name)?;
        let parent_code = AccountCode::new(&stored.parent_account_code)?;
        Ok(SubsidiaryAccountMaster::new(code, name, parent_code, stored.is_active))
    }
}

impl SubsidiaryAccountMasterRepository for SubsidiaryAccountMasterRepositoryImpl {
    async fn find_by_code(
        &self,
        code: &SubsidiaryAccountCode,
    ) -> DomainResult<Option<SubsidiaryAccountMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = code.value().to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            match txn.get(db, &key) {
                Ok(value) => {
                    let stored: StoredSubsidiaryAccountMaster = serde_json::from_slice(value)?;
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

    async fn find_all(&self) -> DomainResult<Vec<SubsidiaryAccountMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let mut cursor = txn.open_ro_cursor(db)?;
            let mut accounts = Vec::new();

            for (_key, value) in cursor.iter() {
                let stored: StoredSubsidiaryAccountMaster = serde_json::from_slice(value)?;
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

    async fn find_by_parent_account(
        &self,
        parent_code: &AccountCode,
    ) -> DomainResult<Vec<SubsidiaryAccountMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let parent_key = parent_code.value().to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let mut cursor = txn.open_ro_cursor(db)?;
            let mut accounts = Vec::new();

            for (_key, value) in cursor.iter() {
                let stored: StoredSubsidiaryAccountMaster = serde_json::from_slice(value)?;
                if stored.parent_account_code == parent_key {
                    let account = Self::from_stored(&stored)?;
                    accounts.push(account);
                }
            }

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(accounts)
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        Ok(result)
    }

    async fn save(&self, account_master: &SubsidiaryAccountMaster) -> DomainResult<()> {
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

    async fn delete(&self, code: &SubsidiaryAccountCode) -> DomainResult<()> {
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
