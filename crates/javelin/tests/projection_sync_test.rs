// Projection同期テスト
// 目的: EventStoreとProjectionDBの同期が正しく動作することを検証

use std::sync::Arc;

use javelin_application::{
    dtos::{
        request::{JournalEntryLineDto, RegisterJournalEntryRequest},
        response::RegisterJournalEntryResponse,
    },
    input_ports::RegisterJournalEntryUseCase,
    interactor::RegisterJournalEntryInteractor,
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
struct TestPresenter;

impl javelin_application::output_ports::JournalEntryOutputPort for TestPresenter {
    async fn present_register_result(&self, _response: RegisterJournalEntryResponse) {}
    async fn notify_progress(&self, _message: String) {}
    async fn notify_error(&self, _error_message: String) {}
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
async fn test_event_store_to_projection_sync() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(TestPresenter),
        Arc::clone(&search_query_service),
    );

    // EventStoreの初期シーケンス
    let initial_sequence = event_store.get_latest_sequence().await.map(|s| s.as_u64()).unwrap_or(0);

    // 仕訳を登録
    let journal_entry = RegisterJournalEntryRequest {
        transaction_date: "2024-04-01".to_string(),
        voucher_number: "V-SYNC-001".to_string(),
        user_id: "test_user".to_string(),
        lines: vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1100".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 75000.0,
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
                amount: 75000.0,
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

    // EventStoreのシーケンスが増えたことを確認
    let after_sequence = event_store.get_latest_sequence().await.map(|s| s.as_u64()).unwrap_or(0);
    assert!(
        after_sequence > initial_sequence,
        "EventStore sequence should increase after registration"
    );

    // 非同期イベント通知を待つ
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Projectionを明示的に再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // ProjectionDBのポジションを確認
    let projection_position = projection_db
        .get_position("default", 1)
        .await
        .expect("Get position should succeed");

    assert_eq!(
        projection_position, after_sequence,
        "Projection position should match EventStore sequence"
    );
}

#[tokio::test]
async fn test_projection_rebuild_idempotency() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(TestPresenter),
        Arc::clone(&search_query_service),
    );

    // 仕訳を登録
    let journal_entry = RegisterJournalEntryRequest {
        transaction_date: "2024-05-01".to_string(),
        voucher_number: "V-IDEMPOTENT-001".to_string(),
        user_id: "test_user".to_string(),
        lines: vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1100".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 30000.0,
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
                amount: 30000.0,
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

    // 1回目のProjection再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("First rebuild should succeed");

    let entries_after_first =
        projection_db.scan_prefix("journal_entry:").await.expect("Scan should succeed");
    let count_after_first = entries_after_first.len();

    // 2回目のProjection再構築（冪等性確認）
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Second rebuild should succeed");

    let entries_after_second =
        projection_db.scan_prefix("journal_entry:").await.expect("Scan should succeed");
    let count_after_second = entries_after_second.len();

    assert_eq!(count_after_first, count_after_second, "Projection rebuild should be idempotent");
}

#[tokio::test]
async fn test_multiple_events_projection_consistency() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(TestPresenter),
        Arc::clone(&search_query_service),
    );

    // 複数の仕訳を連続登録
    for i in 1..=10 {
        let journal_entry = RegisterJournalEntryRequest {
            transaction_date: format!("2024-06-{:02}", i),
            voucher_number: format!("V-MULTI-{:03}", i),
            user_id: "test_user".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1100".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: (i as f64) * 1000.0,
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
                    amount: (i as f64) * 1000.0,
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

    // EventStoreのイベント数を確認
    let events = event_store.get_all_events(0).await.expect("Get all events should succeed");
    let event_count = events.len();

    // Projection再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // ProjectionDBのエントリ数を確認
    let entries = projection_db.scan_prefix("journal_entry:").await.expect("Scan should succeed");
    let projection_count = entries.len();

    assert_eq!(event_count, projection_count, "Projection count should match event count");
}

#[tokio::test]
async fn test_projection_position_tracking() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 初期ポジション
    let initial_position = projection_db
        .get_position("default", 1)
        .await
        .expect("Get position should succeed");

    // 登録Interactor
    let search_query_service =
        Arc::new(JournalEntrySearchQueryServiceImpl::new(Arc::clone(&projection_db)));
    let register_interactor = RegisterJournalEntryInteractor::new(
        Arc::clone(&event_store),
        Arc::new(TestPresenter),
        Arc::clone(&search_query_service),
    );

    // 仕訳を登録
    let journal_entry = RegisterJournalEntryRequest {
        transaction_date: "2024-07-01".to_string(),
        voucher_number: "V-POS-001".to_string(),
        user_id: "test_user".to_string(),
        lines: vec![
            JournalEntryLineDto {
                line_number: 1,
                side: "Debit".to_string(),
                account_code: "1100".to_string(),
                sub_account_code: None,
                department_code: None,
                amount: 20000.0,
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
                amount: 20000.0,
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

    // Projection再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // 更新後のポジション
    let updated_position = projection_db
        .get_position("default", 1)
        .await
        .expect("Get position should succeed");

    assert!(
        updated_position > initial_position,
        "Projection position should advance after rebuild"
    );
}
