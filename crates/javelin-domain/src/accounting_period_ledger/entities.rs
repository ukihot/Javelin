// 会計期間・帳簿集約のエンティティ

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::values::AccountingPeriodStatus;
use crate::{
    entity::{Entity, EntityId},
    error::{DomainError, DomainResult},
};

/// 会計期間ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountingPeriodId(String);

impl AccountingPeriodId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl EntityId for AccountingPeriodId {
    fn value(&self) -> &str {
        &self.0
    }
}

/// 会計期間エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingPeriod {
    id: AccountingPeriodId,
    year: u32,
    month: u8,
    start_date: NaiveDate,
    end_date: NaiveDate,
    status: AccountingPeriodStatus,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl AccountingPeriod {
    pub fn new(year: u32, month: u8) -> DomainResult<Self> {
        if !(1..=12).contains(&month) {
            return Err(DomainError::ValidationError(
                "月は1-12の範囲である必要があります".to_string(),
            ));
        }

        let start_date = NaiveDate::from_ymd_opt(year as i32, month.into(), 1).unwrap();
        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1).unwrap().pred_opt().unwrap()
        } else {
            NaiveDate::from_ymd_opt(year as i32, (month + 1).into(), 1)
                .unwrap()
                .pred_opt()
                .unwrap()
        };

        let now = chrono::Utc::now();
        Ok(Self {
            id: AccountingPeriodId::new(uuid::Uuid::new_v4().to_string()),
            year,
            month,
            start_date,
            end_date,
            status: AccountingPeriodStatus::Open,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters
    pub fn id(&self) -> &AccountingPeriodId {
        &self.id
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    pub fn status(&self) -> &AccountingPeriodStatus {
        &self.status
    }

    /// 期間を閉じる
    pub fn close(&mut self) -> DomainResult<()> {
        if self.status == AccountingPeriodStatus::Closed {
            return Err(DomainError::ValidationError("既に閉じられた期間です".to_string()));
        }
        self.status = AccountingPeriodStatus::Closed;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
}

impl Entity for AccountingPeriod {
    type Id = AccountingPeriodId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
