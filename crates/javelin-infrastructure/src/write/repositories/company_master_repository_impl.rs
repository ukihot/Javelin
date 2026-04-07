// CompanyMasterRepositoryImpl - 会社マスタリポジトリの実装
//
// Organization集約をLMDBに永続化する。
// 後方互換性のため、CompanyMasterの保存・ロードインターフェースを維持しつつ
// 内部でOrganization集約全体を管理する。

use std::{path::Path, sync::Arc};

use javelin_domain::{
    common::RepositoryBase,
    company::{
        entities::{CompanyMaster, Organization},
        repositories::company_master_repository::CompanyMasterRepository,
        values::{CompanyCode, CompanyName, OrganizationId},
    },
    entity::{Entity, EntityId},
    error::DomainResult,
};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StoredCompanyMaster {
    organization_id: String,
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
                Organization::new(
                    OrganizationId::generate(),
                    CompanyCode::new("0001").unwrap(),
                    CompanyName::new("本社").unwrap(),
                ),
                Organization::new(
                    OrganizationId::generate(),
                    CompanyCode::new("0002").unwrap(),
                    CompanyName::new("支社A").unwrap(),
                ),
            ];

            for org in defaults {
                self.save(&org).await?;
            }
        }

        Ok(())
    }

    fn to_stored(org: &Organization) -> StoredCompanyMaster {
        StoredCompanyMaster {
            organization_id: org.id().value().to_string(),
            code: org.company().code().value().to_string(),
            name: org.company().name().value().to_string(),
            is_active: org.company().is_active(),
        }
    }

    #[allow(dead_code)]
    fn from_stored(stored: &StoredCompanyMaster) -> DomainResult<Organization> {
        let id = OrganizationId::new(&stored.organization_id);
        let code = CompanyCode::new(&stored.code)?;
        let name = CompanyName::new(&stored.name)?;
        let mut org = Organization::new(id, code, name);
        if !stored.is_active {
            org.company_mut().deactivate();
        }
        Ok(org)
    }
}

// CompanyMasterRepository trait implementation
impl CompanyMasterRepository for CompanyMasterRepositoryImpl {}

// RepositoryBase trait implementation
impl RepositoryBase<Organization> for CompanyMasterRepositoryImpl {
    async fn save(&self, organization: &Organization) -> DomainResult<()> {
        let stored = Self::to_stored(organization);
        let value = serde_json::to_vec(&stored)
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = organization.company().code().value().to_string();

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

    async fn load(&self, id: &str) -> DomainResult<Option<Organization>> {
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
                let org = Self::from_stored(&stored)?;
                Ok(Some(org))
            }
            None => Ok(None),
        }
    }
}
