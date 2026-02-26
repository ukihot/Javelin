// SubsidiaryAccountMasterPresenter実装
// 補助科目マスタの出力を整形してビューに渡す

use javelin_application::{
    dtos::response::LoadSubsidiaryAccountMasterResponse,
    output_ports::SubsidiaryAccountMasterOutputPort,
};
use tokio::sync::mpsc;

/// 補助科目マスタViewModel
#[derive(Debug, Clone)]
pub struct SubsidiaryAccountMasterViewModel {
    pub accounts: Vec<SubsidiaryAccountMasterItemViewModel>,
}

/// 補助科目マスタ項目ViewModel
#[derive(Debug, Clone)]
pub struct SubsidiaryAccountMasterItemViewModel {
    pub code: String,
    pub name: String,
    pub parent_account_code: String,
    pub is_active: bool,
    pub status_label: String,
}

/// 補助科目マスタPresenter
#[derive(Clone)]
pub struct SubsidiaryAccountMasterPresenter {
    sender: mpsc::UnboundedSender<SubsidiaryAccountMasterViewModel>,
}

impl SubsidiaryAccountMasterPresenter {
    pub fn new(sender: mpsc::UnboundedSender<SubsidiaryAccountMasterViewModel>) -> Self {
        Self { sender }
    }

    /// チャネルを作成
    pub fn create_channel() -> (
        mpsc::UnboundedSender<SubsidiaryAccountMasterViewModel>,
        mpsc::UnboundedReceiver<SubsidiaryAccountMasterViewModel>,
    ) {
        mpsc::unbounded_channel()
    }

    fn format_status_label(is_active: bool) -> String {
        if is_active {
            "有効".to_string()
        } else {
            "無効".to_string()
        }
    }
}

#[allow(async_fn_in_trait)]
impl SubsidiaryAccountMasterOutputPort for SubsidiaryAccountMasterPresenter {
    async fn present_subsidiary_account_master(
        &self,
        response: &LoadSubsidiaryAccountMasterResponse,
    ) {
        let accounts = response
            .accounts
            .iter()
            .map(|item| SubsidiaryAccountMasterItemViewModel {
                code: item.code.clone(),
                name: item.name.clone(),
                parent_account_code: item.parent_account_code.clone(),
                is_active: item.is_active,
                status_label: Self::format_status_label(item.is_active),
            })
            .collect();

        let view_model = SubsidiaryAccountMasterViewModel { accounts };

        let _ = self.sender.send(view_model);
    }

    async fn notify_error(&self, error_message: String) {
        eprintln!("[SubsidiaryAccountMaster Error] {}", error_message);
    }
}
