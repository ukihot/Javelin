// システム統合テスト
// 目的: クレート間の連携を検証し、認識違いによるバグを防ぐ

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::{JournalEntryLineDto, RegisterJournalEntryRequest, SearchCriteriaDto},
        response::RegisterJournalEntryResponse,
    },
    input_ports::{RegisterJournalEntryUseCase, SearchJournalEntryUseCase},
    interactor::{RegisterJournalEntryInteractor, SearchJournalEntryInteractor},
    projection_builder::ProjectionBuilder,
};
use javelin_infrastructure::{
    read::{
        infrastructure::{ProjectionBuilderImpl, ProjectionDb},
        journal_entry::JournalEntrySearchQueryServiceImpl,
    },
    write::event_store::EventStore,
};
use tempfile::TempDir;

/// テスト用のダミーPresenter
struct TestPresenter {
    results: Arc<tokio::sync::Mutex<Vec<String>>>,
}

impl TestPresenter {
    fn new() -> (Self, Arc<tokio::sync::Mutex<Vec<String>>>) {
        let results = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        (Self { results: Arc::clone(&results) }, results)
    }
}

impl javelin_application::output_ports::JournalEntryOutputPort for TestPresenter {
    async fn present_register_result(&self, response: RegisterJournalEntryResponse) {
        self.results.lock().await.push(format!("registered:{}", response.entry_id));
    }

    async fn notify_progress(&self, _message: String) {}

    async fn notify_error(&self, error: String) {
        self.results.lock().await.push(format!("error:{}", error));
    }

    async fn present_update_draft_result(
        &self,
        _response: javelin_application::dtos::response::UpdateDraftJournalEntryResponse,
    ) {
    }

    async fn present_submit_for_approval_result(
        &self,
        _response: javelin_application::dtos::response::SubmitForApprovalResponse,
    ) {
    }

    async fn present_approve_result(
        &self,
        _response: javelin_application::dtos::response::ApproveJournalEntryResponse,
    ) {
    }

    async fn present_reject_result(
        &self,
        _response: javelin_application::dtos::response::RejectJournalEntryResponse,
    ) {
    }

    async fn present_reverse_result(
        &self,
        _response: javelin_application::dtos::response::ReverseJournalEntryResponse,
    ) {
    }

    async fn present_correct_result(
        &self,
        _response: javelin_application::dtos::response::CorrectJournalEntryResponse,
    ) {
    }

    async fn present_delete_draft_result(
        &self,
        _response: javelin_application::dtos::response::DeleteDraftJournalEntryResponse,
    ) {
    }
}

impl javelin_application::output_ports::SearchOutputPort for TestPresenter {
    fn present_search_result(
        &self,
        result: javelin_application::dtos::response::JournalEntrySearchResultDto,
    ) {
        let results_arc = Arc::clone(&self.results);
        tokio::spawn(async move {
            results_arc
                .lock()
                .await
                .push(format!("search_results:{}", result.entries.len()));
        });
    }

    fn present_validation_error(&self, message: String) {
        let results = Arc::clone(&self.results);
        tokio::spawn(async move {
            results.lock().await.push(format!("validation_error:{}", message));
        });
    }

    fn present_no_results(&self) {
        let results = Arc::clone(&self.results);
        tokio::spawn(async move {
            results.lock().await.push("no_results".to_string());
        });
    }

    fn present_progress(&self, _message: String) {}

    fn present_execution_time(&self, _elapsed_ms: usize) {}

    fn notify_error(&self, error: String) {
        let results = Arc::clone(&self.results);
        tokio::spawn(async move {
            results.lock().await.push(format!("search_error:{}", error));
        });
    }
}

/// テストヘルパー: インフラストラクチャのセットアップ
async fn setup_infrastructure() -> (Arc<EventStore>, Arc<ProjectionDb>, Arc<ProjectionBuilderImpl>)
{
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let data_dir = temp_dir.path().to_path_buf();

    let event_store = Arc::new(
        EventStore::new(&data_dir.join("events"))
            .await
            .expect("Failed to create EventStore"),
    );

    let projection_db = Arc::new(
        ProjectionDb::new(&data_dir.join("projections"))
            .await
            .expect("Failed to create ProjectionDb"),
    );

    let projection_builder =
        Arc::new(ProjectionBuilderImpl::new(Arc::clone(&projection_db), Arc::clone(&event_store)));

    // イベント通知ハンドラを登録
    let (error_sender, _error_receiver) = tokio::sync::mpsc::unbounded_channel();
    let notification_handler =
        projection_builder.clone().create_event_notification_handler(error_sender);
    event_store.set_notification_callback(notification_handler);

    // temp_dirをリークしてテスト終了まで保持
    std::mem::forget(temp_dir);

    (event_store, projection_db, projection_builder)
}

#[tokio::test]
async fn test_journal_entry_registration_and_search() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録用Presenter
    let (register_presenter, register_results) = TestPresenter::new();

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(register_presenter),
        Arc::clone(&search_query_service),
    );

    // 仕訳を登録
    let journal_entry = RegisterJournalEntryRequest {
        transaction_date: "2024-01-15".to_string(),
        voucher_number: "V-001".to_string(),
        user_id: "test_user".to_string(),
        lines: vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1100".to_string(),
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
                account_code: "4100".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 100000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        ],
    };

    register_interactor
        .execute(journal_entry)
        .await
        .expect("Registration should succeed");

    // 登録結果を確認
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let results = register_results.lock().await;
    assert_eq!(results.len(), 1);
    assert!(results[0].starts_with("registered:"));
    drop(results);

    // Projectionを再構築（イベント通知が非同期なので明示的に再構築）
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // 検索用Presenter
    let (search_presenter, search_results) = TestPresenter::new();

    // 検索Interactor
    let search_interactor = SearchJournalEntryInteractor::new(
        Arc::clone(&search_query_service),
        Arc::new(search_presenter),
    );

    // 検索を実行
    let search_criteria = SearchCriteriaDto::new();
    search_interactor.execute(search_criteria).await.expect("Search should succeed");

    // 検索結果を確認
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let results = search_results.lock().await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "search_results:1", "Should find 1 journal entry");
}

#[tokio::test]
async fn test_projection_key_format_consistency() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録用Presenter
    let (register_presenter, _register_results) = TestPresenter::new();

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(register_presenter),
        Arc::clone(&search_query_service),
    );

    // 仕訳を登録
    let journal_entry = RegisterJournalEntryRequest {
        transaction_date: "2024-02-01".to_string(),
        voucher_number: "V-002".to_string(),
        user_id: "test_user".to_string(),
        lines: vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1100".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 50000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
            JournalEntryLineDto {
                line_number: 2,
                side: "Credit".to_string(),
                account_code: "4100".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 50000.0,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        ],
    };

    register_interactor
        .execute(journal_entry)
        .await
        .expect("Registration should succeed");

    // Projectionを再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // ProjectionDBから直接スキャンして、キーフォーマットを確認
    let entries = projection_db.scan_prefix("journal_entry:").await.expect("Scan should succeed");

    assert_eq!(entries.len(), 1, "Should have 1 journal entry in projection");

    // キーフォーマットを確認: "journal_entry:{entry_id}" (UUIDなど)
    let (key, _value) = &entries[0];
    assert!(key.starts_with("journal_entry:"), "Key should start with 'journal_entry:'");
    assert!(!key.contains(":00000000"), "Key should NOT use zero-padded sequential format");
}

#[tokio::test]
async fn test_multiple_entries_search() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録用Presenter
    let (register_presenter, _register_results) = TestPresenter::new();

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(register_presenter),
        Arc::clone(&search_query_service),
    );

    // 複数の仕訳を登録
    for i in 1..=5 {
        let journal_entry = RegisterJournalEntryRequest {
            transaction_date: format!("2024-03-{:02}", i),
            voucher_number: format!("V-{:03}", i),
            user_id: "test_user".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1100".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: (i as f64) * 10000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4100".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: (i as f64) * 10000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
        };

        register_interactor
            .execute(journal_entry)
            .await
            .expect("Registration should succeed");
    }

    // Projectionを再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // 検索用Presenter
    let (search_presenter, search_results) = TestPresenter::new();

    // 検索Interactor
    let search_interactor = SearchJournalEntryInteractor::new(
        Arc::clone(&search_query_service),
        Arc::new(search_presenter),
    );

    // 検索を実行
    let search_criteria = SearchCriteriaDto::new();
    search_interactor.execute(search_criteria).await.expect("Search should succeed");

    // 検索結果を確認
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let results = search_results.lock().await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "search_results:5", "Should find 5 journal entries");
}

#[tokio::test]
async fn test_search_with_date_filter() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録用Presenter
    let (register_presenter, _register_results) = TestPresenter::new();

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(register_presenter),
        Arc::clone(&search_query_service),
    );

    // 異なる日付の仕訳を登録
    let dates = vec!["2024-01-10", "2024-01-20", "2024-02-10", "2024-02-20"];
    for (i, date) in dates.iter().enumerate() {
        let journal_entry = RegisterJournalEntryRequest {
            transaction_date: date.to_string(),
            voucher_number: format!("V-{:03}", i + 1),
            user_id: "test_user".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1100".to_string(),
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
                    account_code: "4100".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 10000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
        };

        register_interactor
            .execute(journal_entry)
            .await
            .expect("Registration should succeed");
    }

    // Projectionを再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // 検索用Presenter
    let (search_presenter, search_results) = TestPresenter::new();

    // 検索Interactor
    let search_interactor = SearchJournalEntryInteractor::new(
        Arc::clone(&search_query_service),
        Arc::new(search_presenter),
    );

    // 2024年1月のみを検索
    let search_criteria = SearchCriteriaDto::new()
        .with_from_date("2024-01-01".to_string())
        .with_to_date("2024-01-31".to_string());

    search_interactor.execute(search_criteria).await.expect("Search should succeed");

    // 検索結果を確認
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let results = search_results.lock().await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "search_results:2", "Should find 2 journal entries in January");
}
