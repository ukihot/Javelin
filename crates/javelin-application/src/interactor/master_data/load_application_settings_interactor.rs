// LoadApplicationSettingsInteractor - アプリケーション設定取得Interactor

use crate::{
    dtos::{
        request::LoadApplicationSettingsRequest,
        response::{LoadApplicationSettingsResponse, SystemSettingsDto, UserOptionsDto},
    },
    error::ApplicationResult,
    input_ports::LoadApplicationSettingsInputPort,
    output_ports::ApplicationSettingsOutputPort,
    query_service::ApplicationSettingsMasterQueryService,
};

/// アプリケーション設定取得Interactor
pub struct LoadApplicationSettingsInteractor<Q, O>
where
    Q: ApplicationSettingsMasterQueryService,
    O: ApplicationSettingsOutputPort,
{
    query_service: std::sync::Arc<Q>,
    output_port: O,
}

impl<Q, O> LoadApplicationSettingsInteractor<Q, O>
where
    Q: ApplicationSettingsMasterQueryService,
    O: ApplicationSettingsOutputPort,
{
    pub fn new(query_service: std::sync::Arc<Q>, output_port: O) -> Self {
        Self { query_service, output_port }
    }
}

#[allow(async_fn_in_trait)]
impl<Q, O> LoadApplicationSettingsInputPort for LoadApplicationSettingsInteractor<Q, O>
where
    Q: ApplicationSettingsMasterQueryService,
    O: ApplicationSettingsOutputPort,
{
    async fn execute(
        &self,
        _request: LoadApplicationSettingsRequest,
    ) -> ApplicationResult<LoadApplicationSettingsResponse> {
        // QueryServiceから設定を取得
        let settings = self.query_service.get().await?.ok_or_else(|| {
            crate::error::ApplicationError::QueryExecutionFailed(
                "アプリケーション設定が見つかりません".to_string(),
            )
        })?;

        // DTOに変換
        let user_options = UserOptionsDto {
            default_company_code: settings
                .default_company_code()
                .map(|code| code.value().to_string()),
            language: settings.language().value().to_string(),
            decimal_places: settings.decimal_places().value(),
            date_format: settings.date_format().value().to_string(),
        };

        let system_settings = SystemSettingsDto {
            fiscal_year_start_month: settings.fiscal_year_start_month().value(),
            closing_day: settings.closing_day().value(),
            auto_backup_enabled: settings.auto_backup_enabled(),
            backup_retention_days: settings.backup_retention_days().value(),
        };

        let response = LoadApplicationSettingsResponse { user_options, system_settings };

        // Output Portに通知
        self.output_port.present_application_settings(&response).await;

        Ok(response)
    }
}
