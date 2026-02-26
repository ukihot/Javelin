// ClosingPreparationExecutionPage - 締準備実行画面
// 責務: 締準備処理の実行とプログレス表示

use ratatui::Frame;

use crate::views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus};

pub struct ClosingPreparationExecutionPage {
    template: BatchExecutionTemplate,
}

impl ClosingPreparationExecutionPage {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("締準備処理");

        // プロセスステップを定義
        let steps = vec![
            ProcessStep::new("未登録取引確認"),
            ProcessStep::new("銀行照合差異確認"),
            ProcessStep::new("発生仕訳作成"),
            ProcessStep::new("暫定財務諸表生成"),
            ProcessStep::new("結果確認"),
        ];

        template.set_steps(steps);
        template.add_info("締準備処理画面を開きました");
        template.add_info("処理を開始するには [s] キーを押してください");

        Self { template }
    }

    /// 処理を開始
    pub fn start_execution(&mut self) {
        self.template.add_info("締準備処理を開始します...");
        self.template.update_step(0, ProcessStepStatus::Running, 0);
    }

    /// ステップの状態を更新
    pub fn update_step(&mut self, index: usize, status: ProcessStepStatus, progress: u8) {
        self.template.update_step(index, status, progress);
    }

    /// 情報メッセージを追加
    pub fn add_info(&mut self, message: impl Into<String>) {
        self.template.add_info(message);
    }

    /// エラーメッセージを追加
    pub fn add_error(&mut self, message: impl Into<String>) {
        self.template.add_error(message);
    }

    /// 次のステップを選択
    pub fn select_next(&mut self) {
        self.template.select_next();
    }

    /// 前のステップを選択
    pub fn select_previous(&mut self) {
        self.template.select_previous();
    }

    /// アニメーションフレームを更新
    pub fn tick(&mut self) {
        self.template.tick();
    }

    /// 画面を描画
    pub fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Default for ClosingPreparationExecutionPage {
    fn default() -> Self {
        Self::new()
    }
}
