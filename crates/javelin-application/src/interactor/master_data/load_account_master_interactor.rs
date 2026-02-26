// LoadAccountMasterInteractor - 勘定科目マスタ取得Interactor

use crate::{
    dtos::{AccountMasterItem, LoadAccountMasterRequest, LoadAccountMasterResponse},
    error::ApplicationResult,
    input_ports::LoadAccountMasterInputPort,
    output_ports::AccountMasterOutputPort,
    query_service::master_data_loader::MasterDataLoaderService,
};

/// 勘定科目マスタ取得Interactor
pub struct LoadAccountMasterInteractor<Q, O>
where
    Q: MasterDataLoaderService,
    O: AccountMasterOutputPort,
{
    query_service: std::sync::Arc<Q>,
    output_port: O,
}

impl<Q, O> LoadAccountMasterInteractor<Q, O>
where
    Q: MasterDataLoaderService,
    O: AccountMasterOutputPort,
{
    pub fn new(query_service: std::sync::Arc<Q>, output_port: O) -> Self {
        Self { query_service, output_port }
    }
}

#[allow(async_fn_in_trait)]
impl<Q, O> LoadAccountMasterInputPort for LoadAccountMasterInteractor<Q, O>
where
    Q: MasterDataLoaderService,
    O: AccountMasterOutputPort,
{
    async fn execute(
        &self,
        request: LoadAccountMasterRequest,
    ) -> ApplicationResult<LoadAccountMasterResponse> {
        // マスタデータを取得
        let master_data = self.query_service.load_master_data().await?;

        // フィルタリング
        let mut accounts: Vec<AccountMasterItem> = master_data
            .accounts
            .into_iter()
            .filter(|acc| {
                // アクティブフィルタ
                if request.active_only && !acc.is_active {
                    return false;
                }
                // テキストフィルタ
                if let Some(ref filter) = request.filter {
                    let filter_lower = filter.to_lowercase();
                    return acc.code.to_lowercase().contains(&filter_lower)
                        || acc.name.to_lowercase().contains(&filter_lower);
                }
                true
            })
            .map(|acc| AccountMasterItem {
                code: acc.code,
                name: acc.name,
                account_type: format!("{:?}", acc.account_type),
            })
            .collect();

        // コード順にソート
        accounts.sort_by(|a, b| a.code.cmp(&b.code));

        let response = LoadAccountMasterResponse { accounts };

        // Output Portに通知
        self.output_port.present_account_master(&response).await;

        Ok(response)
    }
}
