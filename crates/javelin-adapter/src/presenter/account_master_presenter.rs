// AccountMasterPresenter実装
// 勘定科目マスタの出力を整形してビューに渡す

use javelin_application::{
    dtos::response::FetchAccountMasterResponse, output_ports::AccountMasterOutputPort,
};
use tokio::sync::mpsc;

/// 勘定科目マスタViewModel
#[derive(Debug, Clone)]
pub struct AccountMasterViewModel {
    pub accounts: Vec<AccountMasterItemViewModel>,
}

/// 勘定科目マスタ項目ViewModel
#[derive(Debug, Clone)]
pub struct AccountMasterItemViewModel {
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub account_type_label: String,
}

/// 勘定科目マスタPresenter
#[derive(Clone)]
pub struct AccountMasterPresenter {
    sender: mpsc::UnboundedSender<AccountMasterViewModel>,
}

impl AccountMasterPresenter {
    pub fn new(sender: mpsc::UnboundedSender<AccountMasterViewModel>) -> Self {
        Self { sender }
    }

    /// チャネルを作成
    pub fn create_channel() -> (
        mpsc::UnboundedSender<AccountMasterViewModel>,
        mpsc::UnboundedReceiver<AccountMasterViewModel>,
    ) {
        mpsc::unbounded_channel()
    }

    fn format_account_type_label(account_type: &str) -> String {
        match account_type {
            "Asset" => "資産",
            "Liability" => "負債",
            "Equity" => "純資産",
            "Revenue" => "収益",
            "Expense" => "費用",
            _ => account_type,
        }
        .to_string()
    }
}

#[allow(async_fn_in_trait)]
impl AccountMasterOutputPort for AccountMasterPresenter {
    async fn present_account_master(&self, response: &FetchAccountMasterResponse) {
        let accounts = response
            .accounts
            .iter()
            .map(|item| AccountMasterItemViewModel {
                code: item.code.clone(),
                name: item.name.clone(),
                account_type: item.account_type.clone(),
                account_type_label: Self::format_account_type_label(&item.account_type),
            })
            .collect();

        let view_model = AccountMasterViewModel { accounts };

        let _ = self.sender.send(view_model);
    }

    async fn notify_error(&self, error_message: String) {
        eprintln!("[AccountMaster Error] {}", error_message);
    }
}
