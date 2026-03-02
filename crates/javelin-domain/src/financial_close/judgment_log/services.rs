// 判断ログのドメインサービス

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::{entities::JudgmentLog, values::JudgmentType};
use crate::error::DomainResult;

/// 判断ログドメインサービス
pub struct JudgmentLogService;

impl JudgmentLogService {
    /// 判断ログの完全性を検証
    pub fn verify_completeness(log: &JudgmentLog) -> DomainResult<bool> {
        // 必須項目のチェック
        if log.judgment_basis().is_empty() {
            return Ok(false);
        }

        if log.approver_id().is_empty() {
            return Ok(false);
        }

        // 判断タイプに応じた必須パラメータのチェック
        match log.judgment_type() {
            JudgmentType::ExpectedCreditLoss => {
                // ECLには PD, LGD, EAD が必須
                if !log.parameters().contains_key("PD")
                    || !log.parameters().contains_key("LGD")
                    || !log.parameters().contains_key("EAD")
                {
                    return Ok(false);
                }
            }
            JudgmentType::Impairment => {
                // 減損には discount_rate が必須
                if !log.parameters().contains_key("discount_rate") {
                    return Ok(false);
                }
            }
            JudgmentType::FairValue => {
                // 公正価値測定には measurement_technique が必須
                if !log.parameters().contains_key("measurement_technique") {
                    return Ok(false);
                }
            }
            _ => {}
        }

        Ok(true)
    }

    /// 判断ログの一貫性を検証
    pub fn verify_consistency(
        current_log: &JudgmentLog,
        previous_log: Option<&JudgmentLog>,
    ) -> DomainResult<ConsistencyReport> {
        let mut issues = Vec::new();

        if let Some(prev) = previous_log {
            // 判断タイプが変更されていないか
            if current_log.judgment_type() != prev.judgment_type() {
                issues.push("Judgment type changed".to_string());
            }

            // 主要パラメータの大幅な変更をチェック
            for (key, current_value) in current_log.parameters() {
                if let Some(prev_value) = prev.parameters().get(key)
                    && let (Some(current_f64), Some(prev_f64)) =
                        (current_value.as_f64(), prev_value.as_f64())
                {
                    let change_rate = ((current_f64 - prev_f64) / prev_f64).abs();
                    if change_rate > 0.5 {
                        // 50%以上の変更
                        issues.push(format!(
                            "Large parameter change: {} ({:.1}%)",
                            key,
                            change_rate * 100.0
                        ));
                    }
                }
            }
        }

        Ok(ConsistencyReport { is_consistent: issues.is_empty(), issues })
    }

    /// 判断ログを検索
    pub fn search_logs<'a>(
        logs: &'a [JudgmentLog],
        judgment_type: Option<JudgmentType>,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
        related_entity_id: Option<&str>,
    ) -> Vec<&'a JudgmentLog> {
        logs.iter()
            .filter(|log| {
                // 判断タイプフィルタ
                if let Some(ref jt) = judgment_type
                    && log.judgment_type() != jt
                {
                    return false;
                }

                // 日付範囲フィルタ
                if let Some(from) = date_from
                    && log.judgment_date() < from
                {
                    return false;
                }
                if let Some(to) = date_to
                    && log.judgment_date() > to
                {
                    return false;
                }

                // 関連エンティティフィルタ
                if let Some(entity_id) = related_entity_id
                    && log.related_entity_id() != Some(entity_id)
                {
                    return false;
                }

                true
            })
            .collect()
    }

    /// 判断ログの統計を生成
    pub fn generate_statistics(logs: &[JudgmentLog]) -> JudgmentLogStatistics {
        let mut by_type: HashMap<String, usize> = HashMap::new();
        let mut total_with_sensitivity = 0;
        let mut total_with_scenarios = 0;
        let mut total_assumption_changes = 0;

        for log in logs {
            *by_type.entry(log.judgment_type().as_str().to_string()).or_insert(0) += 1;

            if log.sensitivity_analysis().is_some() {
                total_with_sensitivity += 1;
            }

            if !log.scenarios().is_empty() {
                total_with_scenarios += 1;
            }

            total_assumption_changes += log.assumption_changes().len();
        }

        JudgmentLogStatistics {
            total_logs: logs.len(),
            by_type,
            total_with_sensitivity,
            total_with_scenarios,
            total_assumption_changes,
        }
    }

    /// 期限切れ判断ログを抽出
    pub fn extract_expired_logs(logs: &[JudgmentLog]) -> Vec<&JudgmentLog> {
        logs.iter().filter(|log| log.is_expired()).collect()
    }

    /// 判断ログの品質スコアを計算
    pub fn calculate_quality_score(log: &JudgmentLog) -> f64 {
        let mut score = 0.0;

        // 判断根拠の詳細度（最大30点）
        let basis_length = log.judgment_basis().len();
        score += (basis_length.min(300) as f64 / 300.0) * 30.0;

        // パラメータ数（最大20点）
        let param_count = log.parameters().len();
        score += (param_count.min(10) as f64 / 10.0) * 20.0;

        // シナリオ数（最大20点）
        let scenario_count = log.scenarios().len();
        score += (scenario_count.min(5) as f64 / 5.0) * 20.0;

        // 感度分析の有無（20点）
        if log.sensitivity_analysis().is_some() {
            score += 20.0;
        }

        // 関連エンティティの設定（10点）
        if log.related_entity_id().is_some() {
            score += 10.0;
        }

        score
    }
}

/// 一貫性レポート
#[derive(Debug, Clone)]
pub struct ConsistencyReport {
    pub is_consistent: bool,
    pub issues: Vec<String>,
}

/// 判断ログ統計
#[derive(Debug, Clone)]
pub struct JudgmentLogStatistics {
    pub total_logs: usize,
    pub by_type: HashMap<String, usize>,
    pub total_with_sensitivity: usize,
    pub total_with_scenarios: usize,
    pub total_assumption_changes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::judgment_log::values::{
        JudgmentLogId, ParameterValue, Scenario, SensitivityAnalysis,
    };

    fn create_test_judgment_log() -> JudgmentLog {
        let id = JudgmentLogId::new();
        let mut params = HashMap::new();
        params.insert("PD".to_string(), ParameterValue::Float(0.05));
        params.insert("LGD".to_string(), ParameterValue::Float(0.45));
        params.insert("EAD".to_string(), ParameterValue::Integer(1_000_000));

        let scenario =
            Scenario::new("Base".to_string(), "Base scenario".to_string(), 1.0, HashMap::new())
                .unwrap()
                .with_result(50_000);

        JudgmentLog::new(
            id,
            JudgmentType::ExpectedCreditLoss,
            "ECL calculation based on historical data".to_string(),
            params,
            vec![scenario],
            "USER001".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_verify_completeness_valid() {
        let log = create_test_judgment_log();
        let is_complete = JudgmentLogService::verify_completeness(&log).unwrap();
        assert!(is_complete);
    }

    #[test]
    fn test_verify_completeness_missing_parameter() {
        let id = JudgmentLogId::new();
        let mut params = HashMap::new();
        params.insert("PD".to_string(), ParameterValue::Float(0.05));
        // Missing LGD and EAD

        let log = JudgmentLog::new(
            id,
            JudgmentType::ExpectedCreditLoss,
            "Incomplete ECL calculation".to_string(),
            params,
            Vec::new(),
            "USER001".to_string(),
        )
        .unwrap();

        let is_complete = JudgmentLogService::verify_completeness(&log).unwrap();
        assert!(!is_complete);
    }

    #[test]
    fn test_verify_consistency_no_previous() {
        let log = create_test_judgment_log();
        let report = JudgmentLogService::verify_consistency(&log, None).unwrap();
        assert!(report.is_consistent);
        assert_eq!(report.issues.len(), 0);
    }

    #[test]
    fn test_verify_consistency_with_large_change() {
        let log1 = create_test_judgment_log();

        let id = JudgmentLogId::new();
        let mut params = HashMap::new();
        params.insert("PD".to_string(), ParameterValue::Float(0.15)); // 3x increase
        params.insert("LGD".to_string(), ParameterValue::Float(0.45));
        params.insert("EAD".to_string(), ParameterValue::Integer(1_000_000));

        let log2 = JudgmentLog::new(
            id,
            JudgmentType::ExpectedCreditLoss,
            "Updated ECL calculation".to_string(),
            params,
            Vec::new(),
            "USER001".to_string(),
        )
        .unwrap();

        let report = JudgmentLogService::verify_consistency(&log2, Some(&log1)).unwrap();
        assert!(!report.is_consistent);
        assert!(!report.issues.is_empty());
    }

    #[test]
    fn test_search_logs() {
        let log1 = create_test_judgment_log();

        let id2 = JudgmentLogId::new();
        let log2 = JudgmentLog::new(
            id2,
            JudgmentType::Impairment,
            "Impairment test".to_string(),
            HashMap::new(),
            Vec::new(),
            "USER002".to_string(),
        )
        .unwrap();

        let logs = vec![log1, log2];

        // Search by type
        let results = JudgmentLogService::search_logs(
            &logs,
            Some(JudgmentType::ExpectedCreditLoss),
            None,
            None,
            None,
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_generate_statistics() {
        let log1 = create_test_judgment_log();
        let log2 = create_test_judgment_log();

        let logs = vec![log1, log2];
        let stats = JudgmentLogService::generate_statistics(&logs);

        assert_eq!(stats.total_logs, 2);
        assert_eq!(stats.by_type.get("ExpectedCreditLoss"), Some(&2));
        assert_eq!(stats.total_with_scenarios, 2);
    }

    #[test]
    fn test_extract_expired_logs() {
        let log = create_test_judgment_log();
        let logs = vec![log];

        let expired = JudgmentLogService::extract_expired_logs(&logs);
        assert_eq!(expired.len(), 0); // Should not be expired immediately
    }

    #[test]
    fn test_calculate_quality_score() {
        // Add more parameters to increase score
        let mut params = HashMap::new();
        params.insert("PD".to_string(), ParameterValue::Float(0.05));
        params.insert("LGD".to_string(), ParameterValue::Float(0.45));
        params.insert("EAD".to_string(), ParameterValue::Integer(1_000_000));
        params.insert("discount_rate".to_string(), ParameterValue::Float(0.03));
        params.insert("recovery_rate".to_string(), ParameterValue::Float(0.55));
        params.insert("maturity".to_string(), ParameterValue::Integer(5));
        params.insert("collateral_value".to_string(), ParameterValue::Integer(800_000));
        params.insert("probability_default".to_string(), ParameterValue::Float(0.05));

        // Add multiple scenarios
        let scenario1 =
            Scenario::new("Base".to_string(), "Base scenario".to_string(), 0.6, HashMap::new())
                .unwrap()
                .with_result(50_000);
        let scenario2 = Scenario::new(
            "Optimistic".to_string(),
            "Optimistic scenario".to_string(),
            0.2,
            HashMap::new(),
        )
        .unwrap()
        .with_result(30_000);
        let scenario3 = Scenario::new(
            "Pessimistic".to_string(),
            "Pessimistic scenario".to_string(),
            0.2,
            HashMap::new(),
        )
        .unwrap()
        .with_result(80_000);

        // Create a new log with enhanced data
        let id = JudgmentLogId::new();
        let mut enhanced_log = JudgmentLog::new(
            id,
            JudgmentType::ExpectedCreditLoss,
            "Comprehensive ECL calculation based on historical default data, current economic conditions, and forward-looking information. The analysis incorporates multiple scenarios weighted by probability, with detailed parameter estimation using statistical models and expert judgment. Collateral values have been assessed and recovery rates estimated based on similar exposures.".to_string(),
            params,
            vec![scenario1, scenario2, scenario3],
            "USER001".to_string(),
        )
        .unwrap();

        // Add sensitivity analysis
        let analysis =
            SensitivityAnalysis::new("PD".to_string(), 0.05, 50_000, 0.1, 55_000, 45_000).unwrap();
        enhanced_log.set_sensitivity_analysis(analysis).unwrap();

        // Set related entity
        enhanced_log.set_related_entity("LOAN001".to_string(), "Loan".to_string());

        let score = JudgmentLogService::calculate_quality_score(&enhanced_log);
        // 判断根拠: 300文字以上 = 30点
        // パラメータ: 8個 = 16点
        // シナリオ: 3個 = 12点
        // 感度分析: 20点
        // 関連エンティティ: 10点
        // 合計: 88点
        assert!(score > 80.0);
    }
}
