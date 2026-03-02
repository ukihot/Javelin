// LedgerConsistencyVerificationPage - 元帳整合性検証画面
// 責務: 元帳整合性検証処理の実行と結果表示

use ratatui::Frame;
use tokio::sync::mpsc;

use crate::{
    navigation::Controllers,
    presenter::LedgerConsistencyVerificationViewModel,
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 元帳整合性検証画面
pub struct LedgerConsistencyVerificationPage {
    template: BatchExecutionTemplate,
    is_running: bool,
    result_rx: mpsc::UnboundedReceiver<LedgerConsistencyVerificationViewModel>,
    progress_rx: mpsc::UnboundedReceiver<String>,
}

impl LedgerConsistencyVerificationPage {
    pub fn new(
        result_rx: mpsc::UnboundedReceiver<LedgerConsistencyVerificationViewModel>,
        progress_rx: mpsc::UnboundedReceiver<String>,
    ) -> Self {
        let mut template = BatchExecutionTemplate::new("元帳整合性検証処理");

        let steps = vec![
            ProcessStep::new("元帳データ取得"),
            ProcessStep::new("基本整合性検証"),
            ProcessStep::new("残高変動分析"),
            ProcessStep::new("異常値検出"),
            ProcessStep::new("仮勘定分析"),
            ProcessStep::new("検証結果保存"),
        ];
        template.set_steps(steps);

        Self { template, is_running: false, result_rx, progress_rx }
    }

    pub fn start_verification(&mut self, controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("元帳整合性検証処理を開始します");
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        let controller = controllers.verify_ledger_consistency.clone();
        let presenter = controllers.ledger_consistency_verification_presenter.clone();

        tokio::spawn(async move {
            use chrono::Utc;
            use javelin_application::dtos::{VerificationLevel, VerifyLedgerConsistencyRequest};

            let request = VerifyLedgerConsistencyRequest {
                period_start: Utc::now(),
                period_end: Utc::now(),
                verification_level: VerificationLevel::Comprehensive,
                compare_with_previous_week: true,
                detect_anomalies: true,
            };

            controller.handle_verify_ledger_consistency(request, presenter).await;
        });
    }

    pub fn update(&mut self, _controllers: &Controllers) {
        while let Ok(message) = self.progress_rx.try_recv() {
            self.template.add_info(message);
            if self.is_running {
                for i in 0..6 {
                    if self.template.get_step_status(i) == ProcessStepStatus::Waiting {
                        self.template.update_step(i, ProcessStepStatus::Running, 50);
                        break;
                    }
                }
            }
        }

        if let Ok(result) = self.result_rx.try_recv() {
            if result.is_success {
                for i in 0..6 {
                    self.template.update_step(i, ProcessStepStatus::Completed, 100);
                }
                self.template.add_info(format!("検証ID: {}", result.verification_id));
                self.template.add_info(format!(
                    "検証結果: {}",
                    if result.is_consistent {
                        "整合"
                    } else {
                        "不整合"
                    }
                ));
                self.template.add_info(format!("差異の数: {}", result.discrepancy_count));
                self.template
                    .add_info(format!("異常値アラート: {}件", result.anomaly_alert_count));
                self.template.add_info(format!("仮勘定: {}件", result.temporary_account_count));
            } else if let Some(error) = result.error_message {
                for i in 0..6 {
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

    pub fn select_next(&mut self) {
        self.template.select_next();
    }

    pub fn select_previous(&mut self) {
        self.template.select_previous();
    }

    pub fn tick(&mut self) {
        self.template.tick();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}
