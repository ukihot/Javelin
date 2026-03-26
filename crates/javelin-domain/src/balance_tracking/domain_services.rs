// BalanceTracking集約のドメインサービス

/// BalanceTrackingドメインサービス
pub struct BalanceTrackingDomainService;

impl BalanceTrackingDomainService {
    /// 期限超過のBalanceTrackingを取得
    pub fn get_overdue_items(
        items: &[super::entities::BalanceTracking],
    ) -> Vec<&super::entities::BalanceTracking> {
        let today = chrono::Utc::now().date_naive();
        items
            .iter()
            .filter(|item| {
                item.due_date() < today
                    && *item.status() == super::values::BalanceTrackingStatus::Pending
            })
            .collect()
    }
}
