// JournalEntryFormPage - 原始記録登録画面
// 責務: 仕訳入力フォーム（4.1 原始記録登録処理）

use javelin_application::dtos::{JournalEntryLineDto, RegisterJournalEntryRequest};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    input_mode::{InputMode, JjEscapeDetector, JournalEntryEditMode, ModifyInputType},
    views::{
        components::{InputField, LoadingSpinner, OverlaySelector, TabbedJournalEntryForm},
        layouts::FormLayout,
    },
};

/// 確定処理の状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubmitState {
    Idle,       // 待機中
    Submitting, // 送信中
    Success,    // 成功
    Failed,     // 失敗
}

pub struct JournalEntryFormPage {
    layout: FormLayout,
    // 編集区分
    edit_mode: JournalEntryEditMode,
    // 参照元伝票ID（新規以外の場合）
    reference_entry_id: Option<String>,
    // ヘッダーフィールド
    date_field: InputField,
    voucher_field: InputField,
    risk_field: InputField,
    // 明細行フォーム（タブ付き）
    tabbed_form: TabbedJournalEntryForm,
    // 状態
    focused_field: usize, // 0-2: ヘッダー, 3-7: 明細行
    // Vimライク操作
    input_mode: InputMode,
    jj_detector: JjEscapeDetector,
    // オーバーレイセレクタ
    overlay_selector: OverlaySelector,
    // データロード要求フラグ
    pending_account_load: bool,
    // AccountMasterデータ受信用（オプション）
    account_master_receiver:
        Option<tokio::sync::mpsc::UnboundedReceiver<crate::presenter::AccountMasterViewModel>>,
    // JournalEntryViewModel受信用（オプション）
    result_receiver:
        Option<tokio::sync::mpsc::UnboundedReceiver<crate::presenter::JournalEntryViewModel>>,
    // 進捗メッセージ受信用（オプション）
    progress_receiver: Option<tokio::sync::mpsc::UnboundedReceiver<String>>,
    // 確定処理の状態
    submit_state: SubmitState,
    submit_error_message: Option<String>,
    loading_spinner: LoadingSpinner,
}

impl JournalEntryFormPage {
    pub fn new() -> Self {
        let mut layout = FormLayout::new("原始記録登録処理", "F-101", InputMode::Normal);
        layout.event_viewer_mut().add_info("原始記録登録画面を開きました");

        // 取引日付のデフォルト値を当日に設定（8桁の数字形式: YYYYMMDD）
        let today = chrono::Local::now().format("%Y%m%d").to_string();

        let mut page = Self {
            layout,
            edit_mode: JournalEntryEditMode::default(),
            reference_entry_id: None,
            date_field: InputField::new("取引日付")
                .required()
                .with_placeholder("YYYY-MM-DD")
                .with_max_length(10)
                .with_value(today)
                .with_input_type(ModifyInputType::Calendar),
            voucher_field: InputField::new("伝票番号").with_placeholder("自動採番").readonly(),
            risk_field: InputField::new("リスク分類").with_value("Low").readonly(),
            tabbed_form: TabbedJournalEntryForm::new(),
            focused_field: 0,
            input_mode: InputMode::Normal,
            jj_detector: JjEscapeDetector::new(),
            overlay_selector: OverlaySelector::new("選択してください"),
            pending_account_load: false,
            account_master_receiver: None,
            result_receiver: None,
            progress_receiver: None,
            submit_state: SubmitState::Idle,
            submit_error_message: None,
            loading_spinner: LoadingSpinner::new(),
        };

        // 初期フォーカスを設定
        page.update_focus();
        page
    }

    /// 明細行を追加
    pub fn add_line(&mut self) {
        self.tabbed_form.add_line();
        self.layout
            .event_viewer_mut()
            .add_info(format!("明細行 #{} を追加しました", self.tabbed_form.line_count()));
    }

    /// 明細行を削除（最低2行は残す）
    pub fn remove_line(&mut self) {
        if self.tabbed_form.remove_line() {
            self.layout.event_viewer_mut().add_info("明細行を削除しました");
        } else {
            self.layout.event_viewer_mut().add_info("最低2行の明細が必要です");
        }
    }

    /// 次の明細行へ移動
    pub fn next_line(&mut self) {
        self.tabbed_form.next_line();
        self.layout
            .event_viewer_mut()
            .add_info(format!("明細行 #{} へ移動", self.tabbed_form.current_line_index() + 1));
    }

    /// 前の明細行へ移動
    pub fn previous_line(&mut self) {
        self.tabbed_form.previous_line();
        self.layout
            .event_viewer_mut()
            .add_info(format!("明細行 #{} へ移動", self.tabbed_form.current_line_index() + 1));
    }

    /// 伝票番号を設定（自動採番された値）
    pub fn set_voucher_number(&mut self, voucher_number: String) {
        self.voucher_field.set_value(voucher_number);
    }

    /// 確定処理を開始
    pub fn start_submit(&mut self) {
        if self.submit_state == SubmitState::Submitting {
            return; // 既に送信中
        }

        // 確定処理開始時にNormalモードに遷移
        self.input_mode.enter_normal();
        self.jj_detector.reset();

        self.submit_state = SubmitState::Submitting;
        self.submit_error_message = None;

        let message = match self.edit_mode {
            JournalEntryEditMode::NewEntry => "新規起票処理を開始しています...",
            JournalEntryEditMode::Cancellation => "取消仕訳を登録しています...",
            JournalEntryEditMode::Reversal => "反対仕訳を登録しています...",
            JournalEntryEditMode::Additional => "追加仕訳を登録しています...",
            JournalEntryEditMode::Reclassification => "再分類仕訳を登録しています...",
            JournalEntryEditMode::Replacement => "洗替仕訳を登録しています...",
        };

        self.layout.event_viewer_mut().add_info(message);
    }

    /// 確定処理が送信中かどうか
    pub fn is_submitting(&self) -> bool {
        self.submit_state == SubmitState::Submitting
    }

    /// 確定処理の成功を設定
    pub fn set_submit_success(&mut self, entry_number: Option<String>) {
        self.submit_state = SubmitState::Success;

        let message = match self.edit_mode {
            JournalEntryEditMode::NewEntry => {
                if let Some(number) = entry_number {
                    format!("新規起票が完了しました（伝票番号: {}）", number)
                } else {
                    "新規起票が完了しました".to_string()
                }
            }
            JournalEntryEditMode::Cancellation => {
                if let Some(number) = entry_number {
                    format!("取消仕訳を登録しました（伝票番号: {}）", number)
                } else {
                    "取消仕訳を登録しました".to_string()
                }
            }
            JournalEntryEditMode::Reversal => {
                if let Some(number) = entry_number {
                    format!("反対仕訳を登録しました（伝票番号: {}）", number)
                } else {
                    "反対仕訳を登録しました".to_string()
                }
            }
            JournalEntryEditMode::Additional => {
                if let Some(number) = entry_number {
                    format!("追加仕訳を登録しました（伝票番号: {}）", number)
                } else {
                    "追加仕訳を登録しました".to_string()
                }
            }
            JournalEntryEditMode::Reclassification => {
                if let Some(number) = entry_number {
                    format!("再分類仕訳を登録しました（伝票番号: {}）", number)
                } else {
                    "再分類仕訳を登録しました".to_string()
                }
            }
            JournalEntryEditMode::Replacement => {
                if let Some(number) = entry_number {
                    format!("洗替仕訳を登録しました（伝票番号: {}）", number)
                } else {
                    "洗替仕訳を登録しました".to_string()
                }
            }
        };

        self.layout.event_viewer_mut().add_info(&message);
    }

    /// 確定処理の失敗を設定
    pub fn set_submit_failed(&mut self, error_message: String) {
        self.submit_state = SubmitState::Failed;
        self.submit_error_message = Some(error_message.clone());

        let prefix = match self.edit_mode {
            JournalEntryEditMode::NewEntry => "新規起票処理に失敗しました",
            JournalEntryEditMode::Cancellation => "取消仕訳の登録に失敗しました",
            JournalEntryEditMode::Reversal => "反対仕訳の登録に失敗しました",
            JournalEntryEditMode::Additional => "追加仕訳の登録に失敗しました",
            JournalEntryEditMode::Reclassification => "再分類仕訳の登録に失敗しました",
            JournalEntryEditMode::Replacement => "洗替仕訳の登録に失敗しました",
        };

        self.layout
            .event_viewer_mut()
            .add_error(format!("{}: {}", prefix, error_message));
    }

    /// 確定処理の状態をリセット
    pub fn reset_submit_state(&mut self) {
        self.submit_state = SubmitState::Idle;
        self.submit_error_message = None;
    }

    /// 入力データをRegisterJournalEntryRequestに変換
    pub fn to_register_request(
        &self,
        user_id: String,
    ) -> Result<RegisterJournalEntryRequest, String> {
        let mut lines = Vec::new();

        for (idx, line_form) in self.tabbed_form.lines().iter().enumerate() {
            let line_number = (idx + 1) as u32;

            // 借方明細
            let debit_account = line_form.debit_account().value();
            let debit_amount_str = line_form.debit_amount().value();
            let description_value = line_form.description().value();
            let description = if description_value.is_empty() {
                None
            } else {
                Some(description_value.to_string())
            };

            if !debit_account.is_empty() && !debit_amount_str.is_empty() {
                let debit_amount: f64 = debit_amount_str
                    .parse()
                    .map_err(|_| format!("明細 #{}: 借方金額が不正です", line_number))?;

                lines.push(JournalEntryLineDto {
                    line_number,
                    side: "Debit".to_string(),
                    account_code: debit_account.to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: debit_amount,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: description.clone(),
                });
            }

            // 貸方明細
            let credit_account = line_form.credit_account().value();
            let credit_amount_str = line_form.credit_amount().value();
            if !credit_account.is_empty() && !credit_amount_str.is_empty() {
                let credit_amount: f64 = credit_amount_str
                    .parse()
                    .map_err(|_| format!("明細 #{}: 貸方金額が不正です", line_number))?;

                lines.push(JournalEntryLineDto {
                    line_number,
                    side: "Credit".to_string(),
                    account_code: credit_account.to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: credit_amount,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description,
                });
            }
        }

        if lines.is_empty() {
            return Err("明細行が入力されていません".to_string());
        }

        Ok(RegisterJournalEntryRequest {
            transaction_date: self.date_field.value().to_string(),
            voucher_number: self.voucher_field.value().to_string(),
            lines,
            user_id,
        })
    }

    /// 入力モードを取得
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    /// 編集区分を取得
    pub fn edit_mode(&self) -> JournalEntryEditMode {
        self.edit_mode
    }

    /// 編集区分を次へ切り替え
    pub fn switch_edit_mode_next(&mut self) {
        self.edit_mode = self.edit_mode.next();
        self.layout
            .event_viewer_mut()
            .add_info(format!("編集区分を切り替えました: {}", self.edit_mode.display_name()));

        if !self.edit_mode.requires_reference() {
            self.reference_entry_id = None;
        }
    }

    /// 編集区分を前へ切り替え
    pub fn switch_edit_mode_previous(&mut self) {
        self.edit_mode = self.edit_mode.previous();
        self.layout
            .event_viewer_mut()
            .add_info(format!("編集区分を切り替えました: {}", self.edit_mode.display_name()));

        if !self.edit_mode.requires_reference() {
            self.reference_entry_id = None;
        }
    }

    /// 参照元伝票IDを設定
    pub fn set_reference_entry_id(&mut self, entry_id: String) {
        self.reference_entry_id = Some(entry_id);
        self.layout.event_viewer_mut().add_info(format!(
            "参照元伝票を設定しました: {}",
            self.reference_entry_id.as_ref().unwrap()
        ));
    }

    /// 参照元伝票IDを取得
    pub fn reference_entry_id(&self) -> Option<&String> {
        self.reference_entry_id.as_ref()
    }

    /// 変更モードに入る（iキー）
    pub fn enter_modify_mode(&mut self) {
        let field = self.get_focused_field();
        let input_type = field.input_type();

        match input_type {
            ModifyInputType::Direct => {
                self.get_focused_field_mut().start_modify();
                self.input_mode.enter_modify();
                self.jj_detector.reset();
            }
            ModifyInputType::OverlayList => {
                self.overlay_selector.start_loading();
                self.input_mode.enter_modify();
                self.jj_detector.reset();
                self.pending_account_load = true;
            }
            ModifyInputType::Calendar => {
                // カレンダー選択モードに入る
                self.get_focused_field_mut().start_modify();
                self.input_mode.enter_modify();
                self.jj_detector.reset();
            }
            ModifyInputType::NumberOnly => {
                // 数値入力モードに入る
                self.get_focused_field_mut().start_modify();
                self.input_mode.enter_modify();
                self.jj_detector.reset();
            }
            ModifyInputType::BooleanToggle => {
                // Boolean切り替えモードに入る
                self.get_focused_field_mut().start_modify();
                self.input_mode.enter_modify();
                self.jj_detector.reset();
            }
        }
    }

    /// データロード要求があるかチェック
    pub fn has_pending_account_load(&self) -> bool {
        self.pending_account_load
    }

    /// データロード要求をクリア
    pub fn clear_pending_account_load(&mut self) {
        self.pending_account_load = false;
    }

    /// AccountMasterレシーバーを設定
    pub fn set_account_master_receiver(
        &mut self,
        receiver: tokio::sync::mpsc::UnboundedReceiver<crate::presenter::AccountMasterViewModel>,
    ) {
        self.account_master_receiver = Some(receiver);
    }

    /// AccountMasterレシーバーを取り出す
    pub fn take_account_master_receiver(
        &mut self,
    ) -> Option<tokio::sync::mpsc::UnboundedReceiver<crate::presenter::AccountMasterViewModel>>
    {
        self.account_master_receiver.take()
    }

    /// AccountMasterデータを受信してビューを更新
    pub fn poll_account_master_data(&mut self) {
        if let Some(receiver) = &mut self.account_master_receiver
            && let Ok(view_model) = receiver.try_recv()
        {
            // AccountMasterViewModelをオーバーレイ形式に変換
            let headers = vec!["コード".to_string(), "名称".to_string()];
            let rows: Vec<Vec<String>> = view_model
                .accounts
                .iter()
                .map(|a| vec![a.code.clone(), a.name.clone()])
                .collect();

            self.set_overlay_data(headers, rows);
            self.pending_account_load = false;
        }
    }

    /// JournalEntryResultレシーバーを設定
    pub fn set_result_receiver(
        &mut self,
        receiver: tokio::sync::mpsc::UnboundedReceiver<crate::presenter::JournalEntryViewModel>,
    ) {
        self.result_receiver = Some(receiver);
    }

    /// JournalEntryResultレシーバーを取り出す
    pub fn take_result_receiver(
        &mut self,
    ) -> Option<tokio::sync::mpsc::UnboundedReceiver<crate::presenter::JournalEntryViewModel>> {
        self.result_receiver.take()
    }

    /// 進捗メッセージレシーバーを設定
    pub fn set_progress_receiver(
        &mut self,
        receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
    ) {
        self.progress_receiver = Some(receiver);
    }

    /// 進捗メッセージレシーバーを取り出す
    pub fn take_progress_receiver(
        &mut self,
    ) -> Option<tokio::sync::mpsc::UnboundedReceiver<String>> {
        self.progress_receiver.take()
    }

    /// 進捗メッセージを受信してイベントログに追加
    pub fn poll_progress_messages(&mut self) {
        if let Some(receiver) = &mut self.progress_receiver {
            while let Ok(message) = receiver.try_recv() {
                self.layout.event_viewer_mut().add_info(&message);
            }
        }
    }

    /// JournalEntryResultデータを受信してビューを更新
    pub fn poll_result_data(&mut self) {
        if let Some(receiver) = &mut self.result_receiver
            && let Ok(view_model) = receiver.try_recv()
        {
            if view_model.is_success() {
                // 成功時: entry_numberがmessageに含まれている可能性がある
                // messageから伝票番号を抽出するか、そのまま表示
                self.submit_state = SubmitState::Success;
                self.layout.event_viewer_mut().add_info(&view_model.message);
            } else {
                // 失敗時: 進捗メッセージチャネルをクリアしてから、エラーを表示
                if let Some(progress_receiver) = &mut self.progress_receiver {
                    while progress_receiver.try_recv().is_ok() {
                        // チャネルをクリア
                    }
                }

                self.submit_state = SubmitState::Failed;
                self.submit_error_message = Some(view_model.message.clone());
                self.layout
                    .event_viewer_mut()
                    .add_error(format!("確定処理に失敗しました: {}", view_model.message));
            }
        }
    }

    /// オーバーレイセレクタが表示中かどうか
    pub fn is_overlay_visible(&self) -> bool {
        self.overlay_selector.is_visible()
    }

    /// オーバーレイセレクタのデータを設定
    pub fn set_overlay_data(&mut self, headers: Vec<String>, rows: Vec<Vec<String>>) {
        self.overlay_selector.set_data(headers, rows);
    }

    /// オーバーレイセレクタで選択を上に移動
    pub fn overlay_select_previous(&mut self) {
        self.overlay_selector.select_previous();
    }

    /// オーバーレイセレクタで選択を下に移動
    pub fn overlay_select_next(&mut self) {
        self.overlay_selector.select_next();
    }

    /// オーバーレイセレクタで選択を確定
    pub fn overlay_confirm_selection(&mut self) {
        let selected_value =
            self.overlay_selector.selected_row().and_then(|row| row.first()).cloned();

        if let Some(value) = selected_value {
            self.get_focused_field_mut().set_value(value);
        }

        self.overlay_selector.hide();
        self.input_mode.enter_normal();
    }

    /// オーバーレイセレクタをキャンセル
    pub fn overlay_cancel(&mut self) {
        self.overlay_selector.hide();
        self.input_mode.enter_normal();
    }

    /// ローディングアニメーションを更新
    pub fn tick_loading(&mut self) {
        self.overlay_selector.tick_loading();
        if self.submit_state == SubmitState::Submitting {
            self.loading_spinner.tick();
        }
    }

    /// 非変更モードに戻る（jjで確定）
    pub fn enter_normal_mode(&mut self) {
        // バリデーション実行
        if let Err(error_msg) = self.get_focused_field_mut().commit_buffer() {
            // エラーメッセージをイベントログに出力
            self.layout.event_viewer_mut().add_info(format!("入力エラー: {}", error_msg));
        }
        self.input_mode.enter_normal();
        self.jj_detector.reset();
    }

    /// 非変更モードに戻る（ESCでクリア）
    pub fn cancel_modify_mode(&mut self) {
        self.get_focused_field_mut().clear_buffer();
        self.input_mode.enter_normal();
        self.jj_detector.reset();
    }

    /// フォーカス中のフィールドを取得
    fn get_focused_field(&self) -> &InputField {
        match self.focused_field {
            0 => &self.date_field,
            1 => &self.voucher_field,
            2 => &self.risk_field,
            // 3-7は現在選択中の明細行のフィールド
            n if (3..=7).contains(&n) => {
                let field_index = n - 3;
                self.tabbed_form
                    .current_line()
                    .get_field(field_index)
                    .unwrap_or(&self.date_field)
            }
            _ => &self.date_field,
        }
    }

    /// フォーカス中のフィールドを取得（可変）
    fn get_focused_field_mut(&mut self) -> &mut InputField {
        match self.focused_field {
            0 => &mut self.date_field,
            1 => &mut self.voucher_field,
            2 => &mut self.risk_field,
            // 3-7は現在選択中の明細行のフィールド
            n if (3..=7).contains(&n) => {
                let field_index = n - 3;
                self.tabbed_form.current_line_mut().get_field_mut(field_index).unwrap()
            }
            _ => &mut self.date_field,
        }
    }

    /// 文字入力処理（変更モード時）
    pub fn input_char(&mut self, ch: char) {
        if !self.input_mode.is_modify() {
            return;
        }

        let (escaped, input_ch) = self.jj_detector.process(ch);

        if escaped {
            self.enter_normal_mode();
            return;
        }

        if let Some(ch) = input_ch {
            // 入力タイプに応じて文字をフィルタリング
            let field = self.get_focused_field();
            let input_type = field.input_type();

            if input_type.is_char_allowed(ch) {
                self.get_focused_field_mut().append_to_buffer(ch);
            }
            // 許可されない文字の場合は無視（何もしない）
        }
    }

    /// バックスペース処理
    pub fn backspace(&mut self) {
        if !self.input_mode.is_modify() {
            return;
        }

        if self.jj_detector.has_pending_j() {
            self.jj_detector.reset();
            return;
        }

        self.get_focused_field_mut().backspace_buffer();
    }

    /// 上に移動（kキー、非変更モード時）
    pub fn move_up(&mut self) {
        if self.input_mode.is_modify() {
            if self.overlay_selector.is_visible() {
                self.overlay_select_previous();
            }
            return;
        }
        self.focus_previous();
    }

    /// 下に移動（jキー、非変更モード時）
    pub fn move_down(&mut self) {
        if self.input_mode.is_modify() {
            if self.overlay_selector.is_visible() {
                self.overlay_select_next();
            }
            return;
        }
        self.focus_next();
    }

    /// 左に移動（hキー、非変更モード時）
    pub fn move_left(&mut self) {
        if !self.input_mode.is_modify() {
            self.previous_line();
        }
    }

    /// 右に移動（lキー、非変更モード時）
    pub fn move_right(&mut self) {
        if !self.input_mode.is_modify() {
            self.next_line();
        }
    }

    /// 次のフィールドへ移動
    pub fn focus_next(&mut self) {
        if self.focused_field < 8 {
            self.focused_field += 1;
        }
        self.update_focus();
    }

    /// 前のフィールドへ移動
    pub fn focus_previous(&mut self) {
        if self.focused_field > 0 {
            self.focused_field -= 1;
        }
        self.update_focus();
    }

    fn update_focus(&mut self) {
        self.date_field.set_focused(self.focused_field == 0);
        self.voucher_field.set_focused(self.focused_field == 1);
        self.risk_field.set_focused(self.focused_field == 2);

        // タブ内のフィールドにフォーカスがある場合
        if self.focused_field >= 3 && self.focused_field <= 7 {
            let field_index = self.focused_field - 3;
            self.tabbed_form.current_line_mut().update_focus(field_index);
        } else {
            // タブ外にフォーカスがある場合、タブ内のすべてのフォーカスをクリア
            self.tabbed_form.current_line_mut().update_focus(usize::MAX);
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        // フォーカス状態を更新
        self.update_focus();

        let input_mode = self.input_mode;
        let is_overlay_visible = self.overlay_selector.is_visible();
        let is_submitting = self.submit_state == SubmitState::Submitting;

        let title = format!("原始記録登録処理 [{}]", self.edit_mode.display_name());
        self.layout.set_title(&title);

        // InputModeをステータスに表示
        self.layout.set_status(input_mode);

        // フッターテキストを作成
        let footer_text = Some(Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("m", Style::default().fg(Color::Cyan)),
            Span::styled("]編集区分 [", Style::default().fg(Color::DarkGray)),
            Span::styled("Tab", Style::default().fg(Color::Cyan)),
            Span::styled("]明細追加 [", Style::default().fg(Color::DarkGray)),
            Span::styled("Shift+Tab", Style::default().fg(Color::Cyan)),
            Span::styled("]明細削除 [", Style::default().fg(Color::DarkGray)),
            Span::styled("h/l", Style::default().fg(Color::Cyan)),
            Span::styled("]明細切替 [", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+s", Style::default().fg(Color::Cyan)),
            Span::styled("]確定 [", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::styled("]戻る", Style::default().fg(Color::DarkGray)),
        ]));

        self.layout.render(frame, input_mode, footer_text, |frame, area| {
            // エリアを分割：ヘッダー + タブ付きフォーム
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4), // 取引日付
                    Constraint::Length(4), // 伝票番号
                    Constraint::Length(4), // リスク分類
                    Constraint::Min(0),    // タブ付きフォーム
                ])
                .split(area);

            let is_in_modify = input_mode.is_modify();

            // ヘッダーフィールドを描画
            self.date_field.render(frame, chunks[0], is_in_modify);
            self.voucher_field.render(frame, chunks[1], is_in_modify);
            self.risk_field.render(frame, chunks[2], is_in_modify);

            // タブ付きフォームを描画
            self.tabbed_form.render(frame, chunks[3], is_in_modify);

            // オーバーレイセレクタを最前面に描画
            if is_overlay_visible {
                self.overlay_selector.render(frame, area);
            }

            // 確定処理中はローディングスピナーを表示
            if is_submitting {
                let loading_message = match self.edit_mode {
                    JournalEntryEditMode::NewEntry => "新規起票処理中...",
                    JournalEntryEditMode::Cancellation => "取消仕訳登録中...",
                    JournalEntryEditMode::Reversal => "反対仕訳登録中...",
                    JournalEntryEditMode::Additional => "追加仕訳登録中...",
                    JournalEntryEditMode::Reclassification => "再分類仕訳登録中...",
                    JournalEntryEditMode::Replacement => "洗替仕訳登録中...",
                };
                self.loading_spinner.render(frame, area, loading_message);
            }
        });
    }
}

impl Default for JournalEntryFormPage {
    fn default() -> Self {
        Self::new()
    }
}
