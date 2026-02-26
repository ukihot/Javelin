// LedgerQueryService プロパティテスト
// プロパティ11: 元帳照会の正確性
// プロパティ12: 試算表照会の正確性
// プロパティ13: 元帳・試算表データ構造の完全性
// プロパティ9: フィルタリングの正確性（元帳用）

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use javelin_application::query_service::ledger_query_service::{
        GetLedgerQuery, GetTrialBalanceQuery, LedgerQueryService,
    };
    use javelin_domain::{
        financial_close::journal_entry::events::{JournalEntryEvent, JournalEntryLineDto},
        repositories::RepositoryBase,
    };
    use proptest::prelude::*;
    use tempfile::TempDir;

    use crate::{read::queries::LedgerQueryServiceImpl, write::event_store::EventStore};

    // テストデータ生成用の戦略

    /// 記帳済イベントを生成
    fn create_posted_event(
        entry_id: String,
        entry_number: String,
        transaction_date: String,
        lines: Vec<JournalEntryLineDto>,
    ) -> (JournalEntryEvent, JournalEntryEvent) {
        let draft_event = JournalEntryEvent::DraftCreated {
            entry_id: entry_id.clone(),
            transaction_date,
            voucher_number: format!("V-{}", entry_number),
            lines,
            created_by: "test_user".to_string(),
            created_at: chrono::Utc::now(),
        };

        let posted_event = JournalEntryEvent::Posted {
            entry_id,
            entry_number,
            posted_by: "approver".to_string(),
            posted_at: chrono::Utc::now(),
        };

        (draft_event, posted_event)
    }

    proptest! {
        /// プロパティ11: 元帳照会の正確性
        /// 検証要件: 4.1, 4.7
        ///
        /// EventStoreに保存されたイベントから元帳データを正確に取得できることを検証
        #[test]
        fn prop_ledger_query_accuracy(
            account_code in "[0-9]{4}",
            num_entries in 1usize..5usize,
        ) {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());

                // テストデータを作成してEventStoreに保存
                let mut expected_debit = 0.0;
                let expected_credit = 0.0;

                for i in 0..num_entries {
                    let entry_id = format!("JE{:03}", i);
                    let entry_number = format!("EN-2024-{:03}", i);
                    let transaction_date = "2024-01-15".to_string();

                    // 借方と貸方の明細を作成（バランスを保つ）
                    let amount = 10000.0 * (i + 1) as f64;
                    let lines = vec![
                        JournalEntryLineDto {
                            line_number: 1,
                            side: "Debit".to_string(),
                            account_code: account_code.clone(),
                            sub_account_code: None,
                            department_code: None,
                            amount,
                            currency: "JPY".to_string(),
                            tax_type: "NonTaxable".to_string(),
                            tax_amount: 0.0,
                            description: None,
                        },
                        JournalEntryLineDto {
                            line_number: 2,
                            side: "Credit".to_string(),
                            account_code: "9999".to_string(),
                            sub_account_code: None,
                            department_code: None,
                            amount,
                            currency: "JPY".to_string(),
                            tax_type: "NonTaxable".to_string(),
                            tax_amount: 0.0,
                            description: None,
                        },
                    ];

                    expected_debit += amount;

                    let (draft_event, posted_event) = create_posted_event(
                        entry_id.clone(),
                        entry_number,
                        transaction_date,
                        lines,
                    );

                    // イベントを保存
                    RepositoryBase::append_events(&*event_store, &entry_id, vec![draft_event, posted_event]).await.unwrap();
                }

                // LedgerQueryServiceで取得
                let service = LedgerQueryServiceImpl::new(event_store);
                let query = GetLedgerQuery {
                    account_code: account_code.clone(),
                    from_date: Some("2024-01-01".to_string()),
                    to_date: Some("2024-01-31".to_string()),
                    limit: None,
                    offset: None,
                };

                let result = service.get_ledger(query).await.unwrap();

                // 検証
                prop_assert_eq!(&result.account_code, &account_code);
                prop_assert_eq!(result.entries.len(), num_entries);

                // 借方合計の検証
                prop_assert!((result.total_debit - expected_debit).abs() < 0.01);
                prop_assert!((result.total_credit - expected_credit).abs() < 0.01);

                Ok::<(), proptest::test_runner::TestCaseError>(())
            })?
        }

        /// プロパティ12: 試算表照会の正確性
        /// 検証要件: 4.4
        ///
        /// EventStoreに保存されたイベントから試算表データを正確に取得できることを検証
        #[test]
        fn prop_trial_balance_query_accuracy(
            num_accounts in 2usize..5usize,
        ) {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());

                let period_year = 2024u32;
                let period_month = 1u8;

                // 複数の勘定科目に対してイベントを作成
                let mut expected_total_debit = 0.0;
                let mut expected_total_credit = 0.0;

                for i in 0..num_accounts {
                    let entry_id = format!("JE{:03}", i);
                    let entry_number = format!("EN-2024-{:03}", i);
                    let account_code = format!("{:04}", 1000 + i);
                    let amount = 10000.0 * (i + 1) as f64;

                    let lines = vec![
                        JournalEntryLineDto {
                            line_number: 1,
                            side: "Debit".to_string(),
                            account_code: account_code.clone(),
                            sub_account_code: None,
                            department_code: None,
                            amount,
                            currency: "JPY".to_string(),
                            tax_type: "NonTaxable".to_string(),
                            tax_amount: 0.0,
                            description: None,
                        },
                        JournalEntryLineDto {
                            line_number: 2,
                            side: "Credit".to_string(),
                            account_code: "9999".to_string(),
                            sub_account_code: None,
                            department_code: None,
                            amount,
                            currency: "JPY".to_string(),
                            tax_type: "NonTaxable".to_string(),
                            tax_amount: 0.0,
                            description: None,
                        },
                    ];

                    expected_total_debit += amount;
                    expected_total_credit += amount;

                    let (draft_event, posted_event) = create_posted_event(
                        entry_id.clone(),
                        entry_number,
                        format!("{:04}-{:02}-15", period_year, period_month),
                        lines,
                    );

                    RepositoryBase::append_events(&*event_store, &entry_id, vec![draft_event, posted_event]).await.unwrap();
                }

                // LedgerQueryServiceで取得
                let service = LedgerQueryServiceImpl::new(event_store);
                let query = GetTrialBalanceQuery { period_year, period_month };

                let result = service.get_trial_balance(query).await.unwrap();

                // 検証
                prop_assert_eq!(result.period_year, period_year);
                prop_assert_eq!(result.period_month, period_month);
                prop_assert!(result.entries.len() >= num_accounts); // 9999も含まれる

                // 借方合計・貸方合計の検証
                prop_assert!((result.total_debit - expected_total_debit).abs() < 0.01);
                prop_assert!((result.total_credit - expected_total_credit).abs() < 0.01);

                Ok::<(), proptest::test_runner::TestCaseError>(())
            })?
        }

        /// プロパティ13: 元帳・試算表データ構造の完全性
        /// 検証要件: 4.5, 4.6
        ///
        /// 取得したデータ構造が完全であることを検証
        #[test]
        fn prop_data_structure_completeness(
            account_code in "[0-9]{4}",
        ) {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());

                // テストデータを作成
                let entry_id = "JE001".to_string();
                let entry_number = "EN-2024-001".to_string();
                let lines = vec![
                    JournalEntryLineDto {
                        line_number: 1,
                        side: "Debit".to_string(),
                        account_code: account_code.clone(),
                        sub_account_code: None,
                        department_code: None,
                        amount: 10000.0,
                        currency: "JPY".to_string(),
                        tax_type: "NonTaxable".to_string(),
                        tax_amount: 0.0,
                        description: None,
                    },
                    JournalEntryLineDto {
                        line_number: 2,
                        side: "Credit".to_string(),
                        account_code: "9999".to_string(),
                        sub_account_code: None,
                        department_code: None,
                        amount: 10000.0,
                        currency: "JPY".to_string(),
                        tax_type: "NonTaxable".to_string(),
                        tax_amount: 0.0,
                        description: None,
                    },
                ];

                let (draft_event, posted_event) = create_posted_event(
                    entry_id.clone(),
                    entry_number,
                    "2024-01-15".to_string(),
                    lines,
                );

                RepositoryBase::append_events(&*event_store, &entry_id, vec![draft_event, posted_event]).await.unwrap();

                // LedgerQueryServiceで取得
                let service = LedgerQueryServiceImpl::new(event_store);
                let query = GetLedgerQuery {
                    account_code: account_code.clone(),
                    from_date: Some("2024-01-01".to_string()),
                    to_date: None,
                    limit: None,
                    offset: None,
                };

                let result = service.get_ledger(query).await.unwrap();

                // データ構造の完全性を検証
                prop_assert!(!result.account_code.is_empty());
                prop_assert!(!result.account_name.is_empty());

                // 各エントリの完全性を検証
                for entry in &result.entries {
                    prop_assert!(!entry.transaction_date.is_empty());
                    prop_assert!(!entry.entry_number.is_empty());
                    prop_assert!(!entry.entry_id.is_empty());
                    prop_assert!(entry.debit_amount >= 0.0);
                    prop_assert!(entry.credit_amount >= 0.0);
                }

                Ok::<(), proptest::test_runner::TestCaseError>(())
            })?
        }

        /// プロパティ9: フィルタリングの正確性（元帳用）
        /// 検証要件: 4.2, 4.3
        ///
        /// ページネーション（limit/offset）が正確に動作することを検証
        #[test]
        fn prop_ledger_filtering_accuracy(
            account_code in "[0-9]{4}",
            num_entries in 10usize..15usize,
            limit in 1usize..5usize,
            offset in 0usize..3usize,
        ) {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let temp_dir = TempDir::new().unwrap();
                let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());

                // テストデータを作成
                for i in 0..num_entries {
                    let entry_id = format!("JE{:03}", i);
                    let entry_number = format!("EN-2024-{:03}", i);
                    let lines = vec![
                        JournalEntryLineDto {
                            line_number: 1,
                            side: "Debit".to_string(),
                            account_code: account_code.clone(),
                            sub_account_code: None,
                            department_code: None,
                            amount: 10000.0,
                            currency: "JPY".to_string(),
                            tax_type: "NonTaxable".to_string(),
                            tax_amount: 0.0,
                            description: None,
                        },
                        JournalEntryLineDto {
                            line_number: 2,
                            side: "Credit".to_string(),
                            account_code: "9999".to_string(),
                            sub_account_code: None,
                            department_code: None,
                            amount: 10000.0,
                            currency: "JPY".to_string(),
                            tax_type: "NonTaxable".to_string(),
                            tax_amount: 0.0,
                            description: None,
                        },
                    ];

                    let (draft_event, posted_event) = create_posted_event(
                        entry_id.clone(),
                        entry_number,
                        "2024-01-15".to_string(),
                        lines,
                    );

                    RepositoryBase::append_events(&*event_store, &entry_id, vec![draft_event, posted_event]).await.unwrap();
                }

                // LedgerQueryServiceで取得（ページネーション適用）
                let service = LedgerQueryServiceImpl::new(event_store);
                let query = GetLedgerQuery {
                    account_code: account_code.clone(),
                    from_date: Some("2024-01-01".to_string()),
                    to_date: None,
                    limit: Some(limit as u32),
                    offset: Some(offset as u32),
                };

                let result = service.get_ledger(query).await.unwrap();

                // ページネーションの検証
                let expected_count = std::cmp::min(limit, num_entries.saturating_sub(offset));
                prop_assert_eq!(result.entries.len(), expected_count);

                Ok::<(), proptest::test_runner::TestCaseError>(())
            })?
        }
    }

    // 単体テスト
    #[tokio::test]
    async fn test_ledger_query_simple() {
        let temp_dir = TempDir::new().unwrap();
        let event_store = Arc::new(EventStore::new(temp_dir.path()).await.unwrap());

        // テストデータを作成
        let entry_id = "JE001".to_string();
        let entry_number = "EN-2024-001".to_string();
        let account_code = "1000".to_string();
        let lines = vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: account_code.clone(),
                sub_account_code: None,
                department_code: None,
                amount: 10000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
            JournalEntryLineDto {
                line_number: 2,
                side: "Credit".to_string(),
                account_code: "9999".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 10000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        ];

        let (draft_event, posted_event) =
            create_posted_event(entry_id.clone(), entry_number, "2024-01-15".to_string(), lines);

        // イベントを保存
        RepositoryBase::append_events(&*event_store, &entry_id, vec![draft_event, posted_event])
            .await
            .unwrap();

        // LedgerQueryServiceで取得
        let service = LedgerQueryServiceImpl::new(event_store);
        let query = GetLedgerQuery {
            account_code: account_code.clone(),
            from_date: Some("2024-01-01".to_string()),
            to_date: Some("2024-01-31".to_string()),
            limit: None,
            offset: None,
        };

        let result = service.get_ledger(query).await.unwrap();

        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.account_code, account_code);
    }
}
