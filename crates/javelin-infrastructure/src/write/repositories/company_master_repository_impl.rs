// CompanyMasterRepositoryImpl - 会社マスタリポジトリの実装

use std::{path::Path, sync::Arc};

use javelin_domain::{
    common::RepositoryBase,
    company::{
        entities::CompanyMaster,
        repositories::CompanyMasterRepository,
        values::{CompanyCode, CompanyName},
    },
    error::DomainResult,
};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StoredCompanyMaster {
    code: String,
    name: String,
    is_active: bool,
}

pub struct CompanyMasterRepositoryImpl {
    env: Arc<Environment>,
    db: Database,
}

impl CompanyMasterRepositoryImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        let env = Environment::new().set_max_dbs(1).set_map_size(50 * 1024 * 1024).open(path)?;

        let db = env.create_db(Some("company_masters"), DatabaseFlags::empty())?;

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

            for company in defaults {
                self.save(&company).await?;
            }
        }

        Ok(())
    }

    fn to_stored(company: &CompanyMaster) -> StoredCompanyMaster {
        StoredCompanyMaster {
            code: company.code().value().to_string(),
            name: company.name().value().to_string(),
            is_active: company.is_active(),
        }
    }

    #[allow(dead_code)]
    fn from_stored(stored: &StoredCompanyMaster) -> DomainResult<CompanyMaster> {
        let code = CompanyCode::new(&stored.code)?;
        let name = CompanyName::new(&stored.name)?;
        Ok(CompanyMaster::new(code, name, stored.is_active))
    }
}

// CompanyMasterRepository trait implementation
impl CompanyMasterRepository for CompanyMasterRepositoryImpl {}

// RepositoryBase trait implementation
impl RepositoryBase<CompanyMaster> for CompanyMasterRepositoryImpl {
    async fn save(&self, company_master: &CompanyMaster) -> DomainResult<()> {
        let stored = Self::to_stored(company_master);
        let value = serde_json::to_vec(&stored)
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = company_master.code().value().to_string();

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

    async fn load(&self, id: &str) -> DomainResult<Option<CompanyMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = id.to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            match txn.get(db, &key) {
                Ok(bytes) => {
                    let stored: StoredCompanyMaster = serde_json::from_slice(bytes)?;
                    Ok(Some(stored))
                }
                Err(lmdb::Error::NotFound) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e: Box<dyn std::error::Error + Send + Sync>| {
            javelin_domain::error::DomainError::RepositoryError(e.to_string())
        })?;

        match result {
            Some(stored) => {
                let company = Self::from_stored(&stored)?;
                Ok(Some(company))
            }
            None => Ok(None),
        }
    }
}
