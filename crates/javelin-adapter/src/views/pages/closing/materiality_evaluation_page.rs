// MaterialityEvaluationPage - 重要性判定画面
// 責務: 重要性判定処理の実行と結果表示

use ratatui::Frame;

use crate::{
    navigation::Controllers,
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 重要性判定画面
pub struct MaterialityEvaluationPage {
    template: BatchExecutionTemplate,
    is_running: bool,
}

impl MaterialityEvaluationPage {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("重要性判定処理");

        // プロセスステップを設定
        let steps = vec![
            ProcessStep::new("財務指標取得"),
            ProcessStep::new("金額的重要性判定"),
            ProcessStep::new("質的重要性判定"),
            ProcessStep::new("承認レベル決定"),
            ProcessStep::new("判定結果保存"),
        ];
        template.set_steps(steps);

        Self { template, is_running: false }
    }

    /// 重要性判定を開始
    pub fn start_evaluation(&mut self, _controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("重要性判定処理を開始します");

        // ステップ1: 財務指標取得
        self.template.update_step(0, ProcessStepStatus::Running, 0);
        self.template.add_info("財務指標を取得中...");

        // 実際の処理はバックグラウンドで実行
        tokio::spawn(async move {
            // TODO: 実際のコントローラー呼び出し
            // let request = EvaluateMaterialityRequest { ... };
            // let result = controllers.closing.evaluate_materiality(request).await;
        });
    }

    /// データを更新
    pub fn update(&mut self, _controllers: &Controllers) {
        // TODO: 非同期処理の結果を受け取ってステップを更新
        // 現在はデモ用の自動進行
        if self.is_running {
            // ステップの進捗をシミュレート
            // 実際の実装では、非同期タスクからの結果を受け取る
        }
    }

    /// 次のステップを選択
    pub fn select_next(&mut self) {
        self.template.select_next();
    }

    /// 前のステップを選択
    pub fn select_previous(&mut self) {
        self.template.select_previous();
    }

    /// アニメーションフレームを進める
    pub fn tick(&mut self) {
        self.template.tick();
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Default for MaterialityEvaluationPage {
    fn default() -> Self {
        Self::new()
    }
}
