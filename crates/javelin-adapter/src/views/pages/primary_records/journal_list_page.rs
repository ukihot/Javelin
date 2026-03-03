// JournalListPage - 仕訳検索・一覧画面
// 責務: 仕訳の検索フォームと結果を一覧表示

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use uuid::Uuid;

use crate::{
    input_mode::{InputMode, JjEscapeDetector, ModifyInputType},
    presenter::{JournalEntryItemViewModel, SearchChannels, SearchResultViewModel},
    views::{
        components::{InputField, LoadingSpinner},
        layouts::FormLayout,
    },
};

/// 仕訳検索・一覧画面
pub struct JournalListPage {
    id: Uuid,
    channels: SearchChannels,
    layout: FormLayout,
    // 検索フォームフィールド
    from_date_field: InputField,
    to_date_field: InputField,
    description_field: InputField,
    account_code_field: InputField,
    debit_credit_field: InputField,
    min_amount_field: InputField,
    max_amount_field: InputField,
    // 状態
    focused_field: usize, // 0-6: 検索フィールド
    input_mode: InputMode,
    jj_detector: JjEscapeDetector,
    // 検索結果
    search_results: Option<SearchResultViewModel>,
    list_state: ListState,
    loading_spinner: LoadingSpinner,
    error_message: Option<String>,
    progress_message: Option<String>,
    execution_time: Option<usize>,
    pending_search: bool,
    is_loading: bool,
    show_form: bool,
}

impl JournalListPage {
    pub fn new(id: Uuid, channels: SearchChannels) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut layout = FormLayout::new("仕訳検索・一覧", "A-03", InputMode::Normal);
        layout.event_viewer_mut().add_info("仕訳検索画面を開きました");

        let mut page = Self {
            id,
            channels,
            layout,
            from_date_field: InputField::new("開始日付")
                .with_placeholder("YYYY-MM-DD")
                .with_input_type(ModifyInputType::Calendar),
            to_date_field: InputField::new("終了日付")
                .with_placeholder("YYYY-MM-DD")
                .with_input_type(ModifyInputType::Calendar),
            description_field: InputField::new("摘要")
                .with_placeholder("部分一致")
                .with_input_type(ModifyInputType::Direct),
            account_code_field: InputField::new("勘定科目")
                .with_placeholder("コード")
                .with_input_type(ModifyInputType::Direct),
            debit_credit_field: InputField::new("借方/貸方")
                .with_placeholder("Debit/Credit")
                .with_input_type(ModifyInputType::Direct),
            min_amount_field: InputField::new("最小金額")
                .with_placeholder("0")
                .with_input_type(ModifyInputType::NumberOnly),
            max_amount_field: InputField::new("最大金額")
                .with_placeholder("999999999")
                .with_input_type(ModifyInputType::NumberOnly),
            focused_field: 0,
            input_mode: InputMode::Normal,
            jj_detector: JjEscapeDetector::new(),
            search_results: None,
            list_state,
            loading_spinner: LoadingSpinner::new(),
            error_message: None,
            progress_message: None,
            execution_time: None,
            pending_search: false,
            is_loading: false,
            show_form: true,
        };

        page.update_focus();
        page
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn poll_search_results(&mut self) {
        while let Ok(result) = self.channels.result_rx.try_recv() {
            self.search_results = Some(result);
            self.is_loading = false;
            self.error_message = None;

            if let Some(ref results) = self.search_results
                && !results.items.is_empty()
            {
                self.list_state.select(Some(0));
            }

            self.layout.event_viewer_mut().add_info("検索が完了しました");
        }

        while let Ok(error) = self.channels.error_rx.try_recv() {
            self.error_message = Some(error.clone());
            self.is_loading = false;
            self.layout.event_viewer_mut().add_error(format!("検索エラー: {}", error));
        }

        while let Ok(progress) = self.channels.progress_rx.try_recv() {
            self.progress_message = Some(progress.clone());
            self.layout.event_viewer_mut().add_info(&progress);
        }

        while let Ok(time) = self.channels.execution_time_rx.try_recv() {
            self.execution_time = Some(time);
        }
    }

    pub fn has_pending_search(&self) -> bool {
        self.pending_search
    }

    pub fn get_search_criteria(
        &self,
    ) -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<f64>,
        Option<f64>,
    ) {
        let from_date = if self.from_date_field.value().is_empty() {
            None
        } else {
            Some(ModifyInputType::format_date_input(self.from_date_field.value()))
        };

        let to_date = if self.to_date_field.value().is_empty() {
            None
        } else {
            Some(ModifyInputType::format_date_input(self.to_date_field.value()))
        };

        let description = if self.description_field.value().is_empty() {
            None
        } else {
            Some(self.description_field.value().to_string())
        };

        let account_code = if self.account_code_field.value().is_empty() {
            None
        } else {
            Some(self.account_code_field.value().to_string())
        };

        let debit_credit = if self.debit_credit_field.value().is_empty() {
            None
        } else {
            Some(self.debit_credit_field.value().to_string())
        };

        let min_amount = if self.min_amount_field.value().is_empty() {
            None
        } else {
            self.min_amount_field.value().parse::<f64>().ok()
        };

        let max_amount = if self.max_amount_field.value().is_empty() {
            None
        } else {
            self.max_amount_field.value().parse::<f64>().ok()
        };

        (
            from_date,
            to_date,
            description,
            account_code,
            debit_credit,
            min_amount,
            max_amount,
        )
    }

    pub fn clear_pending_search(&mut self) {
        self.pending_search = false;
        self.is_loading = true;
        self.error_message = None;
        self.progress_message = None;
        self.layout.event_viewer_mut().add_info("検索を実行中...");
    }

    pub fn trigger_search(&mut self) {
        self.pending_search = true;
        self.show_form = false;
        self.layout.event_viewer_mut().add_info("検索を開始します");
    }

    pub fn toggle_form(&mut self) {
        self.show_form = !self.show_form;
        if self.show_form {
            self.layout.event_viewer_mut().add_info("検索フォームを表示しました");
        } else {
            self.layout.event_viewer_mut().add_info("検索フォームを非表示にしました");
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn enter_modify_mode(&mut self) {
        self.get_focused_field_mut().start_modify();
        self.input_mode.enter_modify();
        self.jj_detector.reset();
        self.layout.set_status(InputMode::Modify);
    }

    pub fn enter_normal_mode(&mut self) {
        if let Err(error_msg) = self.get_focused_field_mut().commit_buffer() {
            self.layout.event_viewer_mut().add_info(format!("入力エラー: {}", error_msg));
        }
        self.input_mode.enter_normal();
        self.jj_detector.reset();
        self.layout.set_status(InputMode::Normal);
    }

    pub fn cancel_modify_mode(&mut self) {
        self.get_focused_field_mut().clear_buffer();
        self.input_mode.enter_normal();
        self.jj_detector.reset();
        self.layout.set_status(InputMode::Normal);
    }

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
            let field = self.get_focused_field();
            let input_type = field.input_type();

            if input_type.is_char_allowed(ch) {
                self.get_focused_field_mut().append_to_buffer(ch);
            }
        }
    }

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

    pub fn focus_next(&mut self) {
        if self.focused_field < 6 {
            self.focused_field += 1;
        }
        self.update_focus();
    }

    pub fn focus_previous(&mut self) {
        if self.focused_field > 0 {
            self.focused_field -= 1;
        }
        self.update_focus();
    }

    fn get_focused_field(&self) -> &InputField {
        match self.focused_field {
            0 => &self.from_date_field,
            1 => &self.to_date_field,
            2 => &self.description_field,
            3 => &self.account_code_field,
            4 => &self.debit_credit_field,
            5 => &self.min_amount_field,
            6 => &self.max_amount_field,
            _ => &self.from_date_field,
        }
    }

    fn get_focused_field_mut(&mut self) -> &mut InputField {
        match self.focused_field {
            0 => &mut self.from_date_field,
            1 => &mut self.to_date_field,
            2 => &mut self.description_field,
            3 => &mut self.account_code_field,
            4 => &mut self.debit_credit_field,
            5 => &mut self.min_amount_field,
            6 => &mut self.max_amount_field,
            _ => &mut self.from_date_field,
        }
    }

    fn update_focus(&mut self) {
        self.from_date_field.set_focused(self.focused_field == 0);
        self.to_date_field.set_focused(self.focused_field == 1);
        self.description_field.set_focused(self.focused_field == 2);
        self.account_code_field.set_focused(self.focused_field == 3);
        self.debit_credit_field.set_focused(self.focused_field == 4);
        self.min_amount_field.set_focused(self.focused_field == 5);
        self.max_amount_field.set_focused(self.focused_field == 6);
    }

    pub fn select_next(&mut self) {
        if let Some(ref results) = self.search_results {
            if results.items.is_empty() {
                return;
            }

            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= results.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(ref results) = self.search_results {
            if results.items.is_empty() {
                return;
            }

            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        results.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn tick(&mut self) {
        if self.is_loading {
            self.loading_spinner.tick();
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.update_focus();

        let input_mode = self.input_mode;
        let footer_text = self.get_footer_text();

        self.layout.set_status(input_mode);

        // 必要なデータを先に取得（借用の競合を避けるため）
        let show_form = self.show_form;
        let is_loading = self.is_loading;
        let error_message = self.error_message.clone();
        let progress_message = self.progress_message.clone();
        let search_results = self.search_results.clone();
        let list_state = &mut self.list_state;
        let loading_spinner = &mut self.loading_spinner;

        self.layout.render(frame, input_mode, Some(footer_text), |frame, area| {
            if show_form {
                // フォームエリアを検索フォームと結果リストに分割
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(4), // from_date
                        Constraint::Length(4), // to_date
                        Constraint::Length(4), // description
                        Constraint::Length(4), // account_code
                        Constraint::Length(4), // debit_credit
                        Constraint::Length(4), // min_amount
                        Constraint::Length(4), // max_amount
                        Constraint::Min(0),    // 結果リスト
                    ])
                    .split(area);

                let is_in_modify = input_mode.is_modify();

                // 検索フォームフィールドを描画
                self.from_date_field.render(frame, chunks[0], is_in_modify);
                self.to_date_field.render(frame, chunks[1], is_in_modify);
                self.description_field.render(frame, chunks[2], is_in_modify);
                self.account_code_field.render(frame, chunks[3], is_in_modify);
                self.debit_credit_field.render(frame, chunks[4], is_in_modify);
                self.min_amount_field.render(frame, chunks[5], is_in_modify);
                self.max_amount_field.render(frame, chunks[6], is_in_modify);

                // 結果リストを描画
                Self::render_results_static(
                    frame,
                    chunks[7],
                    is_loading,
                    &error_message,
                    &progress_message,
                    &search_results,
                    list_state,
                    loading_spinner,
                );
            } else {
                // フォーム非表示時は結果のみ表示
                Self::render_results_static(
                    frame,
                    area,
                    is_loading,
                    &error_message,
                    &progress_message,
                    &search_results,
                    list_state,
                    loading_spinner,
                );
            }
        });
    }

    fn get_footer_text(&self) -> Line<'static> {
        if self.input_mode.is_modify() {
            Line::from(vec![
                Span::styled("[", Style::default().fg(Color::DarkGray)),
                Span::styled("jj/Esc", Style::default().fg(Color::Yellow)),
                Span::styled("] Normal  [", Style::default().fg(Color::DarkGray)),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::styled("] 確定  [", Style::default().fg(Color::DarkGray)),
                Span::styled("Backspace", Style::default().fg(Color::Yellow)),
                Span::styled("] 削除", Style::default().fg(Color::DarkGray)),
            ])
        } else {
            Line::from(vec![
                Span::styled("[", Style::default().fg(Color::DarkGray)),
                Span::styled("i", Style::default().fg(Color::Cyan)),
                Span::styled("] Modify  [", Style::default().fg(Color::DarkGray)),
                Span::styled("jk/↑↓", Style::default().fg(Color::Cyan)),
                Span::styled("] Navigate  [", Style::default().fg(Color::DarkGray)),
                Span::styled("Enter", Style::default().fg(Color::Cyan)),
                Span::styled("] Search  [", Style::default().fg(Color::DarkGray)),
                Span::styled("f", Style::default().fg(Color::Cyan)),
                Span::styled("] Toggle Form  [", Style::default().fg(Color::DarkGray)),
                Span::styled("Esc", Style::default().fg(Color::Cyan)),
                Span::styled("] Back", Style::default().fg(Color::DarkGray)),
            ])
        }
    }

    fn render_results_static(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        is_loading: bool,
        error_message: &Option<String>,
        progress_message: &Option<String>,
        search_results: &Option<SearchResultViewModel>,
        list_state: &mut ListState,
        loading_spinner: &mut LoadingSpinner,
    ) {
        if is_loading {
            let loading_message = if let Some(progress) = progress_message {
                progress.as_str()
            } else {
                "検索中..."
            };

            loading_spinner.render(frame, area, loading_message);
        } else if let Some(error) = error_message {
            let error_widget = Paragraph::new(vec![
                Line::from(Span::styled("エラーが発生しました:", Style::default().fg(Color::Red))),
                Line::from(""),
                Line::from(error.as_str()),
                Line::from(""),
                Line::from("[Enter]キーで再検索してください。"),
            ])
            .block(Block::default().borders(Borders::ALL).title("Error"))
            .style(Style::default().fg(Color::Red));
            frame.render_widget(error_widget, area);
        } else if let Some(results) = search_results {
            if results.items.is_empty() {
                let empty = Paragraph::new(vec![
                    Line::from("検索結果がありません。"),
                    Line::from(""),
                    Line::from("[f]キーで検索フォームを表示してください。"),
                ])
                .block(Block::default().borders(Borders::ALL).title("Search Results"))
                .style(Style::default().fg(Color::Gray));
                frame.render_widget(empty, area);
            } else {
                let items: Vec<ListItem> = results
                    .items
                    .iter()
                    .map(|item| Self::format_journal_entry_item(item))
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(format!("Search Results ({}件)", results.total_count)),
                    )
                    .highlight_style(
                        Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                frame.render_stateful_widget(list, area, list_state);
            }
        } else {
            let empty = Paragraph::new(vec![
                Line::from("検索条件を入力して[Enter]で検索を実行してください。"),
                Line::from(""),
                Line::from("[i] 変更モード  [jk] 移動  [Enter] 検索"),
            ])
            .block(Block::default().borders(Borders::ALL).title("Search Results"))
            .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, area);
        }
    }

    fn format_journal_entry_item(item: &JournalEntryItemViewModel) -> ListItem<'_> {
        let total: f64 = item.lines.iter().map(|line| line.amount).sum();

        let line = Line::from(vec![
            Span::styled(format!("{} ", item.transaction_date), Style::default().fg(Color::Green)),
            Span::styled(
                format!("{:<10} ", item.entry_number.as_deref().unwrap_or("-")),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(format!("{:<10} ", item.status_label), Style::default().fg(Color::Yellow)),
            Span::styled(format!("¥{:>12.0}", total), Style::default().fg(Color::White)),
        ]);

        ListItem::new(line)
    }
}
