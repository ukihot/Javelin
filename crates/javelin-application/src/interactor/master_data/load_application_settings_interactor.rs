// LoadApplicationSettingsInteractor - アプリケーション設定取得Interactor

use crate::{
    dtos::{
        request::LoadApplicationSettingsRequest,
        response::{LoadApplicationSettingsResponse, SystemSettingsDto, UserOptionsDto},
    },
    error::ApplicationResult,
    input_ports::LoadApplicationSettingsInputPort,
    output_ports::ApplicationSettingsOutputPort,
    query_service::master_data_loader::MasterDataLoaderService,
};

/// アプリケーション設定取得Interactor
pub struct LoadApplicationSettingsInteractor<Q, O>
where
    Q: MasterDataLoaderService,
    O: ApplicationSettingsOutputPort,
{
    query_service: std::sync::Arc<Q>,
    output_port: O,
}

impl<Q, O> LoadApplicationSettingsInteractor<Q, O>
where
    Q: MasterDataLoaderService,
    O: ApplicationSettingsOutputPort,
{
    pub fn new(query_service: std::sync::Arc<Q>, output_port: O) -> Self {
        Self { query_service, output_port }
    }
}

#[allow(async_fn_in_trait)]
impl<Q, O> LoadApplicationSettingsInputPort for LoadApplicationSettingsInteractor<Q, O>
where
    Q: MasterDataLoaderService,
    O: ApplicationSettingsOutputPort,
{
    async fn execute(
        &self,
        _request: LoadApplicationSettingsRequest,
    ) -> ApplicationResult<LoadApplicationSettingsResponse> {
        // マスタデータを取得
        let master_data = self.query_service.load_master_data().await?;

        // DTOに変換
        let user_options = UserOptionsDto {
            default_company_code: master_data.user_options.default_company_code,
            language: master_data.user_options.language,
            decimal_places: master_data.user_options.decimal_places,
            date_format: master_data.user_options.date_format,
        };

        let system_settings = SystemSettingsDto {
            fiscal_year_start_month: master_data.system_settings.fiscal_year_start_month,
            closing_day: master_data.system_settings.closing_day,
            auto_backup_enabled: master_data.system_settings.auto_backup_enabled,
            backup_retention_days: master_data.system_settings.backup_retention_days,
        };

        let response = LoadApplicationSettingsResponse { user_options, system_settings };

        // Output Portに通知
        self.output_port.present_application_settings(&response).await;

        Ok(response)
    }
}
