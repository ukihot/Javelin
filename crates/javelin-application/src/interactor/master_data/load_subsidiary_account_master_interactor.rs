// LoadSubsidiaryAccountMasterInteractor - 補助科目マスタ取得Interactor

use std::sync::Arc;

use crate::{
    dtos::{
        request::LoadSubsidiaryAccountMasterRequest,
        response::{LoadSubsidiaryAccountMasterResponse, SubsidiaryAccountMasterItem},
    },
    error::ApplicationResult,
    input_ports::LoadSubsidiaryAccountMasterInputPort,
    output_ports::SubsidiaryAccountMasterOutputPort,
    query_service::SubsidiaryAccountMasterQueryService,
};

/// 補助科目マスタ取得Interactor
///
/// CQRS原則: 読み取りはQueryServiceを使用
pub struct LoadSubsidiaryAccountMasterInteractor<Q, O>
where
    Q: SubsidiaryAccountMasterQueryService,
    O: SubsidiaryAccountMasterOutputPort,
{
    query_service: Arc<Q>,
    output_port: O,
}

impl<Q, O> LoadSubsidiaryAccountMasterInteractor<Q, O>
where
    Q: SubsidiaryAccountMasterQueryService,
    O: SubsidiaryAccountMasterOutputPort,
{
    pub fn new(query_service: Arc<Q>, output_port: O) -> Self {
        Self { query_service, output_port }
    }
}

#[allow(async_fn_in_trait)]
impl<Q, O> LoadSubsidiaryAccountMasterInputPort for LoadSubsidiaryAccountMasterInteractor<Q, O>
where
    Q: SubsidiaryAccountMasterQueryService,
    O: SubsidiaryAccountMasterOutputPort,
{
    async fn execute(
        &self,
        request: LoadSubsidiaryAccountMasterRequest,
    ) -> ApplicationResult<LoadSubsidiaryAccountMasterResponse> {
        // QueryServiceから全件取得
        let accounts = self.query_service.get_all().await?;

        // フィルタリング
        let mut items: Vec<SubsidiaryAccountMasterItem> = accounts
            .into_iter()
            .filter(|acc| {
                // アクティブフィルタ
                if request.active_only && !acc.is_active() {
                    return false;
                }
                // テキストフィルタ
                if let Some(ref filter) = request.filter {
                    let filter_lower = filter.to_lowercase();
                    return acc.code().value().to_lowercase().contains(&filter_lower)
                        || acc.name().value().to_lowercase().contains(&filter_lower);
                }
                true
            })
            .map(|acc| SubsidiaryAccountMasterItem {
                code: acc.code().value().to_string(),
                name: acc.name().value().to_string(),
                parent_account_code: acc.parent_account_code().value().to_string(),
                is_active: acc.is_active(),
            })
            .collect();

        // コード順にソート
        items.sort_by(|a, b| a.code.cmp(&b.code));

        let response = LoadSubsidiaryAccountMasterResponse { accounts: items };

        // Output Portに通知
        self.output_port.present_subsidiary_account_master(&response).await;

        Ok(response)
    }
}
