// SearchPage - 仕訳検索画面
// 責務: 仕訳検索条件入力と検索結果表示

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tokio::sync::mpsc;

use crate::{
    format_amount,
    input_mode::{InputMode, JjEscapeDetector},
    presenter::SearchResultViewModel,
    truncate_text,
    views::components::{DataTable, InputField, OverlaySelector},
};

/// 検索フィールド
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchField {
    FromDate,
    ToDate,
    Description,
    AccountCode,
    DebitCredit,
    MinAmount,
    MaxAmount,
}

/// フォーカスエリア
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Criteria, // 検索条件
    Results,  // 検索結果
}

impl SearchField {
    fn all() -> Vec<Self> {
        vec![
            Self::FromDate,
            Self::ToDate,
            Self::Description,
            Self::AccountCode,
            Self::DebitCredit,
            Self::MinAmount,
            Self::MaxAmount,
        ]
    }

    fn next(&self) -> Self {
        let all = Self::all();
        let current_idx = all.iter().position(|f| f == self).unwrap();
        all[(current_idx + 1) % all.len()]
    }

    fn previous(&self) -> Self {
        let all = Self::all();
        let current_idx = all.iter().position(|f| f == self).unwrap();
        if current_idx == 0 {
            all[all.len() - 1]
        } else {
            all[current_idx - 1]
        }
    }

    // 3列グリッドレイアウトでの移動
    // 左列: FromDate(0), ToDate(1), Description(2)
    // 中央列: AccountCode(3), DebitCredit(4)
    // 右列: MinAmount(5), MaxAmount(6)

    fn move_up(&self) -> Self {
        match self {
            Self::FromDate => Self::FromDate,
            Self::ToDate => Self::FromDate,
            Self::Description => Self::ToDate,
            Self::AccountCode => Self::AccountCode,
            Self::DebitCredit => Self::AccountCode,
            Self::MinAmount => Self::MinAmount,
            Self::MaxAmount => Self::MinAmount,
        }
    }

    fn move_down(&self) -> Self {
        match self {
            Self::FromDate => Self::ToDate,
            Self::ToDate => Self::Description,
            Self::Description => Self::Description,
            Self::AccountCode => Self::DebitCredit,
            Self::DebitCredit => Self::DebitCredit,
            Self::MinAmount => Self::MaxAmount,
            Self::MaxAmount => Self::MaxAmount,
        }
    }

    fn move_left(&self) -> Self {
        match self {
            Self::FromDate => Self::MaxAmount,
            Self::ToDate => Self::DebitCredit,
            Self::Description => Self::DebitCredit,
            Self::AccountCode => Self::FromDate,
            Self::DebitCredit => Self::ToDate,
            Self::MinAmount => Self::AccountCode,
            Self::MaxAmount => Self::DebitCredit,
        }
    }

    fn move_right(&self) -> Self {
        match self {
            Self::FromDate => Self::AccountCode,
            Self::ToDate => Self::DebitCredit,
            Self::Description => Self::DebitCredit,
            Self::AccountCode => Self::MinAmount,
            Self::DebitCredit => Self::MaxAmount,
            Self::MinAmount => Self::MinAmount,
            Self::MaxAmount => Self::FromDate,
        }
    }
}

/// 仕訳検索画面
pub struct SearchPage {
    /// 入力モード
    input_mode: InputMode,
    /// フォーカスエリア
    focus_area: FocusArea,
    /// 現在フォーカス中のフィールド
    focused_field: SearchField,
    /// 検索条件フィールド
    from_date: InputField,
    to_date: InputField,
    description: InputField,
    account_code: InputField,
    debit_credit: InputField,
    min_amount: InputField,
    max_amount: InputField,
    /// 検索結果テーブル
    result_table: DataTable,
    /// ViewModelレシーバー
    result_receiver: mpsc::Receiver<SearchResultViewModel>,
    error_receiver: mpsc::Receiver<String>,
    progress_receiver: mpsc::Receiver<String>,
    execution_time_receiver: mpsc::Receiver<usize>,
    /// 現在表示中の検索結果
    current_result: Option<SearchResultViewModel>,
    /// 保留中の検索結果（最低表示時間待機中）
    pending_result: Option<SearchResultViewModel>,
    /// エラーメッセージ
    error_message: Option<String>,
    /// アニメーションフレーム
    animation_frame: usize,
    /// jjエスケープ検出器
    jj_detector: JjEscapeDetector,
    /// 進捗メッセージ表示開始時刻
    progress_display_start: Option<std::time::Instant>,
    /// 最低表示時間（ミリ秒）
    min_progress_display_duration: u64,
    /// 実行時間（ミリ秒）
    execution_time_ms: Option<usize>,
    /// オーバーレイセレクタ
    overlay_selector: OverlaySelector,
    /// 科目マスター読み込み待機フラグ
    pending_account_load: bool,
    /// 科目マスターレシーバー（ViewModel用、unbounded）
    account_master_receiver_vm:
        Option<tokio::sync::mpsc::UnboundedReceiver<crate::presenter::AccountMasterViewModel>>,
}

impl SearchPage {
    pub fn new(
        result_receiver: mpsc::Receiver<SearchResultViewModel>,
        error_receiver: mpsc::Receiver<String>,
        progress_receiver: mpsc::Receiver<String>,
        execution_time_receiver: mpsc::Receiver<usize>,
    ) -> Self {
        // 検索結果テーブルのヘッダー
        let headers = vec![
            "取引日付".to_string(),
            "伝票No".to_string(),
            "状態".to_string(),
            "摘要".to_string(),
            "勘定科目".to_string(),
            "金額".to_string(),
        ];

        let result_table = DataTable::new("◆ 検索結果 ◆", headers)
            .with_column_widths(vec![12, 15, 10, 30, 15, 13]);

        Self {
            input_mode: InputMode::Normal,
            focus_area: FocusArea::Criteria,
            focused_field: SearchField::FromDate,
            from_date: InputField::new("取引日付(開始)")
                .with_placeholder("YYYY-MM-DD")
                .with_input_type(crate::input_mode::ModifyInputType::Calendar),
            to_date: InputField::new("取引日付(終了)")
                .with_placeholder("YYYY-MM-DD")
                .with_input_type(crate::input_mode::ModifyInputType::Calendar),
            description: InputField::new("摘要")
                .with_placeholder("部分一致検索")
                .with_input_type(crate::input_mode::ModifyInputType::Direct),
            account_code: InputField::new("勘定科目")
                .with_placeholder("科目コード")
                .with_input_type(crate::input_mode::ModifyInputType::OverlayList),
            debit_credit: InputField::new("借方/貸方")
                .with_placeholder("借方/貸方")
                .with_input_type(crate::input_mode::ModifyInputType::BooleanToggle)
                .with_boolean_labels("貸方", "借方")
                .with_value("false".to_string()), // デフォルトは借方
            min_amount: InputField::new("金額(最小)")
                .with_placeholder("0")
                .with_input_type(crate::input_mode::ModifyInputType::NumberOnly),
            max_amount: InputField::new("金額(最大)")
                .with_placeholder("999999999")
                .with_input_type(crate::input_mode::ModifyInputType::NumberOnly),
            result_table,
            result_receiver,
            error_receiver,
            progress_receiver,
            execution_time_receiver,
            current_result: None,
            pending_result: None,
            error_message: None,
            animation_frame: 0,
            jj_detector: JjEscapeDetector::new(),
            progress_display_start: None,
            min_progress_display_duration: 500, // 0.5秒
            execution_time_ms: None,
            overlay_selector: OverlaySelector::new("勘定科目を選択"),
            pending_account_load: false,
            account_master_receiver_vm: None,
        }
    }

    /// ViewModelを受信してテーブルを更新
    pub fn update(&mut self) {
        // 科目マスターを受信（ViewModel形式）
        if let Some(ref mut receiver) = self.account_master_receiver_vm
            && let Ok(view_model) = receiver.try_recv()
        {
            let headers = vec!["科目コード".to_string(), "科目名".to_string()];
            let rows: Vec<Vec<String>> = view_model
                .accounts
                .into_iter()
                .map(|account| vec![account.code, account.name])
                .collect();
            self.overlay_selector.set_data(headers, rows);
            self.pending_account_load = false;
        }

        // すべての進捗メッセージを消費（最新のものを保持）
        let mut latest_progress = None;
        while let Ok(progress_message) = self.progress_receiver.try_recv() {
            latest_progress = Some(progress_message);
        }

        // 進捗メッセージを受信した場合、表示開始時刻を記録
        if let Some(ref progress_message) = latest_progress {
            self.result_table.set_loading_progress(progress_message.clone());
            self.progress_display_start = Some(std::time::Instant::now());
        }

        // 検索結果を受信
        if let Ok(view_model) = self.result_receiver.try_recv() {
            // 結果を保留
            self.pending_result = Some(view_model);
        }

        // 保留中の結果がある場合、最低表示時間をチェック
        if let Some(view_model) = self.pending_result.take() {
            let should_display = if let Some(start_time) = self.progress_display_start {
                let elapsed = start_time.elapsed();
                let min_duration =
                    std::time::Duration::from_millis(self.min_progress_display_duration);
                elapsed >= min_duration
            } else {
                true // 進捗メッセージがない場合は即座に表示
            };

            if should_display {
                // テーブルデータを構築（明細を展開）
                let mut rows: Vec<Vec<String>> = Vec::new();

                for entry in &view_model.items {
                    for (idx, line) in entry.lines.iter().enumerate() {
                        let date = if idx == 0 {
                            entry.transaction_date.clone()
                        } else {
                            "".to_string()
                        };

                        let entry_num = if idx == 0 {
                            entry.entry_number.clone().unwrap_or_default()
                        } else {
                            "".to_string()
                        };

                        let status = if idx == 0 {
                            entry.status_label.clone()
                        } else {
                            "".to_string()
                        };

                        rows.push(vec![
                            date,
                            entry_num,
                            status,
                            truncate_text!(&line.description, 28),
                            format!(
                                "{} {}",
                                line.account_code,
                                truncate_text!(&line.account_name, 8)
                            ),
                            format_amount!(line.amount, 11),
                        ]);
                    }
                }

                self.result_table.set_data(rows);
                self.current_result = Some(view_model);
                self.error_message = None;
                self.progress_display_start = None; // リセット
            } else {
                // まだ最低表示時間に達していないので、結果を戻す
                self.pending_result = Some(view_model);
            }
        }

        // エラーメッセージを受信
        if let Ok(error) = self.error_receiver.try_recv() {
            self.error_message = Some(error);
            self.progress_display_start = None; // リセット
            self.pending_result = None; // 保留中の結果をクリア
        }

        // 実行時間を受信
        if let Ok(elapsed_ms) = self.execution_time_receiver.try_recv() {
            self.execution_time_ms = Some(elapsed_ms);
        }
    }

    /// 次のフィールドにフォーカス
    pub fn focus_next_field(&mut self) {
        self.focused_field = self.focused_field.next();
    }

    /// 前のフィールドにフォーカス
    pub fn focus_previous_field(&mut self) {
        self.focused_field = self.focused_field.previous();
    }

    /// 上のフィールドにフォーカス
    pub fn focus_up(&mut self) {
        self.focused_field = self.focused_field.move_up();
    }

    /// 下のフィールドにフォーカス
    pub fn focus_down(&mut self) {
        self.focused_field = self.focused_field.move_down();
    }

    /// 左のフィールドにフォーカス
    pub fn focus_left(&mut self) {
        self.focused_field = self.focused_field.move_left();
    }

    /// 右のフィールドにフォーカス
    pub fn focus_right(&mut self) {
        self.focused_field = self.focused_field.move_right();
    }

    /// フォーカスエリアを切り替え（検索条件 ⇔ 検索結果）
    pub fn toggle_focus_area(&mut self) {
        self.focus_area = match self.focus_area {
            FocusArea::Criteria => FocusArea::Results,
            FocusArea::Results => FocusArea::Criteria,
        };
    }

    /// 現在のフォーカスエリアを取得
    pub fn focus_area(&self) -> FocusArea {
        self.focus_area
    }

    /// 次の行を選択
    pub fn select_next(&mut self) {
        self.result_table.select_next();
    }

    /// 前の行を選択
    pub fn select_previous(&mut self) {
        self.result_table.select_previous();
    }

    /// 選択中のインデックスを取得
    pub fn selected_index(&self) -> Option<usize> {
        self.result_table.selected_index()
    }

    /// 科目マスターレシーバーを設定（AccountMasterViewModel用、unbounded）
    pub fn set_account_master_receiver(
        &mut self,
        receiver: tokio::sync::mpsc::UnboundedReceiver<crate::presenter::AccountMasterViewModel>,
    ) {
        self.account_master_receiver_vm = Some(receiver);
    }

    /// 科目マスター読み込み待機中かどうか
    pub fn is_pending_account_load(&self) -> bool {
        self.pending_account_load
    }

    /// 科目マスター読み込み待機フラグをクリア
    pub fn clear_pending_account_load(&mut self) {
        self.pending_account_load = false;
    }

    /// オーバーレイセレクタの次の項目を選択
    pub fn overlay_select_next(&mut self) {
        self.overlay_selector.select_next();
    }

    /// オーバーレイセレクタの前の項目を選択
    pub fn overlay_select_previous(&mut self) {
        self.overlay_selector.select_previous();
    }

    /// オーバーレイセレクタで選択を確定
    pub fn overlay_confirm_selection(&mut self) {
        // 選択された行から科目コードを取得（借用を先に解放）
        let selected_code =
            self.overlay_selector.selected_row().and_then(|row| row.first()).cloned();

        // 科目コードをフィールドに設定
        if let Some(code) = selected_code {
            self.get_focused_field_mut().set_value(code);
        }

        self.overlay_selector.hide();
        self.input_mode = InputMode::Normal;
        self.jj_detector.reset();
    }

    /// オーバーレイセレクタをキャンセル
    pub fn overlay_cancel(&mut self) {
        self.overlay_selector.hide();
        self.input_mode = InputMode::Normal;
        self.jj_detector.reset();
    }

    /// オーバーレイセレクタが表示されているか
    pub fn is_overlay_visible(&self) -> bool {
        self.overlay_selector.is_visible()
    }

    /// 入力モードを取得
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    /// 入力モードに切り替え
    pub fn enter_modify_mode(&mut self) {
        let field = self.get_focused_field_mut();
        let input_type = field.input_type();

        match input_type {
            crate::input_mode::ModifyInputType::OverlayList => {
                // オーバーレイリスト選択モードに入る
                self.overlay_selector.start_loading();
                self.input_mode = InputMode::Modify;
                self.jj_detector.reset();
                self.pending_account_load = true;
            }
            _ => {
                // その他の入力タイプ
                field.set_focused(true);
                field.start_modify();
                self.input_mode = InputMode::Modify;
                self.jj_detector.reset();
            }
        }
    }

    /// 通常モードに切り替え
    pub fn enter_normal_mode(&mut self) {
        self.input_mode = InputMode::Normal;

        // バリデーション実行（借用を先に解放するため結果を保存）
        let validation_result = self.get_focused_field_mut().commit_buffer();
        self.get_focused_field_mut().set_focused(false);

        // バリデーション結果に応じてエラーメッセージを更新
        match validation_result {
            Ok(_) => {
                // 成功時はエラーメッセージをクリア
                self.error_message = None;
            }
            Err(error_msg) => {
                // エラー時はメッセージを保存
                self.error_message = Some(format!("入力エラー: {}", error_msg));
            }
        }

        self.jj_detector.reset();
    }

    /// 文字を入力
    pub fn input_char(&mut self, ch: char) -> bool {
        if self.input_mode.is_modify() {
            let (jj_detected, char_to_input) = self.jj_detector.process(ch);

            if jj_detected {
                // jjが検出された - Normalモードに戻る
                self.enter_normal_mode();
                return true;
            }

            // 保留中のjがあり、j以外の文字が来た場合
            if self.jj_detector.has_pending_j()
                && let Some(pending_j) = self.jj_detector.flush_pending()
            {
                self.get_focused_field_mut().append_to_buffer(pending_j);
            }

            // 今回の文字を入力
            if let Some(c) = char_to_input {
                self.get_focused_field_mut().append_to_buffer(c);
            }
        }
        false
    }

    /// バックスペース
    pub fn backspace(&mut self) {
        if self.input_mode.is_modify() {
            // 保留中のjがあればそれをクリア
            if self.jj_detector.has_pending_j() {
                self.jj_detector.flush_pending();
            } else {
                self.get_focused_field_mut().backspace_buffer();
            }
        }
    }

    /// フォーカス中のフィールドを取得
    fn get_focused_field_mut(&mut self) -> &mut InputField {
        match self.focused_field {
            SearchField::FromDate => &mut self.from_date,
            SearchField::ToDate => &mut self.to_date,
            SearchField::Description => &mut self.description,
            SearchField::AccountCode => &mut self.account_code,
            SearchField::DebitCredit => &mut self.debit_credit,
            SearchField::MinAmount => &mut self.min_amount,
            SearchField::MaxAmount => &mut self.max_amount,
        }
    }

    /// 検索条件をクリア
    pub fn clear_criteria(&mut self) {
        self.from_date.set_value(String::new());
        self.to_date.set_value(String::new());
        self.description.set_value(String::new());
        self.account_code.set_value(String::new());
        self.debit_credit.set_value(String::new());
        self.min_amount.set_value(String::new());
        self.max_amount.set_value(String::new());
        self.error_message = None;
    }

    /// 検索条件を取得
    pub fn get_criteria(&self) -> SearchCriteria {
        SearchCriteria {
            from_date: if self.from_date.value().is_empty() {
                None
            } else {
                Some(self.from_date.value().to_string())
            },
            to_date: if self.to_date.value().is_empty() {
                None
            } else {
                Some(self.to_date.value().to_string())
            },
            description: if self.description.value().is_empty() {
                None
            } else {
                Some(self.description.value().to_string())
            },
            account_code: if self.account_code.value().is_empty() {
                None
            } else {
                Some(self.account_code.value().to_string())
            },
            debit_credit: if self.debit_credit.value().is_empty() {
                None
            } else {
                Some(self.debit_credit.value().to_string())
            },
            min_amount: if self.min_amount.value().is_empty() {
                None
            } else {
                Some(self.min_amount.value().to_string())
            },
            max_amount: if self.max_amount.value().is_empty() {
                None
            } else {
                Some(self.max_amount.value().to_string())
            },
        }
    }

    /// 検索条件をDTOに変換
    pub fn to_search_criteria_dto(&self) -> javelin_application::dtos::request::SearchCriteriaDto {
        let criteria = self.get_criteria();

        // 金額文字列をf64に変換（カンマを除去）
        let parse_amount = |s: &str| -> Option<f64> { s.replace(',', "").parse::<f64>().ok() };

        // 日付をYYYYMMDD形式からYYYY-MM-DD形式に変換
        let format_date = |s: &str| -> Option<String> {
            if s.len() == 8 && s.chars().all(|c| c.is_ascii_digit()) {
                Some(format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8]))
            } else {
                Some(s.to_string())
            }
        };

        // 借方/貸方をDebit/Creditに変換
        let format_debit_credit = |s: &str| -> Option<String> {
            match s {
                "true" => Some("Credit".to_string()),
                "false" => Some("Debit".to_string()),
                _ => None,
            }
        };

        javelin_application::dtos::request::SearchCriteriaDto {
            from_date: criteria.from_date.and_then(|s| format_date(&s)),
            to_date: criteria.to_date.and_then(|s| format_date(&s)),
            description: criteria.description,
            account_code: criteria.account_code,
            debit_credit: criteria.debit_credit.and_then(|s| format_debit_credit(&s)),
            min_amount: criteria.min_amount.and_then(|s| parse_amount(&s)),
            max_amount: criteria.max_amount.and_then(|s| parse_amount(&s)),
            limit: Some(100),
            offset: Some(0),
        }
    }

    /// 選択中の仕訳IDを取得
    pub fn selected_entry_id(&self) -> Option<String> {
        self.selected_index().and_then(|idx| {
            self.current_result
                .as_ref()
                .and_then(|result| result.items.get(idx))
                .map(|item| item.entry_id.clone())
        })
    }

    /// レシーバーを取り出す（画面終了時）
    pub fn take_receivers(
        self,
    ) -> (
        mpsc::Receiver<SearchResultViewModel>,
        mpsc::Receiver<String>,
        mpsc::Receiver<String>,
        mpsc::Receiver<usize>,
    ) {
        (
            self.result_receiver,
            self.error_receiver,
            self.progress_receiver,
            self.execution_time_receiver,
        )
    }

    /// アニメーションフレームを進める
    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        // ローディング中のみアニメーションを更新
        self.result_table.tick_loading();
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // 画面を上下に分割（検索条件エリア + 結果エリア + ステータスバー）
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(19), // 検索条件
                Constraint::Min(10),    // 検索結果
                Constraint::Length(3),  // ステータスバー
            ])
            .split(area);

        // 検索条件エリア
        self.render_search_criteria(frame, chunks[0]);

        // 検索結果エリア（左右分割なし、全幅で表示）
        if self.current_result.is_none() && self.error_message.is_none() {
            // 初期状態：検索条件を指定してくださいメッセージ
            self.render_initial_message(frame, chunks[1]);
        } else {
            // 検索結果テーブル
            self.result_table.render(frame, chunks[1]);
        }

        // ステータスバー
        self.render_status_bar(frame, chunks[2]);

        // オーバーレイセレクタを最前面に描画
        if self.overlay_selector.is_visible() {
            self.overlay_selector.render(frame, area);
        }
    }

    /// 初期メッセージを描画
    fn render_initial_message(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("◆ 検索結果 ◆")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let message = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "検索条件を指定してください",
                Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
            )),
            Line::from(""),
            Line::from(Span::styled("[Enter] で検索を実行", Style::default().fg(Color::DarkGray))),
        ])
        .alignment(Alignment::Center);

        frame.render_widget(message, inner);
    }

    /// 検索条件エリアを描画
    fn render_search_criteria(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("◆ 検索条件 ◆")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // フィールドを3列に配置
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(inner);

        // 左列（各フィールドに4行確保）
        let left_fields = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Length(4), Constraint::Length(4)])
            .split(columns[0]);

        // 中央列（各フィールドに4行確保）
        let middle_fields = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Length(4)])
            .split(columns[1]);

        // 右列（各フィールドに4行確保）
        let right_fields = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Length(4)])
            .split(columns[2]);

        // フィールドを描画（左列）
        // Normalモードでもフォーカス中のフィールドをハイライト表示
        self.from_date.set_focused(self.focused_field == SearchField::FromDate);
        self.from_date.render(frame, left_fields[0], self.input_mode.is_modify());

        self.to_date.set_focused(self.focused_field == SearchField::ToDate);
        self.to_date.render(frame, left_fields[1], self.input_mode.is_modify());

        self.description.set_focused(self.focused_field == SearchField::Description);
        self.description.render(frame, left_fields[2], self.input_mode.is_modify());

        // 中央列
        self.account_code.set_focused(self.focused_field == SearchField::AccountCode);
        self.account_code.render(frame, middle_fields[0], self.input_mode.is_modify());

        self.debit_credit.set_focused(self.focused_field == SearchField::DebitCredit);
        self.debit_credit.render(frame, middle_fields[1], self.input_mode.is_modify());

        // 右列
        self.min_amount.set_focused(self.focused_field == SearchField::MinAmount);
        self.min_amount.render(frame, right_fields[0], self.input_mode.is_modify());

        self.max_amount.set_focused(self.focused_field == SearchField::MaxAmount);
        self.max_amount.render(frame, right_fields[1], self.input_mode.is_modify());

        // エラーメッセージを表示
        if let Some(error) = &self.error_message {
            let error_area =
                Rect { x: inner.x, y: inner.y + inner.height - 1, width: inner.width, height: 1 };
            let error_text = Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            frame.render_widget(error_text, error_area);
        }
    }

    /// ステータスバーを描画
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let cursor = if self.animation_frame < 30 {
            "▮"
        } else {
            " "
        };

        let mode_text = if self.input_mode.is_modify() {
            Span::styled(
                format!(" [{}] ", self.input_mode.name()),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                format!(" [{}] ", self.input_mode.name()),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        };

        let area_text = match self.focus_area {
            FocusArea::Criteria => Span::styled(
                " [検索条件] ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            FocusArea::Results => Span::styled(
                " [検索結果] ",
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
        };

        let mut status_spans = vec![
            mode_text,
            area_text,
            Span::styled("[Tab] ", Style::default().fg(Color::DarkGray)),
            Span::styled("エリア切替", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[hjkl] ", Style::default().fg(Color::DarkGray)),
            Span::styled("移動", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[i] ", Style::default().fg(Color::DarkGray)),
            Span::styled("入力", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[jj] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Normal", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Enter] ", Style::default().fg(Color::DarkGray)),
            Span::styled("検索", Style::default().fg(Color::Gray)),
        ];

        // 実行時間を表示
        if let Some(elapsed_ms) = self.execution_time_ms {
            status_spans.push(Span::styled(" │ ", Style::default().fg(Color::DarkGray)));
            status_spans.push(Span::styled(
                format!("{}ms", elapsed_ms),
                Style::default().fg(Color::Yellow),
            ));
        }

        status_spans.push(Span::styled(
            format!(" {}", cursor),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));

        let status_text = vec![Line::from(status_spans)];

        let paragraph = Paragraph::new(status_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(paragraph, area);
    }
}

/// 検索条件（View層用）
#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub description: Option<String>,
    pub account_code: Option<String>,
    pub debit_credit: Option<String>,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
}
