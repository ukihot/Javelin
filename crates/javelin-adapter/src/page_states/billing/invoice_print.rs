// 請求書印刷画面のページステート

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::presenter::invoice_print_presenter::InvoicePrintViewModel;

/// 請求書印刷ページステート
pub struct InvoicePrintPageState {
    id: Uuid,
    view_model_receiver: mpsc::UnboundedReceiver<InvoicePrintViewModel>,
}

impl InvoicePrintPageState {
    pub fn new(view_model_receiver: mpsc::UnboundedReceiver<InvoicePrintViewModel>) -> Self {
        Self { id: Uuid::new_v4(), view_model_receiver }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn view_model_receiver(&mut self) -> &mut mpsc::UnboundedReceiver<InvoicePrintViewModel> {
        &mut self.view_model_receiver
    }
}
