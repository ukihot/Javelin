// 会計期間エンティティ

use crate::{
    entity::Entity,
    error::{DomainError, DomainResult},
    financial_close::{
        accounting_period::values::{Date, DateTime, FiscalYear, Period, PeriodId},
        journal_entry::values::{identifiers::UserId, status::PeriodStatus},
    },
};

/// 会計期間エンティティ
#[derive(Debug, Clone)]
pub struct AccountingPeriod {
    /// 期間ID
    id: PeriodId,
    /// 会計年度
    fiscal_year: FiscalYear,
    /// 会計期間（月）
    period: Period,
    /// 期間ステータス
    status: PeriodStatus,
    /// 開始日
    start_date: Date,
    /// 終了日
    end_date: Date,
    /// 締め日時
    closed_at: Option<DateTime>,
    /// 締め実行者
    closed_by: Option<UserId>,
    /// 再オープン日時
    reopened_at: Option<DateTime>,
    /// 再オープン実行者
    reopened_by: Option<UserId>,
    /// 再オープン理由
    reopen_reason: Option<String>,
}

impl Entity for AccountingPeriod {
    type Id = PeriodId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl AccountingPeriod {
    /// 新しい会計期間を作成
    pub fn new(
        fiscal_year: FiscalYear,
        period: Period,
        start_date: Date,
        end_date: Date,
    ) -> DomainResult<Self> {
        // 開始日と終了日の妥当性チェック
        if start_date >= end_date {
            return Err(DomainError::InvalidAmount(
                "Start date must be before end date".to_string(),
            ));
        }

        let id = PeriodId::from_year_period(fiscal_year, period);

        Ok(Self {
            id,
            fiscal_year,
            period,
            status: PeriodStatus::Open,
            start_date,
            end_date,
            closed_at: None,
            closed_by: None,
            reopened_at: None,
            reopened_by: None,
            reopen_reason: None,
        })
    }

    /// 期間を締める
    pub fn close(&mut self, user_id: UserId, closed_at: DateTime) -> DomainResult<()> {
        // Open状態のみ締められる
        if !matches!(self.status, PeriodStatus::Open) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = PeriodStatus::Closed;
        self.closed_at = Some(closed_at);
        self.closed_by = Some(user_id);

        Ok(())
    }

    /// 期間をロックする
    pub fn lock(&mut self) -> DomainResult<()> {
        // Closed状態のみロックできる
        if !matches!(self.status, PeriodStatus::Closed) {
            return Err(DomainError::InvalidStatusTransition);
        }

        self.status = PeriodStatus::Locked;

        Ok(())
    }

    /// 期間を再オープンする
    pub fn reopen(
        &mut self,
        user_id: UserId,
        reason: String,
        reopened_at: DateTime,
    ) -> DomainResult<()> {
        // Closed状態のみ再オープンできる
        if !matches!(self.status, PeriodStatus::Closed) {
            return Err(DomainError::InvalidStatusTransition);
        }

        if reason.is_empty() {
            return Err(DomainError::InvalidAmount("Reopen reason is required".to_string()));
        }

        self.status = PeriodStatus::Open;
        self.reopened_at = Some(reopened_at);
        self.reopened_by = Some(user_id);
        self.reopen_reason = Some(reason);

        Ok(())
    }

    /// 指定された日付が期間内かチェック
    pub fn contains_date(&self, date: &Date) -> bool {
        date >= &self.start_date && date <= &self.end_date
    }

    /// 仕訳登録が可能かチェック
    pub fn can_post_journal(&self) -> bool {
        self.status.can_post_journal()
    }

    // Getters
    pub fn fiscal_year(&self) -> FiscalYear {
        self.fiscal_year
    }

    pub fn period(&self) -> Period {
        self.period
    }

    pub fn status(&self) -> &PeriodStatus {
        &self.status
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }

    pub fn closed_at(&self) -> Option<DateTime> {
        self.closed_at
    }

    pub fn closed_by(&self) -> Option<&UserId> {
        self.closed_by.as_ref()
    }

    pub fn reopened_at(&self) -> Option<DateTime> {
        self.reopened_at
    }

    pub fn reopened_by(&self) -> Option<&UserId> {
        self.reopened_by.as_ref()
    }

    pub fn reopen_reason(&self) -> Option<&str> {
        self.reopen_reason.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::entity::EntityId;

    fn create_test_period() -> AccountingPeriod {
        let year = FiscalYear::new(2024).unwrap();
        let period = Period::new(3).unwrap();
        let start = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();

        AccountingPeriod::new(year, period, start, end).unwrap()
    }

    #[test]
    fn test_new_accounting_period() {
        let period = create_test_period();

        assert_eq!(period.fiscal_year().value(), 2024);
        assert_eq!(period.period().value(), 3);
        assert_eq!(period.status(), &PeriodStatus::Open);
        assert!(period.can_post_journal());
    }

    #[test]
    fn test_invalid_date_range() {
        let year = FiscalYear::new(2024).unwrap();
        let period = Period::new(3).unwrap();
        let start = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();

        let result = AccountingPeriod::new(year, period, start, end);
        assert!(result.is_err());
    }

    #[test]
    fn test_close_period() {
        let mut period = create_test_period();
        let user_id = UserId::new("user1".to_string());
        let closed_at = chrono::Utc::now().naive_utc();

        let result = period.close(user_id.clone(), closed_at);
        assert!(result.is_ok());
        assert_eq!(period.status(), &PeriodStatus::Closed);
        assert!(!period.can_post_journal());
        assert_eq!(period.closed_by().unwrap().value(), "user1");
    }

    #[test]
    fn test_close_already_closed() {
        let mut period = create_test_period();
        let user_id = UserId::new("user1".to_string());
        let closed_at = chrono::Utc::now().naive_utc();

        period.close(user_id.clone(), closed_at).unwrap();

        // 2回目の締めは失敗
        let result = period.close(user_id, closed_at);
        assert!(result.is_err());
    }

    #[test]
    fn test_lock_period() {
        let mut period = create_test_period();
        let user_id = UserId::new("user1".to_string());
        let closed_at = chrono::Utc::now().naive_utc();

        period.close(user_id, closed_at).unwrap();

        let result = period.lock();
        assert!(result.is_ok());
        assert_eq!(period.status(), &PeriodStatus::Locked);
        assert!(!period.can_post_journal());
    }

    #[test]
    fn test_lock_open_period_fails() {
        let mut period = create_test_period();

        let result = period.lock();
        assert!(result.is_err());
    }

    #[test]
    fn test_reopen_period() {
        let mut period = create_test_period();
        let user_id = UserId::new("user1".to_string());
        let closed_at = chrono::Utc::now().naive_utc();

        period.close(user_id.clone(), closed_at).unwrap();

        let reopened_at = chrono::Utc::now().naive_utc();
        let result = period.reopen(user_id, "Need to add missing entries".to_string(), reopened_at);

        assert!(result.is_ok());
        assert_eq!(period.status(), &PeriodStatus::Open);
        assert!(period.can_post_journal());
        assert_eq!(period.reopen_reason().unwrap(), "Need to add missing entries");
    }

    #[test]
    fn test_reopen_without_reason_fails() {
        let mut period = create_test_period();
        let user_id = UserId::new("user1".to_string());
        let closed_at = chrono::Utc::now().naive_utc();

        period.close(user_id.clone(), closed_at).unwrap();

        let reopened_at = chrono::Utc::now().naive_utc();
        let result = period.reopen(user_id, "".to_string(), reopened_at);

        assert!(result.is_err());
    }

    #[test]
    fn test_reopen_open_period_fails() {
        let mut period = create_test_period();
        let user_id = UserId::new("user1".to_string());
        let reopened_at = chrono::Utc::now().naive_utc();

        let result = period.reopen(user_id, "Test reason".to_string(), reopened_at);
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_date() {
        let period = create_test_period();

        let date_in_period = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(period.contains_date(&date_in_period));

        let date_before = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        assert!(!period.contains_date(&date_before));

        let date_after = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        assert!(!period.contains_date(&date_after));

        // 境界値チェック
        assert!(period.contains_date(&period.start_date()));
        assert!(period.contains_date(&period.end_date()));
    }
}
