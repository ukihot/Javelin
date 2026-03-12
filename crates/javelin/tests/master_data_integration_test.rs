// マスタデータ統合テスト
// 目的: マスタデータのCRUD操作とProjection更新の整合性を検証

use std::sync::Arc;

use javelin_application::{
    projection_builder::ProjectionBuilder,
    query_service::{
        AccountMasterQueryService, CompanyMasterQueryService, SubsidiaryAccountMasterQueryService,
    },
};
use javelin_domain::chart_of_accounts::AccountMasterEvent;
use javelin_infrastructure::{
    read::{
        account_master::AccountMasterQueryServiceImpl,
        company_master::CompanyMasterQueryServiceImpl,
        infrastructure::{ProjectionBuilderImpl, ProjectionDb},
        subsidiary_account_master::SubsidiaryAccountMasterQueryServiceImpl,
    },
    write::event_store::EventStore,
};
use tempfile::TempDir;

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

/// 勘定科目マスタイベントを登録
async fn register_account_master(
    event_store: &Arc<EventStore>,
    code: &str,
    name: &str,
    account_type: javelin_domain::chart_of_accounts::AccountType,
) {
    use javelin_domain::event::DomainEvent;

    let event = AccountMasterEvent::AccountMasterCreated {
        code: code.to_string(),
        name: name.to_string(),
        account_type,
        is_active: true,
    };

    let payload = serde_json::to_vec(&event).expect("Serialization should succeed");

    event_store
        .append_event(
            event.event_type(),
            event.aggregate_id(),
            event.version(),
            javelin_infrastructure::shared::types::ExpectedVersion::any(),
            &payload,
        )
        .await
        .expect("Event append should succeed");
}

#[tokio::test]
async fn test_account_master_load_with_scan_prefix() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 勘定科目マスタを登録
    register_account_master(
        &event_store,
        "1100",
        "現金及び現金同等物",
        javelin_domain::chart_of_accounts::AccountType::Asset,
    )
    .await;
    register_account_master(
        &event_store,
        "1110",
        "営業債権",
        javelin_domain::chart_of_accounts::AccountType::Asset,
    )
    .await;
    register_account_master(
        &event_store,
        "4100",
        "売上収益",
        javelin_domain::chart_of_accounts::AccountType::Revenue,
    )
    .await;

    // Projectionを再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // QueryServiceを使って直接取得
    let query_service = AccountMasterQueryServiceImpl::new(Arc::clone(&projection_db));
    let accounts = query_service.get_all().await.expect("Get all should succeed");

    assert_eq!(accounts.len(), 3, "Should load 3 account masters");

    // コードでソートして確認
    let mut codes: Vec<String> = accounts.iter().map(|a| a.code().value().to_string()).collect();
    codes.sort();
    assert_eq!(codes, vec!["1100", "1110", "4100"]);
}

#[tokio::test]
async fn test_projection_db_scan_prefix_consistency() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 勘定科目マスタを登録
    register_account_master(
        &event_store,
        "2100",
        "営業債務",
        javelin_domain::chart_of_accounts::AccountType::Liability,
    )
    .await;
    register_account_master(
        &event_store,
        "2110",
        "社債及び借入金",
        javelin_domain::chart_of_accounts::AccountType::Liability,
    )
    .await;

    // Projectionを再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // ProjectionDBから直接スキャン
    let entries = projection_db.scan_prefix("account_master:").await.expect("Scan should succeed");

    assert_eq!(entries.len(), 2, "Should have 2 account masters in projection");

    // キーフォーマットを確認
    for (key, _value) in &entries {
        assert!(key.starts_with("account_master:"), "Key should start with 'account_master:'");
        // キーは "account_master:{code}" の形式
        assert!(key.len() > "account_master:".len(), "Key should have account code after prefix");
    }
}

#[tokio::test]
async fn test_company_master_empty_list() {
    // インフラストラクチャのセットアップ
    let (_event_store, projection_db, _projection_builder) = setup_infrastructure().await;

    // QueryServiceを使って直接取得
    let query_service = CompanyMasterQueryServiceImpl::new(Arc::clone(&projection_db));
    let companies = query_service.get_all().await.expect("Get all should succeed");

    assert_eq!(companies.len(), 0, "Should have no company masters initially");
}

#[tokio::test]
async fn test_subsidiary_account_master_empty_list() {
    // インフラストラクチャのセットアップ
    let (_event_store, projection_db, _projection_builder) = setup_infrastructure().await;

    // QueryServiceを使って直接取得
    let query_service = SubsidiaryAccountMasterQueryServiceImpl::new(Arc::clone(&projection_db));
    let accounts = query_service.get_all().await.expect("Get all should succeed");

    assert_eq!(accounts.len(), 0, "Should have no subsidiary account masters initially");
}

#[tokio::test]
async fn test_account_master_get_by_code() {
    // インフラストラクチャのセットアップ
    let (event_store, projection_db, projection_builder) = setup_infrastructure().await;

    // 勘定科目マスタを登録
    register_account_master(
        &event_store,
        "3100",
        "資本金",
        javelin_domain::chart_of_accounts::AccountType::Equity,
    )
    .await;

    // Projectionを再構築
    projection_builder
        .rebuild_all_projections()
        .await
        .expect("Projection rebuild should succeed");

    // QueryServiceを使って特定のコードで取得
    let query_service = AccountMasterQueryServiceImpl::new(Arc::clone(&projection_db));
    let code = javelin_domain::chart_of_accounts::AccountCode::new("3100").expect("Valid code");
    let account = query_service.get_by_code(&code).await.expect("Get by code should succeed");

    assert!(account.is_some(), "Should find account with code 3100");
    let account = account.unwrap();
    assert_eq!(account.code().value(), "3100");
    assert_eq!(account.name().value(), "資本金");
}
