// マスタデータイベント

use serde::{Deserialize, Serialize};

use super::AccountType;

/// 勘定科目マスタイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AccountMasterEvent {
    /// 勘定科目作成
    AccountMasterCreated { code: String, name: String, account_type: AccountType, is_active: bool },
    /// 勘定科目更新
    AccountMasterUpdated { code: String, name: String, account_type: AccountType, is_active: bool },
    /// 勘定科目削除
    AccountMasterDeleted { code: String },
}

impl AccountMasterEvent {
    fn event_type(&self) -> &str {
        match self {
            AccountMasterEvent::AccountMasterCreated { .. } => "AccountMasterCreated",
            AccountMasterEvent::AccountMasterUpdated { .. } => "AccountMasterUpdated",
            AccountMasterEvent::AccountMasterDeleted { .. } => "AccountMasterDeleted",
        }
    }

    fn aggregate_id(&self) -> &str {
        match self {
            AccountMasterEvent::AccountMasterCreated { code, .. }
            | AccountMasterEvent::AccountMasterUpdated { code, .. }
            | AccountMasterEvent::AccountMasterDeleted { code } => code,
        }
    }
}

impl crate::event::DomainEvent for AccountMasterEvent {
    fn event_type(&self) -> &str {
        self.event_type()
    }

    fn aggregate_id(&self) -> &str {
        self.aggregate_id()
    }

    fn version(&self) -> u64 {
        1
    }
}
