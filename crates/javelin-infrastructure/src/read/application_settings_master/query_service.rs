// ApplicationSettingsMasterQueryServiceImpl - アプリケーション設定マスタQueryService実装
// DISABLED: ApplicationSettings aggregate has been removed

// use std::sync::Arc;
//
// use javelin_application::{
// error::{ApplicationError, ApplicationResult},
// query_service::ApplicationSettingsMasterQueryService,
// };
// use javelin_domain::chart_of_accounts::ApplicationSettings;
//
// use super::projection::ApplicationSettingsMasterProjection;
// use crate::read::infrastructure::db::ProjectionDb;
//
// アプリケーション設定マスタQueryService実装
//
// CQRS原則: ProjectionDBからアプリケーション設定マスタデータを取得
// pub struct ApplicationSettingsMasterQueryServiceImpl {
// projection: ApplicationSettingsMasterProjection,
// }
//
// impl ApplicationSettingsMasterQueryServiceImpl {
// pub fn new(projection_db: Arc<ProjectionDb>) -> Self {
// Self { projection: ApplicationSettingsMasterProjection::new(projection_db) }
// }
// }
//
// impl ApplicationSettingsMasterQueryService for ApplicationSettingsMasterQueryServiceImpl {
// async fn get(&self) -> ApplicationResult<Option<ApplicationSettings>> {
// self.projection
// .get()
// .await
// .map_err(|e| ApplicationError::QueryExecutionFailed(e.to_string()))
// }
// }
//
// #[cfg(test)]
// mod tests {
// use tempfile::TempDir;
//
// use super::*;
//
// #[tokio::test]
// async fn test_get_none() {
// let temp_dir = TempDir::new().unwrap();
// let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());
// let service = ApplicationSettingsMasterQueryServiceImpl::new(projection_db);
//
// let result = service.get().await.unwrap();
// assert!(result.is_none());
// }
// }
