// CompanyMasterProjection - 会社マスタProjection

use std::sync::Arc;

use javelin_domain::masters::{CompanyCode, CompanyMaster, CompanyName};
use serde::{Deserialize, Serialize};

use crate::read::infrastructure::db::ProjectionDb;

/// 会社マスタProjection用の保存データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCompanyMaster {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// 会社マスタProjection
pub struct CompanyMasterProjection {
    projection_db: Arc<ProjectionDb>,
}

impl CompanyMasterProjection {
    pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
        Self { projection_db }
    }

    /// 会社マスタを保存
    pub async fn save(
        &self,
        company: &CompanyMaster,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stored = StoredCompanyMaster {
            code: company.code().value().to_string(),
            name: company.name().value().to_string(),
            is_active: company.is_active(),
        };

        let key = format!("company_master:{}", company.code().value());
        let value = serde_json::to_vec(&stored)?;

        self.projection_db
            .update_projection_batch("company_master", 1, vec![(key, value)], 0)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    /// 全会社マスタを取得
    pub async fn get_all(
        &self,
    ) -> Result<Vec<CompanyMaster>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Vec::new())
    }

    /// コードで会社マスタを取得
    pub async fn get_by_code(
        &self,
        code: &CompanyCode,
    ) -> Result<Option<CompanyMaster>, Box<dyn std::error::Error + Send + Sync>> {
        let key = format!("company_master:{}", code.value());
        let value = self.projection_db.get_projection(&key).await.map_err(|e| {
            Box::new(std::io::Error::other(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        match value {
            Some(v) => {
                let stored: StoredCompanyMaster = serde_json::from_slice(&v)?;
                let company = Self::from_stored(&stored)?;
                Ok(Some(company))
            }
            None => Ok(None),
        }
    }

    fn from_stored(
        stored: &StoredCompanyMaster,
    ) -> Result<CompanyMaster, Box<dyn std::error::Error + Send + Sync>> {
        let code = CompanyCode::new(&stored.code)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let name = CompanyName::new(&stored.name)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(CompanyMaster::new(code, name, stored.is_active))
    }
}
