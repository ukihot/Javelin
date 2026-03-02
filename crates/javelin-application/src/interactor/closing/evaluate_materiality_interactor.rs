// EvaluateMaterialityInteractor - 重要性判定処理
// 責務: ドメイン層の重要性基準サービスを活用した判定実行

use javelin_domain::{
    common::Amount,
    financial_close::materiality::{
        entities::{ApprovalLevel, MaterialityJudgment},
        services::MaterialityService,
        values::{QualitativeFactor, QuantitativeThreshold},
    },
};

use crate::{
    dtos::{
        EvaluateMaterialityRequest, EvaluateMaterialityResponse, ThresholdInfo,
        response::ApprovalLevel as DtoApprovalLevel,
    },
    error::ApplicationResult,
    input_ports::EvaluateMaterialityUseCase,
};

pub struct EvaluateMaterialityInteractor;

impl Default for EvaluateMaterialityInteractor {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluateMaterialityInteractor {
    pub fn new() -> Self {
        Self
    }

    /// ドメインのApprovalLevelをDTOに変換
    fn convert_approval_level(level: &ApprovalLevel) -> DtoApprovalLevel {
        match level {
            ApprovalLevel::Staff => DtoApprovalLevel::Staff,
            ApprovalLevel::Manager => DtoApprovalLevel::Manager,
            ApprovalLevel::Director => DtoApprovalLevel::Director,
            ApprovalLevel::CFO => DtoApprovalLevel::CFO,
            ApprovalLevel::Board => DtoApprovalLevel::Board,
        }
    }
}

impl EvaluateMaterialityUseCase for EvaluateMaterialityInteractor {
    async fn execute(
        &self,
        request: EvaluateMaterialityRequest,
    ) -> ApplicationResult<EvaluateMaterialityResponse> {
        // 財務指標をAmountに変換
        let pretax_income = Amount::from_i64(request.financial_metrics.pretax_income);
        let total_assets = Amount::from_i64(request.financial_metrics.total_assets);
        let revenue = Amount::from_i64(request.financial_metrics.revenue);
        let equity = Amount::from_i64(request.financial_metrics.equity);

        // 金額的重要性基準を計算
        let threshold =
            QuantitativeThreshold::new(&pretax_income, &total_assets, &revenue, &equity);

        // 判定対象金額
        let amount = Amount::from_i64(request.amount);

        // 金額的重要性判定エンティティを作成
        let judgment = MaterialityJudgment::new_quantitative(
            threshold.clone(),
            &amount,
            request.reason.clone(),
            request.judged_by.clone(),
        )?;

        // 金額的重要性判定
        let is_material = judgment.requires_adjustment();
        let approval_level = judgment.approval_level().clone();
        let excess_rate = if is_material {
            Some(MaterialityService::calculate_excess_ratio(
                &amount,
                threshold.lowest_threshold(),
            )?)
        } else {
            None
        };

        // 質的重要性判定（オプション）
        let qualitative_materiality = if let Some(factors) = &request.qualitative_factors {
            // 文字列から質的要因を変換（簡易実装）
            let qualitative_factors: Vec<QualitativeFactor> = factors
                .iter()
                .map(|f| match f.as_str() {
                    "AccountingPolicyChange" => QualitativeFactor::AccountingPolicyChange,
                    "RelatedPartyTransaction" => QualitativeFactor::RelatedPartyTransaction,
                    "LegalViolation" => QualitativeFactor::LegalViolation,
                    "Litigation" => QualitativeFactor::Litigation,
                    "ManagementFraud" => QualitativeFactor::ManagementFraud,
                    "GoingConcernUncertainty" => QualitativeFactor::GoingConcernUncertainty,
                    "SubsequentEvent" => QualitativeFactor::SubsequentEvent,
                    _ => QualitativeFactor::Other(f.clone()),
                })
                .collect();

            if !qualitative_factors.is_empty() {
                let severity =
                    MaterialityService::assess_qualitative_severity(&qualitative_factors);
                Some(severity > 0)
            } else {
                None
            }
        } else {
            None
        };

        // 判定IDを取得
        let judgment_id = judgment.id().clone();

        // レスポンスを構築
        let response = EvaluateMaterialityResponse {
            judgment_id: judgment_id.to_string(),
            is_material,
            approval_level: Self::convert_approval_level(&approval_level),
            applied_threshold: ThresholdInfo {
                threshold_type: "Quantitative".to_string(),
                threshold_amount: threshold.lowest_threshold().to_i64().unwrap_or(0),
                base_metric: "Lowest".to_string(),
            },
            threshold_excess_rate: excess_rate,
            qualitative_materiality,
            judgment_reason: request.reason,
            judgment_date: request.judgment_date,
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[tokio::test]
    async fn test_evaluate_materiality_material() {
        let interactor = EvaluateMaterialityInteractor::new();

        let request = EvaluateMaterialityRequest {
            item_name: "固定資産減損".to_string(),
            amount: 10_000_000, // 1000万円
            judgment_date: Utc::now(),
            reason: "市場価値の著しい下落".to_string(),
            judged_by: "CFO".to_string(),
            financial_metrics: crate::dtos::FinancialMetrics {
                pretax_income: 100_000_000,  // 1億円
                total_assets: 5_000_000_000, // 50億円
                revenue: 3_000_000_000,      // 30億円
                equity: 1_000_000_000,       // 10億円
            },
            qualitative_factors: None,
        };

        let response = interactor.execute(request).await.unwrap();

        // 1000万円は税引前利益1億円の10%なので重要
        assert!(response.is_material);
        assert!(response.threshold_excess_rate.is_some());
    }

    #[tokio::test]
    async fn test_evaluate_materiality_immaterial() {
        let interactor = EvaluateMaterialityInteractor::new();

        let request = EvaluateMaterialityRequest {
            item_name: "事務用品費".to_string(),
            amount: 100_000, // 10万円
            judgment_date: Utc::now(),
            reason: "通常の事務用品購入".to_string(),
            judged_by: "Staff".to_string(),
            financial_metrics: crate::dtos::FinancialMetrics {
                pretax_income: 100_000_000,  // 1億円
                total_assets: 5_000_000_000, // 50億円
                revenue: 3_000_000_000,      // 30億円
                equity: 1_000_000_000,       // 10億円
            },
            qualitative_factors: None,
        };

        let response = interactor.execute(request).await.unwrap();

        // 10万円は閾値未満なので重要でない
        assert!(!response.is_material);
    }
}
