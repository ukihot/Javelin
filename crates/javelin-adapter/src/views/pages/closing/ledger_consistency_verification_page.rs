// LedgerConsistencyVerificationPage - 元帳整合性検証画面
// 責務: 元帳整合性検証処理の実行と結果表示

use ratatui::Frame;

use crate::{
    navigation::Controllers,
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 元帳整合性検証画面
pub struct LedgerConsistencyVerificationPage {
    template: BatchExecutionTemplate,
    is_running: bool,
}

impl LedgerConsistencyVerificationPage {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("元帳整合性検証処理");

        // プロセスステップを設定
        let steps = vec![
            ProcessStep::new("元帳データ取得"),
            ProcessStep::new("基本整合性検証"),
            ProcessStep::new("残高変動分析"),
            ProcessStep::new("異常値検出"),
            ProcessStep::new("仮勘定分析"),
            ProcessStep::new("検証結果保存"),
        ];
        template.set_steps(steps);

        Self { template, is_running: false }
    }

    /// 整合性検証を開始
    pub fn start_verification(&mut self, _controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("元帳整合性検証処理を開始します");

        // ステップ1: 元帳データ取得
        self.template.update_step(0, ProcessStepStatus::Running, 0);
        self.template.add_info("元帳データを取得中...");

        // 実際の処理はバックグラウンドで実行
        tokio::spawn(async move {
            // TODO: 実際のコントローラー呼び出し
            // let request = VerifyLedgerConsistencyRequest { ... };
            // let result = controllers.closing.verify_ledger_consistency(request).await;
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

impl Default for LedgerConsistencyVerificationPage {
    fn default() -> Self {
        Self::new()
    }
}
