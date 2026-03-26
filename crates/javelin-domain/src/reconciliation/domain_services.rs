// 消込集約のドメインサービス

use crate::balance_tracking::entities::BalanceTracking;

/// 消込ドメインサービス
pub struct ReconciliationDomainService;

impl ReconciliationDomainService {
    /// 消込可能な金額を計算
    pub fn calculate_reconcilable_amount(
        balance_tracking: &BalanceTracking,
        existing_reconciliations: &[super::entities::Reconciliation],
    ) -> crate::common::Amount {
        let total_reconciled = existing_reconciliations
            .iter()
            .map(|r| r.reconciled_amount().clone())
            .fold(crate::common::Amount::zero(), |acc, amount| &acc + &amount);

        balance_tracking.amount() - &total_reconciled
    }
}
