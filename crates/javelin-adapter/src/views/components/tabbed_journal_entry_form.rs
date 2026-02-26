// TabbedJournalEntryForm - タブ付き仕訳入力フォーム
// 責務: 複数明細行の入力をタブで切り替えて管理

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::Tabs,
};

use crate::{input_mode::ModifyInputType, views::components::InputField};

/// 仕訳明細行（UI用）
pub struct JournalEntryLineForm {
    debit_account: InputField,
    debit_amount: InputField,
    credit_account: InputField,
    credit_amount: InputField,
    description: InputField,
}

impl JournalEntryLineForm {
    pub fn new(line_number: usize) -> Self {
        Self {
            debit_account: InputField::new(format!("借方科目 #{}", line_number))
                .with_placeholder("科目コード")
                .with_input_type(ModifyInputType::OverlayList),
            debit_amount: InputField::new(format!("借方金額 #{}", line_number))
                .with_placeholder("0")
                .with_input_type(ModifyInputType::NumberOnly),
            credit_account: InputField::new(format!("貸方科目 #{}", line_number))
                .with_placeholder("科目コード")
                .with_input_type(ModifyInputType::OverlayList),
            credit_amount: InputField::new(format!("貸方金額 #{}", line_number))
                .with_placeholder("0")
                .with_input_type(ModifyInputType::NumberOnly),
            description: InputField::new(format!("摘要 #{}", line_number))
                .with_placeholder("取引内容")
                .with_input_type(ModifyInputType::Direct),
        }
    }

    pub fn debit_account(&self) -> &InputField {
        &self.debit_account
    }

    pub fn debit_account_mut(&mut self) -> &mut InputField {
        &mut self.debit_account
    }

    pub fn debit_amount(&self) -> &InputField {
        &self.debit_amount
    }

    pub fn debit_amount_mut(&mut self) -> &mut InputField {
        &mut self.debit_amount
    }

    pub fn credit_account(&self) -> &InputField {
        &self.credit_account
    }

    pub fn credit_account_mut(&mut self) -> &mut InputField {
        &mut self.credit_account
    }

    pub fn credit_amount(&self) -> &InputField {
        &self.credit_amount
    }

    pub fn credit_amount_mut(&mut self) -> &mut InputField {
        &mut self.credit_amount
    }

    pub fn description(&self) -> &InputField {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut InputField {
        &mut self.description
    }

    /// フィールドのフォーカスを更新
    pub fn update_focus(&mut self, field_index: usize) {
        self.debit_account.set_focused(field_index == 0);
        self.debit_amount.set_focused(field_index == 1);
        self.credit_account.set_focused(field_index == 2);
        self.credit_amount.set_focused(field_index == 3);
        self.description.set_focused(field_index == 4);
    }

    /// 指定されたフィールドを取得
    pub fn get_field(&self, field_index: usize) -> Option<&InputField> {
        match field_index {
            0 => Some(&self.debit_account),
            1 => Some(&self.debit_amount),
            2 => Some(&self.credit_account),
            3 => Some(&self.credit_amount),
            4 => Some(&self.description),
            _ => None,
        }
    }

    /// 指定されたフィールドを取得（可変）
    pub fn get_field_mut(&mut self, field_index: usize) -> Option<&mut InputField> {
        match field_index {
            0 => Some(&mut self.debit_account),
            1 => Some(&mut self.debit_amount),
            2 => Some(&mut self.credit_account),
            3 => Some(&mut self.credit_amount),
            4 => Some(&mut self.description),
            _ => None,
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame, chunks: &[Rect], is_in_modify: bool) {
        if chunks.len() >= 5 {
            self.debit_account.render(frame, chunks[0], is_in_modify);
            self.debit_amount.render(frame, chunks[1], is_in_modify);
            self.credit_account.render(frame, chunks[2], is_in_modify);
            self.credit_amount.render(frame, chunks[3], is_in_modify);
            self.description.render(frame, chunks[4], is_in_modify);
        }
    }
}

/// タブ付き仕訳入力フォーム
pub struct TabbedJournalEntryForm {
    lines: Vec<JournalEntryLineForm>,
    current_line_index: usize,
}

impl TabbedJournalEntryForm {
    pub fn new() -> Self {
        // 初期状態で2行の明細を用意
        let lines = vec![JournalEntryLineForm::new(1), JournalEntryLineForm::new(2)];
        Self { lines, current_line_index: 0 }
    }

    /// 明細行を追加
    pub fn add_line(&mut self) {
        let line_number = self.lines.len() + 1;
        self.lines.push(JournalEntryLineForm::new(line_number));
    }

    /// 明細行を削除（最低2行は残す）
    pub fn remove_line(&mut self) -> bool {
        if self.lines.len() > 2 {
            self.lines.pop();
            // フォーカスが削除された行にある場合は調整
            if self.current_line_index >= self.lines.len() {
                self.current_line_index = self.lines.len() - 1;
            }
            true
        } else {
            false
        }
    }

    /// 次の明細行へ移動
    pub fn next_line(&mut self) {
        if self.current_line_index < self.lines.len() - 1 {
            self.current_line_index += 1;
        }
    }

    /// 前の明細行へ移動
    pub fn previous_line(&mut self) {
        if self.current_line_index > 0 {
            self.current_line_index -= 1;
        }
    }

    /// 現在の明細行インデックスを取得
    pub fn current_line_index(&self) -> usize {
        self.current_line_index
    }

    /// 明細行数を取得
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// 現在の明細行を取得
    pub fn current_line(&self) -> &JournalEntryLineForm {
        &self.lines[self.current_line_index]
    }

    /// 現在の明細行を取得（可変）
    pub fn current_line_mut(&mut self) -> &mut JournalEntryLineForm {
        &mut self.lines[self.current_line_index]
    }

    /// すべての明細行を取得
    pub fn lines(&self) -> &[JournalEntryLineForm] {
        &self.lines
    }

    /// すべての明細行を取得（可変）
    pub fn lines_mut(&mut self) -> &mut [JournalEntryLineForm] {
        &mut self.lines
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_in_modify: bool) {
        // エリアを分割：タブバー + フォーム
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // タブバー
                Constraint::Min(0),    // フォーム
            ])
            .split(area);

        // タブタイトルを生成
        let tab_titles: Vec<Line> = self
            .lines
            .iter()
            .enumerate()
            .map(|(idx, _)| Line::from(format!("明細 #{}", idx + 1)))
            .collect();

        // タブウィジェットを描画
        let tabs = Tabs::new(tab_titles)
            .select(self.current_line_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .divider(" | ");

        frame.render_widget(tabs, chunks[0]);

        // フォームエリア
        let form_area = chunks[1];

        // 明細行のフィールド
        let constraints = vec![
            Constraint::Length(4), // 借方科目
            Constraint::Length(4), // 借方金額
            Constraint::Length(4), // 貸方科目
            Constraint::Length(4), // 貸方金額
            Constraint::Length(4), // 摘要
        ];

        let form_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(form_area);

        // 現在選択中の明細行を描画
        if let Some(line) = self.lines.get_mut(self.current_line_index) {
            line.render(frame, &form_chunks, is_in_modify);
        }
    }
}

impl Default for TabbedJournalEntryForm {
    fn default() -> Self {
        Self::new()
    }
}
