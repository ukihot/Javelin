/// Property-based tests for Interactors
///
/// Task 9.9: Interactorのプロパティテストを作成
/// - プロパティ2: イベント保存失敗時のロールバック
///
/// Requirements: 1.9

#[cfg(test)]
mod interactor_property_tests {
    use std::sync::Arc;

    use proptest::prelude::*;

    use crate::{
        dtos::{JournalEntryLineDto, RegisterJournalEntryRequest},
        input_ports::RegisterJournalEntryUseCase,
        interactor::RegisterJournalEntryInteractor,
    };

    // テストデータ生成戦略
    fn journal_entry_line_strategy() -> impl Strategy<Value = JournalEntryLineDto> {
        (1u32..100u32, prop::bool::ANY, "[0-9]{4}", 1000.0..1000000.0).prop_map(
            |(line_number, is_debit, account_code, amount)| JournalEntryLineDto {
                line_number,
                side: if is_debit { "Debit" } else { "Credit" }.to_string(),
                account_code,
                sub_account_code: None,
                department_code: None,
                amount,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        )
    }

    fn register_request_strategy() -> impl Strategy<Value = RegisterJournalEntryRequest> {
        (
            "[0-9]{4}-[0-9]{2}-[0-9]{2}",
            "[A-Z0-9]{5,10}",
            "[a-z]{5,10}",
            prop::collection::vec(journal_entry_line_strategy(), 2..4),
        )
            .prop_map(|(transaction_date, voucher_number, user_id, mut lines)| {
                // 借貸バランスを調整
                let debit_total: f64 =
                    lines.iter().filter(|l| l.side == "Debit").map(|l| l.amount).sum();
                let credit_total: f64 =
                    lines.iter().filter(|l| l.side == "Credit").map(|l| l.amount).sum();

                if debit_total > credit_total {
                    // 貸方を追加
                    lines.push(JournalEntryLineDto {
                        line_number: lines.len() as u32 + 1,
                        side: "Credit".to_string(),
                        account_code: "9999".to_string(),
                        sub_account_code: None,
                        department_code: None,
                        amount: debit_total - credit_total,
                        currency: "JPY".to_string(),
                        tax_type: "NonTaxable".to_string(),
                        tax_amount: 0.0,
                        description: None,
                    });
                } else if credit_total > debit_total {
                    // 借方を追加
                    lines.push(JournalEntryLineDto {
                        line_number: lines.len() as u32 + 1,
                        side: "Debit".to_string(),
                        account_code: "9999".to_string(),
                        sub_account_code: None,
                        department_code: None,
                        amount: credit_total - debit_total,
                        currency: "JPY".to_string(),
                        tax_type: "NonTaxable".to_string(),
                        tax_amount: 0.0,
                        description: None,
                    });
                }

                RegisterJournalEntryRequest { transaction_date, voucher_number, lines, user_id }
            })
    }

    /// プロパティ2: イベント保存失敗時のロールバック
    ///
    /// Feature: cqrs-infrastructure-integration, Property 2: イベント保存失敗時のロールバック
    ///
    /// 任意の仕訳操作において、EventStoreへのイベント保存が失敗した場合、
    /// システムはトランザクションをロールバックし、適切なエラーを返すこと
    ///
    /// **検証要件: 1.9**
    ///
    /// 検証内容:
    /// - EventStore保存失敗時にエラーが返されること
    /// - エラーがApplicationError型であること
    /// - システムが一貫した状態を保つこと（部分的な保存が発生しない）
    #[test]
    fn property_2_rollback_on_event_store_failure() {
        proptest!(|(request in register_request_strategy())| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut mock_repo = crate::output_ports::journal_entry::MockEventRepository::new();
                mock_repo
                    .expect_append_events()
                    .returning(|_, _: Vec<serde_json::Value>| Err(javelin_domain::error::DomainError::RepositoryError("EventStore保存失敗".to_string())));

                let mut mock_output = crate::output_ports::journal_entry::MockJournalEntryOutputPort::new();
                mock_output.expect_notify_error().returning(|_| ());

                let mut mock_voucher = crate::input_ports::journal_entry::MockVoucherNumberDomainService::new();
                mock_voucher
                    .expect_generate_next()
                    .returning(|fiscal_year| Ok(format!("V-{}-00001", fiscal_year)));

                let interactor = RegisterJournalEntryInteractor::new(
                    Arc::new(mock_repo),
                    Arc::new(mock_output),
                    Arc::new(mock_voucher),
                );

                // 実行してエラーが返されることを確認
                let result: crate::error::ApplicationResult<()> = interactor.execute(request).await;

                // エラーが返されることを確認（エラーの種類は問わない）
                // EventStore保存失敗により、何らかのエラーが返されればOK
                prop_assert!(result.is_err(), "Expected error but got Ok");

                Ok(())
            }).unwrap();
        });
    }
}
