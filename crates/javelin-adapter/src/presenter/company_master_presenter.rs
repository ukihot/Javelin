// CompanyMasterPresenter実装
// 会社マスタの出力を整形してビューに渡す

use javelin_application::{
    dtos::response::LoadCompanyMasterResponse, output_ports::CompanyMasterOutputPort,
};
use tokio::sync::mpsc;

/// 会社マスタViewModel
#[derive(Debug, Clone)]
pub struct CompanyMasterViewModel {
    pub companies: Vec<CompanyMasterItemViewModel>,
}

/// 会社マスタ項目ViewModel
#[derive(Debug, Clone)]
pub struct CompanyMasterItemViewModel {
    pub code: String,
    pub name: String,
    pub is_active: bool,
    pub status_label: String,
}

/// 会社マスタPresenter
#[derive(Clone)]
pub struct CompanyMasterPresenter {
    sender: mpsc::UnboundedSender<CompanyMasterViewModel>,
}

impl CompanyMasterPresenter {
    pub fn new(sender: mpsc::UnboundedSender<CompanyMasterViewModel>) -> Self {
        Self { sender }
    }

    /// チャネルを作成
    pub fn create_channel() -> (
        mpsc::UnboundedSender<CompanyMasterViewModel>,
        mpsc::UnboundedReceiver<CompanyMasterViewModel>,
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
impl CompanyMasterOutputPort for CompanyMasterPresenter {
    async fn present_company_master(&self, response: &LoadCompanyMasterResponse) {
        let companies = response
            .companies
            .iter()
            .map(|item| CompanyMasterItemViewModel {
                code: item.code.clone(),
                name: item.name.clone(),
                is_active: item.is_active,
                status_label: Self::format_status_label(item.is_active),
            })
            .collect();

        let view_model = CompanyMasterViewModel { companies };

        let _ = self.sender.send(view_model);
    }

    async fn notify_error(&self, error_message: String) {
        eprintln!("[CompanyMaster Error] {}", error_message);
    }
}
