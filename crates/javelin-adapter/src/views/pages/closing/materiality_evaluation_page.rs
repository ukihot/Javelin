// MaterialityEvaluationPage - 重要性判定画面
// 責務: 重要性判定処理の実行と結果表示

use ratatui::Frame;
use tokio::sync::mpsc;

use crate::{
    navigation::Controllers,
    presenter::MaterialityEvaluationViewModel,
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 重要性判定画面
pub struct MaterialityEvaluationPage {
    template: BatchExecutionTemplate,
    is_running: bool,
    result_rx: mpsc::UnboundedReceiver<MaterialityEvaluationViewModel>,
    progress_rx: mpsc::UnboundedReceiver<String>,
}

impl MaterialityEvaluationPage {
    pub fn new(
        result_rx: mpsc::UnboundedReceiver<MaterialityEvaluationViewModel>,
        progress_rx: mpsc::UnboundedReceiver<String>,
    ) -> Self {
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

        Self { template, is_running: false, result_rx, progress_rx }
    }

    /// 重要性判定を開始
    pub fn start_evaluation(&mut self, controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("重要性判定処理を開始します");

        // ステップ1: 財務指標取得
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        // 実際の処理はバックグラウンドで実行
        let controller = controllers.closing.clone();
        let presenter = controllers.materiality_evaluation_presenter.clone();

        tokio::spawn(async move {
            use chrono::Utc;
            use javelin_application::dtos::{EvaluateMaterialityRequest, FinancialMetrics};

            // サンプルリクエストを作成（実際には画面から入力を受け取る）
            let request = EvaluateMaterialityRequest {
                item_name: "固定資産減損".to_string(),
                amount: 10_000_000, // 1000万円
                judgment_date: Utc::now(),
                reason: "市場価値の著しい下落".to_string(),
                judged_by: "CFO".to_string(),
                financial_metrics: FinancialMetrics {
                    pretax_income: 100_000_000,  // 1億円
                    total_assets: 5_000_000_000, // 50億円
                    revenue: 3_000_000_000,      // 30億円
                    equity: 1_000_000_000,       // 10億円
                },
                qualitative_factors: None,
            };

            // コントローラーを呼び出し
            controller.handle_evaluate_materiality(request, presenter).await;
        });
    }

    /// データを更新
    pub fn update(&mut self, _controllers: &Controllers) {
        // 進捗メッセージを受信
        while let Ok(message) = self.progress_rx.try_recv() {
            self.template.add_info(message);

            // メッセージに応じてステップを更新
            if self.is_running {
                // 簡易的なステップ進行（実際にはメッセージ内容で判断）
                for i in 0..5 {
                    if self.template.get_step_status(i) == ProcessStepStatus::Waiting {
                        self.template.update_step(i, ProcessStepStatus::Running, 50);
                        break;
                    }
                }
            }
        }

        // 結果を受信
        if let Ok(result) = self.result_rx.try_recv() {
            if result.is_success {
                // 全ステップを完了にする
                for i in 0..5 {
                    self.template.update_step(i, ProcessStepStatus::Completed, 100);
                }

                self.template.add_info(format!("判定ID: {}", result.judgment_id));
                self.template.add_info(format!(
                    "判定結果: {}",
                    if result.is_material {
                        "重要"
                    } else {
                        "重要でない"
                    }
                ));
                self.template.add_info(format!("承認レベル: {}", result.approval_level));
                self.template.add_info(format!("基準タイプ: {}", result.threshold_type));
                self.template.add_info(format!("基準金額: {}円", result.threshold_amount));

                if let Some(excess_rate) = result.threshold_excess_rate {
                    self.template.add_info(format!("超過率: {:.2}%", excess_rate * 100.0));
                }
            } else if let Some(error) = result.error_message {
                // エラー時は現在のステップを失敗にする
                for i in 0..5 {
                    if self.template.get_step_status(i) == ProcessStepStatus::Running {
                        self.template.update_step(i, ProcessStepStatus::Error(error.clone()), 0);
                        break;
                    }
                }
                self.template.add_error(error);
            }
            self.is_running = false;
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
