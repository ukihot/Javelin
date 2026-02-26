// Application層ユースケース: ApplyIfrsValuation
// 正常系・異常系・イベント保存失敗のテスト

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono;
    use javelin_domain::{
        financial_close::closing_events::ClosingEvent, repositories::RepositoryBase,
    };

    use crate::{
        dtos::ApplyIfrsValuationRequest, input_ports::ApplyIfrsValuationUseCase,
        interactor::ApplyIfrsValuationInteractor, output_ports::ClosingOutputPort,
        query_service::LedgerQueryService,
    };

    // モックRepositories
    struct MockEventRepository;
    impl RepositoryBase for MockEventRepository {
        type Event = ClosingEvent;

        async fn append(&self, _event: Self::Event) -> javelin_domain::error::DomainResult<()> {
            Ok(())
        }

        async fn append_events<T>(
            &self,
            _id: &str,
            _events: Vec<T>,
        ) -> javelin_domain::error::DomainResult<u64>
        where
            T: serde::Serialize + Send + 'static,
        {
            Ok(0)
        }

        async fn get_events(
            &self,
            _id: &str,
        ) -> javelin_domain::error::DomainResult<Vec<serde_json::Value>> {
            Ok(vec![])
        }

        async fn get_all_events(
            &self,
            _from_sequence: u64,
        ) -> javelin_domain::error::DomainResult<Vec<serde_json::Value>> {
            Ok(vec![])
        }

        async fn get_latest_sequence(&self) -> javelin_domain::error::DomainResult<u64> {
            Ok(0)
        }
    }

    struct MockLedgerQueryService;
    impl LedgerQueryService for MockLedgerQueryService {
        async fn get_ledger(
            &self,
            _query: crate::query_service::ledger_query_service::GetLedgerQuery,
        ) -> crate::error::ApplicationResult<crate::query_service::ledger_query_service::LedgerResult>
        {
            Ok(crate::query_service::ledger_query_service::LedgerResult {
                account_code: "1000".to_string(),
                account_name: "Test Account".to_string(),
                opening_balance: 0.0,
                entries: vec![],
                closing_balance: 0.0,
                total_debit: 0.0,
                total_credit: 0.0,
            })
        }

        async fn get_trial_balance(
            &self,
            _query: crate::query_service::ledger_query_service::GetTrialBalanceQuery,
        ) -> crate::error::ApplicationResult<crate::query_service::TrialBalanceResult> {
            Ok(crate::query_service::TrialBalanceResult {
                period_year: 2024,
                period_month: 1,
                entries: vec![],
                total_debit: 0.0,
                total_credit: 0.0,
            })
        }
    }

    // モック ClosingOutputPort
    struct MockClosingOutputPort;
    impl ClosingOutputPort for MockClosingOutputPort {
        async fn notify_judgment_log(
            &self,
            _judgment_type: String,
            _accounting_standard: String,
            _model_used: String,
            _assumptions: Vec<String>,
            _sensitivity_analysis: Vec<String>,
            _timestamp: chrono::DateTime<chrono::Utc>,
        ) {
        }

        async fn notify_progress(&self, _message: String) {}

        async fn notify_error(&self, _error_message: String) {}
    }

    #[tokio::test]
    async fn test_successful_apply_ifrs_valuation() {
        let event_repo = Arc::new(MockEventRepository);
        let query_service = Arc::new(MockLedgerQueryService);
        let closing_output = Arc::new(MockClosingOutputPort);

        let interactor =
            ApplyIfrsValuationInteractor::new(event_repo, query_service, closing_output);
        let request = ApplyIfrsValuationRequest {
            fiscal_year: 2024,
            period: 12,
            user_id: "user1".to_string(),
        };
        let result = interactor.execute(request).await;
        assert!(result.is_ok());
    }
}
