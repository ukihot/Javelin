// JournalEntryPresenter実装
// 仕訳登録の出力を整形してビューに渡す

use javelin_application::{
    dtos::{
        ApproveJournalEntryResponse, CorrectJournalEntryResponse, DeleteDraftJournalEntryResponse,
        JournalEntryDetail, JournalEntryListResult, RegisterJournalEntryResponse,
        RejectJournalEntryResponse, ReverseJournalEntryResponse, SubmitForApprovalResponse,
        UpdateDraftJournalEntryResponse,
    },
    output_ports::{JournalEntryOutputPort, QueryOutputPort},
};
use tokio::sync::mpsc;

/// 仕訳一覧ViewModel
#[derive(Debug, Clone)]
pub struct JournalEntryListViewModel {
    pub items: Vec<JournalEntryListItemViewModel>,
    pub total_count: u32,
}

/// 仕訳一覧項目ViewModel
#[derive(Debug, Clone)]
pub struct JournalEntryListItemViewModel {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub status: String,
    pub status_label: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub total_debit: f64,
    pub total_credit: f64,
    pub created_by: String,
    pub created_at: String,
}

/// 仕訳詳細ViewModel
#[derive(Debug, Clone)]
pub struct JournalEntryDetailViewModel {
    pub entry_id: String,
    pub entry_number: Option<String>,
    pub status: String,
    pub status_label: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub lines: Vec<JournalEntryLineViewModel>,
    pub created_by: String,
    pub created_at: String,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
}

/// 仕訳明細ViewModel
#[derive(Debug, Clone)]
pub struct JournalEntryLineViewModel {
    pub line_number: u32,
    pub side: String,
    pub side_label: String,
    pub account_code: String,
    pub account_name: String,
    pub sub_account_code: Option<String>,
    pub department_code: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub tax_type: String,
    pub tax_amount: f64,
    pub partner_id: Option<String>,
    pub external_name: Option<String>,
    pub tracking_number: Option<String>,
}

/// 仕訳登録ビューモデル
#[derive(Debug, Clone)]
pub struct JournalEntryViewModel {
    pub entry_id: String,
    pub status: String,
    pub message: String,
    pub success: bool,
}

impl JournalEntryViewModel {
    /// ステータスバッジの色を取得
    pub fn status_color(&self) -> &str {
        match self.status.as_str() {
            "Draft" => "gray",
            "PendingApproval" => "orange",
            "Posted" => "green",
            "Reversed" => "red",
            "Corrected" => "blue",
            "Closed" => "dark-gray",
            "Deleted" => "light-gray",
            _ => "gray",
        }
    }

    /// ステータスの日本語表示を取得
    pub fn status_label(&self) -> &str {
        match self.status.as_str() {
            "Draft" => "下書き",
            "PendingApproval" => "承認待ち",
            "Posted" => "記帳済",
            "Reversed" => "取消済",
            "Corrected" => "修正済",
            "Closed" => "締め済",
            "Deleted" => "削除済",
            _ => "不明",
        }
    }

    /// 成功メッセージかどうか
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// エラーメッセージかどうか
    pub fn is_error(&self) -> bool {
        !self.success
    }
}

/// 仕訳登録Presenter
#[derive(Clone)]
pub struct JournalEntryPresenter {
    list_sender: mpsc::UnboundedSender<JournalEntryListViewModel>,
    detail_sender: mpsc::UnboundedSender<JournalEntryDetailViewModel>,
    result_sender: mpsc::UnboundedSender<JournalEntryViewModel>,
    progress_sender: mpsc::UnboundedSender<String>,
}

/// チャネル作成の戻り値型
pub type JournalEntryChannels = (
    mpsc::UnboundedSender<JournalEntryListViewModel>,
    mpsc::UnboundedReceiver<JournalEntryListViewModel>,
    mpsc::UnboundedSender<JournalEntryDetailViewModel>,
    mpsc::UnboundedReceiver<JournalEntryDetailViewModel>,
    mpsc::UnboundedSender<JournalEntryViewModel>,
    mpsc::UnboundedReceiver<JournalEntryViewModel>,
    mpsc::UnboundedSender<String>,
    mpsc::UnboundedReceiver<String>,
);

impl JournalEntryPresenter {
    pub fn new(
        list_sender: mpsc::UnboundedSender<JournalEntryListViewModel>,
        detail_sender: mpsc::UnboundedSender<JournalEntryDetailViewModel>,
        result_sender: mpsc::UnboundedSender<JournalEntryViewModel>,
        progress_sender: mpsc::UnboundedSender<String>,
    ) -> Self {
        Self { list_sender, detail_sender, result_sender, progress_sender }
    }

    /// チャネルを作成
    pub fn create_channels() -> JournalEntryChannels {
        let (list_tx, list_rx) = mpsc::unbounded_channel();
        let (detail_tx, detail_rx) = mpsc::unbounded_channel();
        let (result_tx, result_rx) = mpsc::unbounded_channel();
        let (progress_tx, progress_rx) = mpsc::unbounded_channel();
        (
            list_tx,
            list_rx,
            detail_tx,
            detail_rx,
            result_tx,
            result_rx,
            progress_tx,
            progress_rx,
        )
    }

    fn format_status_label(status: &str) -> String {
        match status {
            "Draft" => "下書き",
            "PendingApproval" => "承認待ち",
            "Posted" => "記帳済",
            "Reversed" => "取消済",
            "Corrected" => "修正済",
            "Closed" => "締め済",
            "Deleted" => "削除済",
            _ => "不明",
        }
        .to_string()
    }

    fn format_side_label(side: &str) -> String {
        match side {
            "Debit" => "借方",
            "Credit" => "貸方",
            _ => side,
        }
        .to_string()
    }

    /// エラーをビューモデルに変換
    pub fn present_error(&self, error: String) -> JournalEntryViewModel {
        JournalEntryViewModel {
            entry_id: String::new(),
            status: "Error".to_string(),
            message: format!("エラーが発生しました: {}", error),
            success: false,
        }
    }
}

#[allow(async_fn_in_trait)]
impl QueryOutputPort for JournalEntryPresenter {
    async fn present_journal_entry_list(&self, result: JournalEntryListResult) {
        let items = result
            .items
            .into_iter()
            .map(|item| JournalEntryListItemViewModel {
                entry_id: item.entry_id,
                entry_number: item.entry_number,
                status: item.status.clone(),
                status_label: Self::format_status_label(&item.status),
                transaction_date: item.transaction_date,
                voucher_number: item.voucher_number,
                total_debit: item.total_debit,
                total_credit: item.total_credit,
                created_by: item.created_by,
                created_at: item.created_at,
            })
            .collect();

        let view_model = JournalEntryListViewModel { items, total_count: result.total_count };

        let _ = self.list_sender.send(view_model);
    }

    async fn present_journal_entry_detail(&self, result: JournalEntryDetail) {
        let lines = result
            .lines
            .into_iter()
            .map(|line| JournalEntryLineViewModel {
                line_number: line.line_number,
                side: line.side.clone(),
                side_label: Self::format_side_label(&line.side),
                account_code: line.account_code,
                account_name: line.account_name,
                sub_account_code: line.sub_account_code,
                department_code: line.department_code,
                amount: line.amount,
                currency: line.currency,
                tax_type: line.tax_type,
                tax_amount: line.tax_amount,
                partner_id: line.partner_id,
                external_name: line.external_name,
                tracking_number: line.tracking_number,
            })
            .collect();

        let view_model = JournalEntryDetailViewModel {
            entry_id: result.entry_id,
            entry_number: result.entry_number,
            status: result.status.clone(),
            status_label: Self::format_status_label(&result.status),
            transaction_date: result.transaction_date,
            voucher_number: result.voucher_number,
            lines,
            created_by: result.created_by,
            created_at: result.created_at,
            updated_by: result.updated_by,
            updated_at: result.updated_at,
            approved_by: result.approved_by,
            approved_at: result.approved_at,
        };

        let _ = self.detail_sender.send(view_model);
    }

    async fn present_ledger(&self, _result: javelin_application::query_service::LedgerResult) {
        // Not implemented for JournalEntryPresenter
    }

    async fn present_trial_balance(
        &self,
        _result: javelin_application::query_service::TrialBalanceResult,
    ) {
        // Not implemented for JournalEntryPresenter
    }
}

#[allow(async_fn_in_trait)]
impl JournalEntryOutputPort for JournalEntryPresenter {
    async fn present_register_result(&self, response: RegisterJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: "仕訳を下書きとして保存しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn notify_progress(&self, message: String) {
        let _ = self.progress_sender.send(message);
    }

    async fn notify_error(&self, error_message: String) {
        let view_model = JournalEntryViewModel {
            entry_id: String::new(),
            status: "Error".to_string(),
            message: error_message,
            success: false,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_update_draft_result(&self, response: UpdateDraftJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: "下書きを更新しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_submit_for_approval_result(&self, response: SubmitForApprovalResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: "承認申請しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_approve_result(&self, response: ApproveJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: format!("仕訳を承認しました（伝票番号: {}）", response.entry_number),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_reject_result(&self, response: RejectJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: "仕訳を差し戻しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_reverse_result(&self, response: ReverseJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: "仕訳を取り消しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_correct_result(&self, response: CorrectJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: response.entry_id,
            status: response.status,
            message: "仕訳を修正しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }

    async fn present_delete_draft_result(&self, _response: DeleteDraftJournalEntryResponse) {
        let view_model = JournalEntryViewModel {
            entry_id: String::new(),
            status: "Deleted".to_string(),
            message: "下書きを削除しました".to_string(),
            success: true,
        };
        let _ = self.result_sender.send(view_model);
    }
}
