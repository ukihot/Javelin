use crate::dtos::response::{
    FetchAccountMasterResponse, FetchApplicationSettingsResponse, FetchCompanyMasterResponse,
    FetchSubsidiaryAccountMasterResponse,
};

/// AccountMasterOutputPort - 勘定科目マスタ結果の出力
#[allow(async_fn_in_trait)]
pub trait AccountMasterOutputPort: Send + Sync {
    /// 勘定科目マスタ結果を出力
    async fn present_account_master(&self, response: &FetchAccountMasterResponse);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);
}

/// CompanyMasterOutputPort - 会社マスタ結果の出力
#[allow(async_fn_in_trait)]
pub trait CompanyMasterOutputPort: Send + Sync {
    /// 会社マスタ結果を出力
    async fn present_company_master(&self, response: &FetchCompanyMasterResponse);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);
}

/// ApplicationSettingsOutputPort - アプリケーション設定結果の出力
#[allow(async_fn_in_trait)]
pub trait ApplicationSettingsOutputPort: Send + Sync {
    /// アプリケーション設定結果を出力
    async fn present_application_settings(&self, response: &FetchApplicationSettingsResponse);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);
}

/// SubsidiaryAccountMasterOutputPort - 補助科目マスタ結果の出力
#[allow(async_fn_in_trait)]
pub trait SubsidiaryAccountMasterOutputPort: Send + Sync {
    /// 補助科目マスタ結果を出力
    async fn present_subsidiary_account_master(
        &self,
        response: &FetchSubsidiaryAccountMasterResponse,
    );

    /// エラーを通知
    async fn notify_error(&self, error_message: String);
}
