// 請求書印刷プレゼンター
use javelin_application::output_ports::InvoicePrintOutputPort;
use tokio::sync::mpsc;

/// 請求書印刷ビューモデル
#[derive(Debug, Clone)]
pub struct InvoicePrintViewModel {
    pub status: PrintStatus,
    pub message: String,
}

/// 印刷ステータス
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrintStatus {
    Idle,
    Printing,
    Success { file_path: String },
    Error,
}

/// 請求書印刷プレゼンター
#[derive(Clone)]
pub struct InvoicePrintPresenter {
    sender: mpsc::UnboundedSender<InvoicePrintViewModel>,
}

impl InvoicePrintPresenter {
    pub fn new(sender: mpsc::UnboundedSender<InvoicePrintViewModel>) -> Self {
        Self { sender }
    }
}

// InvoicePrintOutputPortの実装
impl InvoicePrintOutputPort for InvoicePrintPresenter {
    fn notify_print_started(&self) -> impl std::future::Future<Output = ()> + Send {
        let sender = self.sender.clone();
        async move {
            let _ = sender.send(InvoicePrintViewModel {
                status: PrintStatus::Printing,
                message: "請求書を生成中...".to_string(),
            });
        }
    }

    fn notify_print_success(
        &self,
        file_path: String,
    ) -> impl std::future::Future<Output = ()> + Send {
        let sender = self.sender.clone();
        async move {
            let _ = sender.send(InvoicePrintViewModel {
                status: PrintStatus::Success { file_path: file_path.clone() },
                message: format!("請求書を保存しました: {}", file_path),
            });
        }
    }

    fn notify_print_error(
        &self,
        error_message: String,
    ) -> impl std::future::Future<Output = ()> + Send {
        let sender = self.sender.clone();
        async move {
            let _ = sender.send(InvoicePrintViewModel {
                status: PrintStatus::Error,
                message: format!("印刷に失敗しました: {}", error_message),
            });
        }
    }

    fn notify_progress(&self, message: String) -> impl std::future::Future<Output = ()> + Send {
        let sender = self.sender.clone();
        async move {
            let _ = sender.send(InvoicePrintViewModel { status: PrintStatus::Printing, message });
        }
    }
}
