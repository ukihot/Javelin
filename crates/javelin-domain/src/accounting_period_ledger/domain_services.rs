// 会計期間・帳簿集約のドメインサービス

/// 会計期間ドメインサービス
pub struct AccountingPeriodDomainService;

impl AccountingPeriodDomainService {
    /// 2つの会計期間が連続しているかを判定
    pub fn are_consecutive(
        period1: &super::entities::AccountingPeriod,
        period2: &super::entities::AccountingPeriod,
    ) -> bool {
        if period1.year() == period2.year() {
            period1.month() + 1 == period2.month()
        } else if period1.year() + 1 == period2.year() {
            period1.month() == 12 && period2.month() == 1
        } else {
            false
        }
    }
}
