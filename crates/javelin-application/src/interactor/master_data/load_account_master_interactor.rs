// LoadAccountMasterInteractor - 勘定科目マスタ取得Interactor

use crate::{
    dtos::{AccountMasterItem, LoadAccountMasterRequest, LoadAccountMasterResponse},
    error::ApplicationResult,
    input_ports::LoadAccountMasterInputPort,
    output_ports::AccountMasterOutputPort,
    query_service::AccountMasterQueryService,
};

/// 勘定科目マスタ取得Interactor
pub struct LoadAccountMasterInteractor<Q, O>
where
    Q: AccountMasterQueryService,
    O: AccountMasterOutputPort,
{
    query_service: std::sync::Arc<Q>,
    output_port: O,
}

impl<Q, O> LoadAccountMasterInteractor<Q, O>
where
    Q: AccountMasterQueryService,
    O: AccountMasterOutputPort,
{
    pub fn new(query_service: std::sync::Arc<Q>, output_port: O) -> Self {
        Self { query_service, output_port }
    }
}

#[allow(async_fn_in_trait)]
impl<Q, O> LoadAccountMasterInputPort for LoadAccountMasterInteractor<Q, O>
where
    Q: AccountMasterQueryService,
    O: AccountMasterOutputPort,
{
    async fn execute(
        &self,
        request: LoadAccountMasterRequest,
    ) -> ApplicationResult<LoadAccountMasterResponse> {
        // QueryServiceから全件取得
        let accounts = self.query_service.get_all().await?;

        // フィルタリング
        let mut filtered_accounts: Vec<AccountMasterItem> = accounts
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
            .map(|acc| AccountMasterItem {
                code: acc.code().value().to_string(),
                name: acc.name().value().to_string(),
                account_type: format!("{:?}", acc.account_type()),
            })
            .collect();

        // コード順にソート
        filtered_accounts.sort_by(|a, b| a.code.cmp(&b.code));

        let response = LoadAccountMasterResponse { accounts: filtered_accounts };

        // Output Portに通知
        self.output_port.present_account_master(&response).await;

        Ok(response)
    }
}
