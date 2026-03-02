// VerifyLedgerConsistencyInteractor - 元帳整合性検証処理
// 責務: ドメイン層の元帳サービスを活用した整合性検証実行

use chrono::Utc;
use javelin_domain::financial_close::ledger::{
    entities::{GeneralLedger, SubsidiaryLedger, SubsidiaryLedgerType},
    services::LedgerService,
};

use crate::{
    dtos::{
        AlertSeverity, AnomalyAlert, BalanceChange, DiscrepancyDetail, TemporaryAccountBalance,
        VerificationLevel, VerifyLedgerConsistencyRequest, VerifyLedgerConsistencyResponse,
    },
    error::ApplicationResult,
    input_ports::VerifyLedgerConsistencyUseCase,
};

pub struct VerifyLedgerConsistencyInteractor;

impl Default for VerifyLedgerConsistencyInteractor {
    fn default() -> Self {
        Self::new()
    }
}

impl VerifyLedgerConsistencyInteractor {
    pub fn new() -> Self {
        Self
    }

    /// 差異をDTOに変換
    fn convert_discrepancies(
        discrepancies: &[javelin_domain::financial_close::ledger::Discrepancy],
    ) -> Vec<DiscrepancyDetail> {
        discrepancies
            .iter()
            .map(|disc| DiscrepancyDetail {
                account_code: disc.account_code.clone(),
                account_name: format!("勘定科目{}", disc.account_code),
                subsidiary_balance: disc.subsidiary_total.to_i64().unwrap_or(0),
                general_ledger_balance: disc.gl_balance.to_i64().unwrap_or(0),
                difference: disc.difference.to_i64().unwrap_or(0),
            })
            .collect()
    }

    /// 残高変動をDTOに変換
    fn convert_balance_changes(
        changes: &[javelin_domain::financial_close::ledger::BalanceChange],
    ) -> Vec<BalanceChange> {
        changes
            .iter()
            .map(|bc| BalanceChange {
                account_code: bc.account_code.clone(),
                account_name: format!("勘定科目{}", bc.account_code),
                previous_balance: bc.previous_balance.to_i64().unwrap_or(0),
                current_balance: bc.current_balance.to_i64().unwrap_or(0),
                change_amount: bc.change_amount.to_i64().unwrap_or(0),
                change_rate: bc.change_rate,
            })
            .collect()
    }

    /// 異常値アラートを生成
    fn generate_anomaly_alerts(
        changes: &[javelin_domain::financial_close::ledger::BalanceChange],
    ) -> Vec<AnomalyAlert> {
        let mut alerts = Vec::new();

        for change in changes {
            // 変動率が50%以上の場合は警告
            if change.change_rate.abs() > 50.0 {
                let severity = if change.change_rate.abs() > 100.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                };

                alerts.push(AnomalyAlert {
                    alert_type: "LargeBalanceChange".to_string(),
                    severity,
                    account_code: change.account_code.clone(),
                    message: format!("残高が{}%変動しました", change.change_rate),
                    details: format!("前週末比で{}%の変動を検出", change.change_rate),
                });
            }
        }

        alerts
    }

    /// 仮勘定残高を抽出
    fn extract_temporary_accounts(gl: &GeneralLedger) -> Vec<TemporaryAccountBalance> {
        let temporary_account_prefixes = ["999", "998", "997"]; // 仮勘定のプレフィックス

        gl.get_all_account_codes()
            .iter()
            .filter(|code| temporary_account_prefixes.iter().any(|prefix| code.starts_with(prefix)))
            .filter_map(|code| {
                let balance = gl.get_balance(code);
                if !balance.is_zero() {
                    Some(TemporaryAccountBalance {
                        account_code: code.clone(),
                        account_name: format!("仮勘定{}", code),
                        balance: balance.to_i64().unwrap_or(0),
                        days_outstanding: 30, // 実際にはエントリの日付から計算
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl VerifyLedgerConsistencyUseCase for VerifyLedgerConsistencyInteractor {
    async fn execute(
        &self,
        request: VerifyLedgerConsistencyRequest,
    ) -> ApplicationResult<VerifyLedgerConsistencyResponse> {
        // サンプルデータで元帳を作成（実際にはリポジトリから取得）
        let general_ledger = GeneralLedger::new();
        let subsidiary_ledger = SubsidiaryLedger::new(
            SubsidiaryLedgerType::ProfitAndLoss,
            "ALL".to_string(),
            "全勘定".to_string(),
        )?;

        // 基本検証: 補助元帳と総勘定元帳の整合性確認
        let consistency_report = LedgerService::verify_subsidiary_gl_consistency(
            &[&subsidiary_ledger],
            &general_ledger,
        )?;

        let discrepancies = Self::convert_discrepancies(consistency_report.discrepancies());
        let is_consistent = consistency_report.is_consistent();

        // 詳細検証: 前週末残高との比較
        let balance_changes = if matches!(
            request.verification_level,
            VerificationLevel::Detailed | VerificationLevel::Comprehensive
        ) && request.compare_with_previous_week
        {
            // 前週末残高を取得（サンプル）
            let previous_gl = GeneralLedger::new();

            let changes = LedgerService::analyze_balance_changes(&general_ledger, &previous_gl);

            Some(Self::convert_balance_changes(&changes))
        } else {
            None
        };

        // 包括的検証: 異常値検出と仮勘定分析
        let (anomaly_alerts, temporary_accounts) =
            if matches!(request.verification_level, VerificationLevel::Comprehensive) {
                let alerts = if request.detect_anomalies && balance_changes.is_some() {
                    // 残高変動から異常値を検出
                    let previous_gl = GeneralLedger::new();
                    let changes =
                        LedgerService::analyze_balance_changes(&general_ledger, &previous_gl);

                    Some(Self::generate_anomaly_alerts(&changes))
                } else {
                    None
                };

                let temp_accounts = Some(Self::extract_temporary_accounts(&general_ledger));

                (alerts, temp_accounts)
            } else {
                (None, None)
            };

        // レスポンスを構築
        let response = VerifyLedgerConsistencyResponse {
            verification_id: uuid::Uuid::new_v4().to_string(),
            verified_at: Utc::now(),
            is_consistent,
            discrepancy_count: discrepancies.len(),
            discrepancies,
            balance_changes,
            anomaly_alerts,
            temporary_accounts,
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_verify_ledger_consistency_basic() {
        let interactor = VerifyLedgerConsistencyInteractor::new();

        let request = VerifyLedgerConsistencyRequest {
            period_start: Utc::now(),
            period_end: Utc::now(),
            verification_level: VerificationLevel::Basic,
            compare_with_previous_week: false,
            detect_anomalies: false,
        };

        let response = interactor.execute(request).await.unwrap();

        assert!(!response.verification_id.is_empty());
        assert!(response.balance_changes.is_none());
        assert!(response.anomaly_alerts.is_none());
    }

    #[tokio::test]
    async fn test_verify_ledger_consistency_comprehensive() {
        let interactor = VerifyLedgerConsistencyInteractor::new();

        let request = VerifyLedgerConsistencyRequest {
            period_start: Utc::now(),
            period_end: Utc::now(),
            verification_level: VerificationLevel::Comprehensive,
            compare_with_previous_week: true,
            detect_anomalies: true,
        };

        let response = interactor.execute(request).await.unwrap();

        assert!(!response.verification_id.is_empty());
        // 包括的検証では全ての分析結果が含まれる
        assert!(response.balance_changes.is_some());
        assert!(response.temporary_accounts.is_some());
    }
}
