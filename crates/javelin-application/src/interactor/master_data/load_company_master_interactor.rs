// LoadCompanyMasterInteractor - 会社マスタ取得Interactor

use crate::{
    dtos::{
        request::LoadCompanyMasterRequest,
        response::{CompanyMasterItem, LoadCompanyMasterResponse},
    },
    error::ApplicationResult,
    input_ports::LoadCompanyMasterInputPort,
    output_ports::CompanyMasterOutputPort,
    query_service::CompanyMasterQueryService,
};

/// 会社マスタ取得Interactor
pub struct LoadCompanyMasterInteractor<Q, O>
where
    Q: CompanyMasterQueryService,
    O: CompanyMasterOutputPort,
{
    query_service: std::sync::Arc<Q>,
    output_port: O,
}

impl<Q, O> LoadCompanyMasterInteractor<Q, O>
where
    Q: CompanyMasterQueryService,
    O: CompanyMasterOutputPort,
{
    pub fn new(query_service: std::sync::Arc<Q>, output_port: O) -> Self {
        Self { query_service, output_port }
    }
}

#[allow(async_fn_in_trait)]
impl<Q, O> LoadCompanyMasterInputPort for LoadCompanyMasterInteractor<Q, O>
where
    Q: CompanyMasterQueryService,
    O: CompanyMasterOutputPort,
{
    async fn execute(
        &self,
        request: LoadCompanyMasterRequest,
    ) -> ApplicationResult<LoadCompanyMasterResponse> {
        // QueryServiceから全件取得
        let companies = self.query_service.get_all().await?;

        // フィルタリング
        let mut filtered_companies: Vec<CompanyMasterItem> = companies
            .into_iter()
            .filter(|company| {
                // アクティブフィルタ
                if request.active_only && !company.is_active() {
                    return false;
                }
                // テキストフィルタ
                if let Some(ref filter) = request.filter {
                    let filter_lower = filter.to_lowercase();
                    return company.code().value().to_lowercase().contains(&filter_lower)
                        || company.name().value().to_lowercase().contains(&filter_lower);
                }
                true
            })
            .map(|company| CompanyMasterItem {
                code: company.code().value().to_string(),
                name: company.name().value().to_string(),
                is_active: company.is_active(),
            })
            .collect();

        // コード順にソート
        filtered_companies.sort_by(|a, b| a.code.cmp(&b.code));

        let response = LoadCompanyMasterResponse { companies: filtered_companies };

        // Output Portに通知
        self.output_port.present_company_master(&response).await;

        Ok(response)
    }
}
