// 入出金集約のドメインサービス

/// 入出金ドメインサービス
pub struct CashFlowDomainService;

impl CashFlowDomainService {
    /// 入出金の残高を計算
    pub fn calculate_balance(cash_flows: &[super::entities::CashFlow]) -> crate::common::Amount {
        cash_flows
            .iter()
            .map(|cf| cf.amount().clone())
            .fold(crate::common::Amount::zero(), |acc, amount| &acc + &amount)
    }
}
