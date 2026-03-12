// AccountMasterRepositoryImpl - 勘定科目マスタリポジトリの実装

use std::{path::Path, sync::Arc};

use javelin_domain::{
    chart_of_accounts::{
        entities::AccountMaster,
        repositories::AccountMasterRepository,
        values::{AccountCode, AccountName, AccountType},
    },
    common::RepositoryBase,
    error::DomainResult,
};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};

use crate::{types::ExpectedVersion, write::event_store::EventStore};

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
    event_store: Arc<EventStore>,
}

impl AccountMasterRepositoryImpl {
    pub async fn new(
        path: &Path,
        event_store: Arc<EventStore>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        let env = Environment::new().set_max_dbs(1).set_map_size(50 * 1024 * 1024).open(path)?;

        let db = env.create_db(Some("account_masters"), DatabaseFlags::empty())?;

        let repository = Self { env: Arc::new(env), db, event_store };
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
            // IFRSベースの勘定科目テンプレート
            let defaults = vec![
                // 資産の部 (Assets)
                // 流動資産 (Current Assets)
                AccountMaster::new(
                    AccountCode::new("1100").unwrap(),
                    AccountName::new("現金及び現金同等物").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1110").unwrap(),
                    AccountName::new("営業債権及びその他の債権").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1120").unwrap(),
                    AccountName::new("棚卸資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1130").unwrap(),
                    AccountName::new("その他の金融資産（流動）").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1140").unwrap(),
                    AccountName::new("その他の流動資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                // 非流動資産 (Non-current Assets)
                AccountMaster::new(
                    AccountCode::new("1200").unwrap(),
                    AccountName::new("有形固定資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1210").unwrap(),
                    AccountName::new("使用権資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1220").unwrap(),
                    AccountName::new("のれん").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1230").unwrap(),
                    AccountName::new("無形資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1240").unwrap(),
                    AccountName::new("持分法で会計処理されている投資").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1250").unwrap(),
                    AccountName::new("その他の金融資産（非流動）").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1260").unwrap(),
                    AccountName::new("繰延税金資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("1270").unwrap(),
                    AccountName::new("その他の非流動資産").unwrap(),
                    AccountType::Asset,
                    true,
                ),
                // 負債の部 (Liabilities)
                // 流動負債 (Current Liabilities)
                AccountMaster::new(
                    AccountCode::new("2100").unwrap(),
                    AccountName::new("営業債務及びその他の債務").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2110").unwrap(),
                    AccountName::new("社債及び借入金（流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2120").unwrap(),
                    AccountName::new("リース負債（流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2130").unwrap(),
                    AccountName::new("その他の金融負債（流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2140").unwrap(),
                    AccountName::new("未払法人所得税").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2150").unwrap(),
                    AccountName::new("引当金（流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2160").unwrap(),
                    AccountName::new("その他の流動負債").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                // 非流動負債 (Non-current Liabilities)
                AccountMaster::new(
                    AccountCode::new("2200").unwrap(),
                    AccountName::new("社債及び借入金（非流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2210").unwrap(),
                    AccountName::new("リース負債（非流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2220").unwrap(),
                    AccountName::new("その他の金融負債（非流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2230").unwrap(),
                    AccountName::new("退職給付に係る負債").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2240").unwrap(),
                    AccountName::new("引当金（非流動）").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2250").unwrap(),
                    AccountName::new("繰延税金負債").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("2260").unwrap(),
                    AccountName::new("その他の非流動負債").unwrap(),
                    AccountType::Liability,
                    true,
                ),
                // 資本の部 (Equity)
                AccountMaster::new(
                    AccountCode::new("3100").unwrap(),
                    AccountName::new("資本金").unwrap(),
                    AccountType::Equity,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("3110").unwrap(),
                    AccountName::new("資本剰余金").unwrap(),
                    AccountType::Equity,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("3120").unwrap(),
                    AccountName::new("利益剰余金").unwrap(),
                    AccountType::Equity,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("3130").unwrap(),
                    AccountName::new("自己株式").unwrap(),
                    AccountType::Equity,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("3140").unwrap(),
                    AccountName::new("その他の資本の構成要素").unwrap(),
                    AccountType::Equity,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("3150").unwrap(),
                    AccountName::new("非支配持分").unwrap(),
                    AccountType::Equity,
                    true,
                ),
                // 収益の部 (Revenue)
                AccountMaster::new(
                    AccountCode::new("4100").unwrap(),
                    AccountName::new("売上収益").unwrap(),
                    AccountType::Revenue,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("4200").unwrap(),
                    AccountName::new("その他の収益").unwrap(),
                    AccountType::Revenue,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("4300").unwrap(),
                    AccountName::new("金融収益").unwrap(),
                    AccountType::Revenue,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("4400").unwrap(),
                    AccountName::new("持分法による投資利益").unwrap(),
                    AccountType::Revenue,
                    true,
                ),
                // 費用の部 (Expenses)
                AccountMaster::new(
                    AccountCode::new("5100").unwrap(),
                    AccountName::new("売上原価").unwrap(),
                    AccountType::Expense,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("5200").unwrap(),
                    AccountName::new("販売費及び一般管理費").unwrap(),
                    AccountType::Expense,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("5300").unwrap(),
                    AccountName::new("研究開発費").unwrap(),
                    AccountType::Expense,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("5400").unwrap(),
                    AccountName::new("その他の費用").unwrap(),
                    AccountType::Expense,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("5500").unwrap(),
                    AccountName::new("金融費用").unwrap(),
                    AccountType::Expense,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("5600").unwrap(),
                    AccountName::new("持分法による投資損失").unwrap(),
                    AccountType::Expense,
                    true,
                ),
                AccountMaster::new(
                    AccountCode::new("5700").unwrap(),
                    AccountName::new("法人所得税費用").unwrap(),
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

    #[allow(dead_code)]
    fn from_stored(stored: &StoredAccountMaster) -> DomainResult<AccountMaster> {
        let code = AccountCode::new(&stored.code)?;
        let name = AccountName::new(&stored.name)?;
        Ok(AccountMaster::new(code, name, stored.account_type, stored.is_active))
    }
}

// AccountMasterRepository trait implementation
impl AccountMasterRepository for AccountMasterRepositoryImpl {}

// RepositoryBase trait implementation
impl RepositoryBase<AccountMaster> for AccountMasterRepositoryImpl {
    async fn save(&self, account_master: &AccountMaster) -> DomainResult<()> {
        let stored = Self::to_stored(account_master);
        let value = serde_json::to_vec(&stored)
            .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = account_master.code().value().to_string();

        // LMDBに保存
        tokio::task::spawn_blocking(move || {
            let mut txn = env.begin_rw_txn()?;
            txn.put(db, &key, &value, lmdb::WriteFlags::empty())?;
            txn.commit()?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?
        .map_err(|e| javelin_domain::error::DomainError::RepositoryError(e.to_string()))?;

        // AccountMaster does not use event sourcing - direct LMDB storage only

        Ok(())
    }

    async fn load(&self, id: &str) -> DomainResult<Option<AccountMaster>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let key = id.to_string();

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            match txn.get(db, &key) {
                Ok(bytes) => {
                    let stored: StoredAccountMaster = serde_json::from_slice(bytes)?;
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
                let account = Self::from_stored(&stored)?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }
}
