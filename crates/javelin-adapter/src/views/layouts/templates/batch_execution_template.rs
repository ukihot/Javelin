//! BatchExecutionTemplate - バッチ実行画面の汎用テンプレート
//!
//! 複数ステップのバッチ処理画面の共通レイアウトを提供します。
//!
//! # 概要
//!
//! このテンプレートは、元帳集約処理、締準備処理、財務諸表生成などの
//! バッチ処理画面で共通して使用される以下の機能を提供します：
//!
//! - プロセスステップの表示と進捗追跡
//! - リアルタイムログ出力
//! - 実行制御（開始/停止/再試行）
//! - エラーハンドリング
//!
//! # 使用例
//!
//! ```rust
//! use javelin_adapter::views::layouts::templates::{
//!     BatchExecutionTemplate, ProcessStep, ProcessStepStatus,
//! };
//!
//! // テンプレートを作成
//! let mut template = BatchExecutionTemplate::new("元帳集約処理");
//!
//! // プロセスステップを設定
//! let steps = vec![
//!     ProcessStep::new("データ検証"),
//!     ProcessStep::new("集約処理"),
//!     ProcessStep::new("結果確認"),
//! ];
//! template.set_steps(steps);
//!
//! // ログメッセージを追加
//! template.add_info("処理を開始しました");
//!
//! // ステップの状態を更新
//! template.update_step(0, ProcessStepStatus::Running, 50);
//! template.add_info("データ検証中...");
//!
//! // 完了
//! template.update_step(0, ProcessStepStatus::Completed, 100);
//! template.add_info("データ検証が完了しました");
//! ```
//!
//! # レイアウト
//!
//! 画面は以下のように分割されます：
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │  ┌──────────────────────┐  ┌──────────────────────────────┐   │
//! │  │  Process Steps       │  │  Event Log                   │   │
//! │  │  (40% width)         │  │  (60% width)                 │   │
//! │  │                      │  │                              │   │
//! │  │  ✓ Step 1 [100%]     │  │  [12:34:56] INFO Starting... │   │
//! │  │  ▶ Step 2 [45%]      │  │  [12:34:57] INFO Step 1 OK   │   │
//! │  │  ○ Step 3 [0%]       │  │  [12:34:58] INFO Step 2...   │   │
//! │  │                      │  │                              │   │
//! │  │  ┌────────────────┐  │  │                              │   │
//! │  │  │ [s] 開始       │  │  │                              │   │
//! │  │  │ [x] 停止       │  │  │                              │   │
//! │  │  │ [r] 再試行     │  │  │                              │   │
//! │  │  └────────────────┘  │  └──────────────────────────────┘   │
//! │  └──────────────────────┘                                     │
//! │  ┌──────────────────────────────────────────────────────────┐  │
//! │  │ [↑↓] 選択  [s] 開始  [x] 停止  [r] 再試行  [Esc] 戻る │  │
//! │  └──────────────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

/// プロセスステップの状態
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStepStatus {
    /// 待機中
    Waiting,
    /// 実行中
    Running,
    /// 完了
    Completed,
    /// エラー（エラーメッセージ付き）
    Error(String),
}

/// プロセスステップ
#[derive(Debug, Clone)]
pub struct ProcessStep {
    /// ステップ名
    pub name: String,
    /// ステップの状態
    pub status: ProcessStepStatus,
    /// 進捗率（0-100）
    pub progress: u8,
}

impl ProcessStep {
    /// 新しいプロセスステップを作成
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), status: ProcessStepStatus::Waiting, progress: 0 }
    }

    /// 状態を更新
    pub fn set_status(&mut self, status: ProcessStepStatus) {
        self.status = status;
    }

    /// 進捗率を更新
    pub fn set_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
    }
}

/// ローディング状態
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    /// 読み込み中
    Loading,
    /// 読み込み完了
    Loaded,
    /// エラー
    Error(String),
}

/// バッチ実行画面のデータ項目
pub trait BatchExecutionItem {
    /// プロセスステップのリストを取得
    fn steps(&self) -> Vec<ProcessStep>;

    /// バッチ処理のタイトルを取得
    fn title(&self) -> &str;

    /// 実行中かどうかを確認
    fn is_running(&self) -> bool;

    /// エラーがあるかどうかを確認
    fn has_errors(&self) -> bool;
}

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::views::components::{EventViewer, LoadingSpinner};

/// バッチ実行テンプレート
///
/// 複数ステップのバッチ処理画面の共通レイアウトを提供します。
/// プロセスステップの表示、進捗追跡、ログ出力、実行制御を統一的に扱います。
#[allow(dead_code)]
pub struct BatchExecutionTemplate {
    /// バッチ処理のタイトル
    title: String,
    /// プロセスステップのリスト
    steps: Vec<ProcessStep>,
    /// 選択中のステップインデックス
    selected_step_index: usize,
    /// ローディング状態
    loading_state: LoadingState,
    /// イベントビューア（ログ表示用）
    event_viewer: EventViewer,
    /// ローディングスピナー
    loading_spinner: LoadingSpinner,
    /// アニメーションフレームカウンター
    animation_frame: usize,
}

impl BatchExecutionTemplate {
    /// 新しいBatchExecutionTemplateを作成
    ///
    /// # Arguments
    ///
    /// * `title` - バッチ処理のタイトル
    ///
    /// # Example
    ///
    /// ```
    /// let template = javelin_adapter::views::layouts::templates::BatchExecutionTemplate::new("元帳集約処理");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            steps: Vec::new(),
            selected_step_index: 0,
            loading_state: LoadingState::Loading,
            event_viewer: EventViewer::new(),
            loading_spinner: LoadingSpinner::new(),
            animation_frame: 0,
        }
    }

    /// プロセスステップを設定
    ///
    /// ステップリストを設定し、ローディング状態をLoadedに変更します。
    ///
    /// # Arguments
    ///
    /// * `steps` - プロセスステップのリスト
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::{BatchExecutionTemplate, ProcessStep};
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// let steps = vec![ProcessStep::new("データ検証"), ProcessStep::new("集約処理")];
    /// template.set_steps(steps);
    /// ```
    pub fn set_steps(&mut self, steps: Vec<ProcessStep>) {
        self.steps = steps;
        self.loading_state = LoadingState::Loaded;
    }

    /// 特定のステップを更新
    ///
    /// インデックスで指定されたステップの状態と進捗率を更新します。
    /// インデックスが範囲外の場合は何もしません。
    ///
    /// # Arguments
    ///
    /// * `index` - 更新するステップのインデックス
    /// * `status` - 新しい状態
    /// * `progress` - 新しい進捗率（0-100）
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::{
    ///     BatchExecutionTemplate, ProcessStep, ProcessStepStatus,
    /// };
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// template.set_steps(vec![ProcessStep::new("Step1")]);
    /// template.update_step(0, ProcessStepStatus::Running, 50);
    /// ```
    pub fn update_step(&mut self, index: usize, status: ProcessStepStatus, progress: u8) {
        if let Some(step) = self.steps.get_mut(index) {
            step.status = status;
            step.progress = progress.min(100);
        }
    }

    /// 特定のステップの状態を取得
    ///
    /// インデックスで指定されたステップの状態を取得します。
    /// インデックスが範囲外の場合はWaitingを返します。
    ///
    /// # Arguments
    ///
    /// * `index` - 取得するステップのインデックス
    ///
    /// # Returns
    ///
    /// ステップの状態
    pub fn get_step_status(&self, index: usize) -> ProcessStepStatus {
        self.steps
            .get(index)
            .map(|step| step.status.clone())
            .unwrap_or(ProcessStepStatus::Waiting)
    }

    /// ローディング状態に設定
    ///
    /// テンプレートをローディング状態に変更します。
    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    /// エラー状態に設定
    ///
    /// テンプレートをエラー状態に変更します。
    ///
    /// # Arguments
    ///
    /// * `error` - エラーメッセージ
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.loading_state = LoadingState::Error(error.into());
    }

    /// 情報メッセージを追加
    ///
    /// EventViewerに情報メッセージを追加します。
    ///
    /// # Arguments
    ///
    /// * `message` - 情報メッセージ
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::BatchExecutionTemplate;
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// template.add_info("処理を開始しました");
    /// ```
    pub fn add_info(&mut self, message: impl Into<String>) {
        self.event_viewer.add_info(message);
    }

    /// エラーメッセージを追加
    ///
    /// EventViewerにエラーメッセージを追加します。
    ///
    /// # Arguments
    ///
    /// * `message` - エラーメッセージ
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::BatchExecutionTemplate;
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// template.add_error("処理に失敗しました");
    /// ```
    pub fn add_error(&mut self, message: impl Into<String>) {
        self.event_viewer.add_error(message);
    }

    /// 次のステップを選択
    ///
    /// 選択インデックスを1つ増やします。
    /// 最後のステップの場合は何もしません。
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::{BatchExecutionTemplate, ProcessStep};
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// template.set_steps(vec![ProcessStep::new("A"), ProcessStep::new("B")]);
    /// template.select_next();
    /// ```
    pub fn select_next(&mut self) {
        if self.selected_step_index < self.steps.len().saturating_sub(1) {
            self.selected_step_index += 1;
        }
    }

    /// 前のステップを選択
    ///
    /// 選択インデックスを1つ減らします。
    /// 最初のステップの場合は何もしません。
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::{BatchExecutionTemplate, ProcessStep};
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// template.set_steps(vec![ProcessStep::new("A"), ProcessStep::new("B")]);
    /// template.select_next();
    /// template.select_previous();
    /// ```
    pub fn select_previous(&mut self) {
        if self.selected_step_index > 0 {
            self.selected_step_index -= 1;
        }
    }

    /// アニメーションフレームを更新
    ///
    /// アニメーションフレームカウンターを更新し、
    /// ローディング中の場合はLoadingSpinnerも更新します。
    ///
    /// # Example
    ///
    /// ```
    /// use javelin_adapter::views::layouts::templates::BatchExecutionTemplate;
    /// let mut template = BatchExecutionTemplate::new("処理");
    /// template.tick();
    /// ```
    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.tick();
        }
    }

    /// プロセスステップリストを描画
    fn render_process_steps(&self, frame: &mut Frame, area: Rect) {
        let mut lines = Vec::new();

        for (i, step) in self.steps.iter().enumerate() {
            let is_selected = i == self.selected_step_index;

            // ステータスシンボルと色を決定
            let (symbol, color) = match &step.status {
                ProcessStepStatus::Waiting => ("○", Color::Gray),
                ProcessStepStatus::Running => ("▶", Color::Yellow),
                ProcessStepStatus::Completed => ("✓", Color::Green),
                ProcessStepStatus::Error(_) => ("✗", Color::Red),
            };

            // プログレスバーを作成（実行中の場合）
            let progress_bar = if matches!(step.status, ProcessStepStatus::Running) {
                let filled = (step.progress as usize * 20) / 100;
                let empty = 20 - filled;
                format!(" [{}%] {}{}", step.progress, "█".repeat(filled), "░".repeat(empty))
            } else if matches!(step.status, ProcessStepStatus::Completed) {
                " [100%] ████████████████████".to_string()
            } else {
                String::new()
            };

            // 行を作成
            let line_style = if is_selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::styled(format!(" {} ", symbol), Style::default().fg(color)),
                Span::styled(&step.name, line_style),
                Span::styled(progress_bar, line_style.fg(color)),
            ])
            .style(line_style);

            lines.push(line);
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("プロセスステップ"));

        frame.render_widget(paragraph, area);
    }

    /// 実行コントロールを描画
    fn render_execution_controls(&self, frame: &mut Frame, area: Rect) {
        let controls =
            vec![Line::from(" [s] 開始"), Line::from(" [x] 停止"), Line::from(" [r] 再試行")];

        let paragraph = Paragraph::new(controls)
            .block(Block::default().borders(Borders::ALL).title("実行制御"));

        frame.render_widget(paragraph, area);
    }

    /// ステータスバーを描画
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status = "[↑↓] 選択  [s] 開始  [x] 停止  [r] 再試行  [Esc] 戻る";

        let paragraph = Paragraph::new(status).block(Block::default().borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }

    /// メインレイアウトを描画
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // ローディング中
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.render(frame, area, "読み込み中...");
            return;
        }

        // エラー表示
        if let LoadingState::Error(error) = &self.loading_state {
            let error_widget = Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("エラー"));
            frame.render_widget(error_widget, area);
            return;
        }

        // 水平分割: プロセスステップ (40%) | イベントログ (60%)
        let horizontal_chunks =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(area);

        // 左側の垂直分割: ステップリスト | コントロール | ステータスバー
        let left_chunks =
            Layout::vertical([Constraint::Min(10), Constraint::Length(5), Constraint::Length(3)])
                .split(horizontal_chunks[0]);

        // 各セクションを描画
        self.render_process_steps(frame, left_chunks[0]);
        self.render_execution_controls(frame, left_chunks[1]);
        self.render_status_bar(frame, left_chunks[2]);

        // 右側: イベントビューア
        self.event_viewer.render(frame, horizontal_chunks[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_step_new() {
        let step = ProcessStep::new("テストステップ");
        assert_eq!(step.name, "テストステップ");
        assert_eq!(step.status, ProcessStepStatus::Waiting);
        assert_eq!(step.progress, 0);
    }

    #[test]
    fn test_process_step_set_status() {
        let mut step = ProcessStep::new("テスト");
        step.set_status(ProcessStepStatus::Running);
        assert_eq!(step.status, ProcessStepStatus::Running);
    }

    #[test]
    fn test_process_step_set_progress() {
        let mut step = ProcessStep::new("テスト");
        step.set_progress(50);
        assert_eq!(step.progress, 50);

        // 100を超える値は100にクランプされる
        step.set_progress(150);
        assert_eq!(step.progress, 100);
    }

    #[test]
    fn test_batch_execution_template_new() {
        let template = BatchExecutionTemplate::new("テスト処理");
        assert_eq!(template.title, "テスト処理");
        assert_eq!(template.steps.len(), 0);
        assert_eq!(template.selected_step_index, 0);
        assert_eq!(template.loading_state, LoadingState::Loading);
    }

    #[test]
    fn test_set_steps() {
        let mut template = BatchExecutionTemplate::new("テスト");
        let steps = vec![ProcessStep::new("ステップ1"), ProcessStep::new("ステップ2")];

        template.set_steps(steps);
        assert_eq!(template.steps.len(), 2);
        assert_eq!(template.loading_state, LoadingState::Loaded);
    }

    #[test]
    fn test_update_step() {
        let mut template = BatchExecutionTemplate::new("テスト");
        let steps = vec![ProcessStep::new("ステップ1"), ProcessStep::new("ステップ2")];
        template.set_steps(steps);

        template.update_step(0, ProcessStepStatus::Running, 50);
        assert_eq!(template.steps[0].status, ProcessStepStatus::Running);
        assert_eq!(template.steps[0].progress, 50);

        // 範囲外のインデックスは無視される
        template.update_step(10, ProcessStepStatus::Completed, 100);
        // パニックしないことを確認
    }

    #[test]
    fn test_update_step_progress_clamping() {
        let mut template = BatchExecutionTemplate::new("テスト");
        let steps = vec![ProcessStep::new("ステップ1")];
        template.set_steps(steps);

        template.update_step(0, ProcessStepStatus::Running, 150);
        assert_eq!(template.steps[0].progress, 100);
    }

    #[test]
    fn test_set_loading() {
        let mut template = BatchExecutionTemplate::new("テスト");
        template.set_steps(vec![ProcessStep::new("ステップ1")]);
        assert_eq!(template.loading_state, LoadingState::Loaded);

        template.set_loading();
        assert_eq!(template.loading_state, LoadingState::Loading);
    }

    #[test]
    fn test_set_error() {
        let mut template = BatchExecutionTemplate::new("テスト");
        template.set_error("エラーが発生しました");

        match &template.loading_state {
            LoadingState::Error(msg) => assert_eq!(msg, "エラーが発生しました"),
            _ => panic!("Expected Error state"),
        }
    }

    #[test]
    fn test_select_next() {
        let mut template = BatchExecutionTemplate::new("テスト");
        let steps = vec![
            ProcessStep::new("ステップ1"),
            ProcessStep::new("ステップ2"),
            ProcessStep::new("ステップ3"),
        ];
        template.set_steps(steps);

        assert_eq!(template.selected_step_index, 0);

        template.select_next();
        assert_eq!(template.selected_step_index, 1);

        template.select_next();
        assert_eq!(template.selected_step_index, 2);

        // 最後のステップで止まる
        template.select_next();
        assert_eq!(template.selected_step_index, 2);
    }

    #[test]
    fn test_select_previous() {
        let mut template = BatchExecutionTemplate::new("テスト");
        let steps = vec![
            ProcessStep::new("ステップ1"),
            ProcessStep::new("ステップ2"),
            ProcessStep::new("ステップ3"),
        ];
        template.set_steps(steps);
        template.selected_step_index = 2;

        template.select_previous();
        assert_eq!(template.selected_step_index, 1);

        template.select_previous();
        assert_eq!(template.selected_step_index, 0);

        // 最初のステップで止まる
        template.select_previous();
        assert_eq!(template.selected_step_index, 0);
    }

    #[test]
    fn test_select_navigation_empty_steps() {
        let mut template = BatchExecutionTemplate::new("テスト");

        // 空のステップリストでパニックしないことを確認
        template.select_next();
        template.select_previous();
        assert_eq!(template.selected_step_index, 0);
    }

    #[test]
    fn test_tick() {
        let mut template = BatchExecutionTemplate::new("テスト");

        assert_eq!(template.animation_frame, 0);

        template.tick();
        assert_eq!(template.animation_frame, 1);

        // 60でラップする
        template.animation_frame = 59;
        template.tick();
        assert_eq!(template.animation_frame, 0);
    }

    #[test]
    fn test_process_step_status_equality() {
        assert_eq!(ProcessStepStatus::Waiting, ProcessStepStatus::Waiting);
        assert_eq!(ProcessStepStatus::Running, ProcessStepStatus::Running);
        assert_eq!(ProcessStepStatus::Completed, ProcessStepStatus::Completed);
        assert_eq!(
            ProcessStepStatus::Error("test".to_string()),
            ProcessStepStatus::Error("test".to_string())
        );
        assert_ne!(
            ProcessStepStatus::Error("test1".to_string()),
            ProcessStepStatus::Error("test2".to_string())
        );
    }
}
