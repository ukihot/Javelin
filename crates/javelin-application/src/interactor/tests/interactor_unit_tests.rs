/// Unit tests for Interactors
///
/// Task 9.10: Interactorのユニットテストを作成
/// - 正常な仕訳登録フロー
/// - バリデーションエラー
/// - EventStore保存失敗
///
/// Requirements: 1.1, 1.9

#[cfg(test)]
mod interactor_unit_tests {
    use std::sync::Arc;

    use mockall::predicate::*;

    use crate::{
        dtos::{JournalEntryLineDto, RegisterJournalEntryRequest},
        input_ports::RegisterJournalEntryUseCase,
        interactor::RegisterJournalEntryInteractor,
    };

    #[tokio::test]
    async fn test_successful_journal_entry_registration() {
        // 正常な仕訳登録フロー
        let mut mock_repo = crate::output_ports::journal_entry::MockEventRepository::new();
        mock_repo
            .expect_append_events()
            .times(1)
            .returning(|_, events: Vec<serde_json::Value>| Ok(events.len() as u64));

        let mut mock_output = crate::output_ports::journal_entry::MockJournalEntryOutputPort::new();
        mock_output.expect_present_register_result().times(1).returning(|_| ());

        let mut mock_voucher = crate::input_ports::journal_entry::MockVoucherNumberDomainService::new();
        mock_voucher
            .expect_generate_next()
            .with(eq(2024))
            .times(1)
            .returning(|_| Ok("V-2024-00001".to_string()));

        let interactor = RegisterJournalEntryInteractor::new(
            Arc::new(mock_repo),
            Arc::new(mock_output),
            Arc::new(mock_voucher),
        );

        let request = RegisterJournalEntryRequest {
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result: crate::error::ApplicationResult<()> = interactor.execute(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validation_error_invalid_date() {
        // バリデーションエラー: 無効な日付形式
        let mock_repo = crate::output_ports::journal_entry::MockEventRepository::new();
        let mut mock_output = crate::output_ports::journal_entry::MockJournalEntryOutputPort::new();
        mock_output.expect_notify_error().times(1).returning(|_| ());

        let mock_voucher = crate::input_ports::journal_entry::MockVoucherNumberDomainService::new();

        let interactor = RegisterJournalEntryInteractor::new(
            Arc::new(mock_repo),
            Arc::new(mock_output),
            Arc::new(mock_voucher),
        );

        let request = RegisterJournalEntryRequest {
            transaction_date: "invalid-date".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result: crate::error::ApplicationResult<()> = interactor.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(crate::error::ApplicationError::ValidationFailed(_))));
    }

    #[tokio::test]
    async fn test_validation_error_unbalanced_entry() {
        // バリデーションエラー: 借貸不一致
        let mock_repo = crate::output_ports::journal_entry::MockEventRepository::new();
        let mut mock_output = crate::output_ports::journal_entry::MockJournalEntryOutputPort::new();
        mock_output.expect_notify_error().times(1).returning(|_| ());

        let mut mock_voucher = crate::input_ports::journal_entry::MockVoucherNumberDomainService::new();
        mock_voucher
            .expect_generate_next()
            .returning(|_| Ok("V-2024-00001".to_string()));

        let interactor = RegisterJournalEntryInteractor::new(
            Arc::new(mock_repo),
            Arc::new(mock_output),
            Arc::new(mock_voucher),
        );

        let request = RegisterJournalEntryRequest {
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 50000.0, // 借貸不一致
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result: crate::error::ApplicationResult<()> = interactor.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(crate::error::ApplicationError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_event_store_save_failure() {
        // EventStore保存失敗
        let mut mock_repo = crate::output_ports::journal_entry::MockEventRepository::new();
        mock_repo
            .expect_append_events()
            .times(1)
            .returning(|_, _: Vec<serde_json::Value>| {
                Err(javelin_domain::error::DomainError::RepositoryError("EventStore保存失敗".to_string()))
            });

        let mut mock_output = crate::output_ports::journal_entry::MockJournalEntryOutputPort::new();
        mock_output.expect_notify_error().times(1).returning(|_| ());

        let mut mock_voucher = crate::input_ports::journal_entry::MockVoucherNumberDomainService::new();
        mock_voucher
            .expect_generate_next()
            .returning(|_| Ok("V-2024-00001".to_string()));

        let interactor = RegisterJournalEntryInteractor::new(
            Arc::new(mock_repo),
            Arc::new(mock_output),
            Arc::new(mock_voucher),
        );

        let request = RegisterJournalEntryRequest {
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result: crate::error::ApplicationResult<()> = interactor.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(crate::error::ApplicationError::DomainError(_))));
    }
}
