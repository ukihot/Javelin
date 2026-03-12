// EvaluateMaterialityInteractor - 重要性評価処理
// 責務: 金額的重要性判定・質的重要性判定

use std::sync::Arc;

use crate::{
    dtos::{EvaluateMaterialityRequest, EvaluateMaterialityResponse},
    error::ApplicationResult,
    input_ports::EvaluateMaterialityUseCase,
    query_service::ledger_query_service::LedgerQueryService,
};

pub struct EvaluateMaterialityInteractor<Q>
where
    Q: LedgerQueryService,
{
    ledger_query_service: Arc<Q>,
}

impl<Q> EvaluateMaterialityInteractor<Q>
where
    Q: LedgerQueryService,
{
    pub fn new(ledger_query_service: Arc<Q>) -> Self {
        Self { ledger_query_service }
    }
}

impl<Q> EvaluateMaterialityUseCase for EvaluateMaterialityInteractor<Q>
where
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        _request: EvaluateMaterialityRequest,
    ) -> ApplicationResult<EvaluateMaterialityResponse> {
        // TODO: 重要性評価処理の実装
        // 1. 金額的重要性の判定
        // 2. 質的重要性の判定
        // 3. 承認レベルの決定

        Ok(EvaluateMaterialityResponse {
            judgment_id: format!("MAT-{}", chrono::Utc::now().timestamp()),
            is_material: false,
            approval_level: crate::dtos::response::materiality_evaluation::ApprovalLevel::Manager,
            applied_threshold: crate::dtos::response::materiality_evaluation::ThresholdInfo {
                threshold_type: "Quantitative".to_string(),
                threshold_amount: 0,
                base_metric: "TotalAssets".to_string(),
            },
            threshold_excess_rate: None,
            qualitative_materiality: None,
            judgment_reason: "Automated evaluation".to_string(),
            judgment_date: chrono::Utc::now(),
        })
    }
}
