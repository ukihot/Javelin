// InputField - 入力フィールドコンポーネント
// 責務: フォーム入力欄の描画

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::input_mode::ModifyInputType;

/// 入力フィールド
pub struct InputField {
    label: String,
    value: String,
    is_focused: bool,
    is_required: bool,
    is_readonly: bool,
    placeholder: String,
    max_length: Option<usize>,
    input_type: ModifyInputType,
    // 一時入力バッファ（MODIFYモード中の入力）
    temp_buffer: String,
    // BooleanToggle用の表示ラベル（true時, false時）
    boolean_labels: Option<(String, String)>,
}

impl InputField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            is_focused: false,
            is_required: false,
            is_readonly: false,
            placeholder: String::new(),
            max_length: None,
            input_type: ModifyInputType::Direct,
            temp_buffer: String::new(),
            boolean_labels: None,
        }
    }

    /// 数値文字列をカンマ区切りでフォーマット
    fn format_number_with_commas(num_str: &str) -> String {
        if num_str.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let chars: Vec<char> = num_str.chars().collect();

        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(*ch);
        }
        result
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn required(mut self) -> Self {
        self.is_required = true;
        self
    }

    pub fn readonly(mut self) -> Self {
        self.is_readonly = true;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn with_input_type(mut self, input_type: ModifyInputType) -> Self {
        self.input_type = input_type;
        self
    }

    pub fn with_boolean_labels(
        mut self,
        true_label: impl Into<String>,
        false_label: impl Into<String>,
    ) -> Self {
        self.boolean_labels = Some((true_label.into(), false_label.into()));
        self
    }

    pub fn input_type(&self) -> ModifyInputType {
        self.input_type
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn set_value(&mut self, value: String) {
        if let Some(max_len) = self.max_length {
            // 文字数（バイト数ではない）でチェック
            if value.chars().count() <= max_len {
                self.value = value;
            }
        } else {
            self.value = value;
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// MODIFYモード開始時：一時バッファを初期化
    pub fn start_modify(&mut self) {
        self.temp_buffer = self.value.clone();
    }

    /// 一時バッファに文字を追加
    pub fn append_to_buffer(&mut self, ch: char) {
        // BooleanToggle: スペースキーで切り替え
        if self.input_type == ModifyInputType::BooleanToggle && ch == ' ' {
            self.toggle_boolean();
            return;
        }

        // Calendar入力タイプの場合は8桁まで
        if self.input_type == ModifyInputType::Calendar {
            if self.temp_buffer.len() < 8 {
                self.temp_buffer.push(ch);
            }
            return;
        }

        // その他の入力タイプ
        if let Some(max_len) = self.max_length {
            // 文字数（バイト数ではない）でチェック
            if self.temp_buffer.chars().count() < max_len {
                self.temp_buffer.push(ch);
            }
        } else {
            self.temp_buffer.push(ch);
        }
    }

    /// Boolean値を切り替え
    fn toggle_boolean(&mut self) {
        let current_value = self.temp_buffer == "true";
        self.temp_buffer = if current_value { "false" } else { "true" }.to_string();
    }

    /// 一時バッファから文字を削除
    pub fn backspace_buffer(&mut self) {
        self.temp_buffer.pop();
    }

    /// 一時バッファの内容を取得
    pub fn temp_buffer(&self) -> &str {
        &self.temp_buffer
    }

    /// jjで確定：一時バッファを値に反映
    pub fn commit_buffer(&mut self) -> Result<(), String> {
        // Calendar入力タイプの場合はバリデーション
        if self.input_type == ModifyInputType::Calendar {
            let (valid, error_msg) = ModifyInputType::validate_date_input(&self.temp_buffer);
            if !valid {
                return Err(error_msg.unwrap_or("入力エラー").to_string());
            }
        }

        self.value = self.temp_buffer.clone();
        Ok(())
    }

    /// ESCでクリア：一時バッファを破棄
    pub fn clear_buffer(&mut self) {
        self.temp_buffer.clear();
    }

    /// Boolean値を表示用にフォーマット
    fn format_boolean_display(&self, value: &str) -> String {
        if let Some((true_label, false_label)) = &self.boolean_labels {
            if value == "true" {
                true_label.clone()
            } else {
                false_label.clone()
            }
        } else {
            value.to_string()
        }
    }

    /// 描画
    pub fn render(&self, frame: &mut Frame, area: Rect, is_in_modify: bool) {
        // ラベルスタイル
        let label_style = if self.is_focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        // ラベルテキスト
        let label_text = if self.is_required {
            Line::from(vec![
                Span::styled("※", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(&self.label, label_style),
            ])
        } else {
            Line::from(Span::styled(&self.label, label_style))
        };

        // 入力欄スタイル
        let input_style = if self.is_readonly {
            Style::default().fg(Color::DarkGray).bg(Color::Black)
        } else if self.is_focused && is_in_modify {
            // MODIFYモード中は黄色背景
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else if self.is_focused {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        };

        // 表示テキスト（MODIFYモード中は一時バッファを表示）
        let display_text = if self.is_focused && is_in_modify {
            if self.temp_buffer.is_empty() && !self.placeholder.is_empty() {
                self.placeholder.to_string()
            } else {
                match self.input_type {
                    // NumberOnlyの場合は編集中もカンマ区切りで表示
                    ModifyInputType::NumberOnly if !self.temp_buffer.is_empty() => {
                        Self::format_number_with_commas(&self.temp_buffer)
                    }
                    // Calendarの場合は編集中もYYYY-MM-DD形式で表示
                    ModifyInputType::Calendar if !self.temp_buffer.is_empty() => {
                        ModifyInputType::format_date_input(&self.temp_buffer)
                    }
                    // BooleanToggleの場合はラベルで表示
                    ModifyInputType::BooleanToggle => {
                        self.format_boolean_display(&self.temp_buffer)
                    }
                    _ => self.temp_buffer.clone(),
                }
            }
        } else if self.value.is_empty() && !self.placeholder.is_empty() {
            self.placeholder.to_string()
        } else {
            match self.input_type {
                // NumberOnlyの場合はカンマ区切りで表示
                ModifyInputType::NumberOnly if !self.value.is_empty() => {
                    Self::format_number_with_commas(&self.value)
                }
                // Calendarの場合はYYYY-MM-DD形式で表示
                ModifyInputType::Calendar if !self.value.is_empty() => {
                    ModifyInputType::format_date_input(&self.value)
                }
                // BooleanToggleの場合はラベルで表示
                ModifyInputType::BooleanToggle => self.format_boolean_display(&self.value),
                _ => self.value.clone(),
            }
        };

        // カーソル
        let cursor = if self.is_focused && !self.is_readonly {
            "_" // Windowsでも安全なアンダースコア
        } else {
            ""
        };

        let input_text = format!("{}{}", display_text, cursor);

        // 2段レイアウト: ラベル + 入力欄
        let label_widget = Paragraph::new(label_text);
        let input_widget = Paragraph::new(input_text)
            .style(input_style)
            .block(Block::default().borders(Borders::ALL));

        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(1),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        frame.render_widget(label_widget, chunks[0]);
        frame.render_widget(input_widget, chunks[1]);
    }
}
