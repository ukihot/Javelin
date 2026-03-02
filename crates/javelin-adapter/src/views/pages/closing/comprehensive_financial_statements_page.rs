// ComprehensiveFinancialStatementsPage - 包括的財務諸表生成画面
// 責務: 包括的財務諸表生成処理の実行と結果表示

use ratatui::Frame;

use crate::{
    navigation::Controllers,
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 包括的財務諸表生成画面
pub struct ComprehensiveFinancialStatementsPage {
    template: BatchExecutionTemplate,
    is_running: bool,
}

impl ComprehensiveFinancialStatementsPage {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("包括的財務諸表生成処理");

        // プロセスステップを設定
        let steps = vec![
            ProcessStep::new("元帳データ取得"),
            ProcessStep::new("貸借対照表生成"),
            ProcessStep::new("損益計算書生成"),
            ProcessStep::new("キャッシュフロー計算書生成"),
            ProcessStep::new("整合性検証"),
            ProcessStep::new("クロスチェック"),
            ProcessStep::new("財務諸表保存"),
        ];
        template.set_steps(steps);

        Self { template, is_running: false }
    }

    /// 財務諸表生成を開始
    pub fn start_generation(&mut self, _controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("包括的財務諸表生成処理を開始します");

        // ステップ1: 元帳データ取得
        self.template.update_step(0, ProcessStepStatus::Running, 0);
        self.template.add_info("元帳データを取得中...");

        // 実際の処理はバックグラウンドで実行
        tokio::spawn(async move {
            // TODO: 実際のコントローラー呼び出し
            // let request = GenerateComprehensiveFinancialStatementsRequest { ... };
            // let result =
            // controllers.closing.generate_comprehensive_financial_statements(request).await;
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

impl Default for ComprehensiveFinancialStatementsPage {
    fn default() -> Self {
        Self::new()
    }
}
