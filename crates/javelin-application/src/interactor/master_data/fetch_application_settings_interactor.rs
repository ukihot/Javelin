// FetchApplicationSettingsInteractor - アプリケーション設定取得Interactor
// NOTE: ApplicationSettings 集約が削除されたため、このインタラクタは無効化されています

// use crate::{
//     dtos::{
//         request::FetchApplicationSettingsRequest,
//         response::{FetchApplicationSettingsResponse, SystemSettingsDto, UserOptionsDto},
//     },
//     error::ApplicationResult,
//     input_ports::FetchApplicationSettingsInputPort,
//     output_ports::ApplicationSettingsOutputPort,
//     query_service::ApplicationSettingsMasterQueryService,
// };

// /// アプリケーション設定取得Interactor
// ///
// /// CQRS原則: 読み取りはQueryServiceを使用
// pub struct FetchApplicationSettingsInteractor<Q, O>
// where
//     Q: ApplicationSettingsMasterQueryService,
//     O: ApplicationSettingsOutputPort,
// {
//     query_service: std::sync::Arc<Q>,
//     output_port: O,
// }

// impl<Q, O> FetchApplicationSettingsInteractor<Q, O>
// where
//     Q: ApplicationSettingsMasterQueryService,
//     O: ApplicationSettingsOutputPort,
// {
//     pub fn new(query_service: std::sync::Arc<Q>, output_port: O) -> Self {
//         Self { query_service, output_port }
//     }
// }

// #[allow(async_fn_in_trait)]
// impl<Q, O> FetchApplicationSettingsInputPort for FetchApplicationSettingsInteractor<Q, O>
// where
//     Q: ApplicationSettingsMasterQueryService,
//     O: ApplicationSettingsOutputPort,
// {
//     async fn execute(
//         &self,
//         _request: FetchApplicationSettingsRequest,
//     ) -> ApplicationResult<FetchApplicationSettingsResponse> {
//         // QueryServiceから設定を取得
//         let settings = self.query_service.get().await?.ok_or_else(|| {
//             crate::error::ApplicationError::QueryExecutionFailed(
//                 "アプリケーション設定が見つかりません".to_string(),
//             )
//         })?;

//         // DTOに変換
//         let user_options = UserOptionsDto {
//             default_company_code: settings
//                 .default_company_code()
//                 .map(|code| code.value().to_string()),
//             language: settings.language().value().to_string(),
//             decimal_places: settings.decimal_places().value(),
//             date_format: settings.date_format().value().to_string(),
//         };

//         let system_settings = SystemSettingsDto {
//             fiscal_year_start_month: settings.fiscal_year_start_month().value(),
//             closing_day: settings.closing_day().value(),
//             auto_backup_enabled: settings.auto_backup_enabled(),
//             backup_retention_days: settings.backup_retention_days().value(),
//         };

//         let response = FetchApplicationSettingsResponse { user_options, system_settings };

//         // Output Portに通知
//         self.output_port.present_application_settings(&response).await;

//         Ok(response)
//     }
// }
