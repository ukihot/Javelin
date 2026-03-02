// ComprehensiveFinancialStatementsPage - 包括的財務諸表生成画面
// 責務: 包括的財務諸表生成処理の実行と結果表示

use ratatui::Frame;
use tokio::sync::mpsc;

use crate::{
    navigation::Controllers,
    presenter::ComprehensiveFinancialStatementsViewModel,
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 包括的財務諸表生成画面
pub struct ComprehensiveFinancialStatementsPage {
    template: BatchExecutionTemplate,
    is_running: bool,
    result_rx: mpsc::UnboundedReceiver<ComprehensiveFinancialStatementsViewModel>,
    progress_rx: mpsc::UnboundedReceiver<String>,
}

impl ComprehensiveFinancialStatementsPage {
    pub fn new(
        result_rx: mpsc::UnboundedReceiver<ComprehensiveFinancialStatementsViewModel>,
        progress_rx: mpsc::UnboundedReceiver<String>,
    ) -> Self {
        let mut template = BatchExecutionTemplate::new("包括的財務諸表生成処理");

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

        Self { template, is_running: false, result_rx, progress_rx }
    }

    pub fn start_generation(&mut self, controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("包括的財務諸表生成処理を開始します");
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        let controller = controllers.generate_comprehensive_financial_statements.clone();
        let presenter = controllers.comprehensive_financial_statements_presenter.clone();

        tokio::spawn(async move {
            use chrono::Utc;
            use javelin_application::dtos::{
                GenerateComprehensiveFinancialStatementsRequest, StatementType,
            };

            let request = GenerateComprehensiveFinancialStatementsRequest {
                period_start: Utc::now(),
                period_end: Utc::now(),
                statement_types: vec![
                    StatementType::BalanceSheet,
                    StatementType::IncomeStatement,
                    StatementType::CashFlow,
                ],
                verify_consistency: true,
                perform_cross_check: true,
                approver: Some("CFO".to_string()),
            };

            controller
                .handle_generate_comprehensive_financial_statements(request, presenter)
                .await;
        });
    }

    pub fn update(&mut self, _controllers: &Controllers) {
        while let Ok(message) = self.progress_rx.try_recv() {
            self.template.add_info(message);
            if self.is_running {
                for i in 0..7 {
                    if self.template.get_step_status(i) == ProcessStepStatus::Waiting {
                        self.template.update_step(i, ProcessStepStatus::Running, 50);
                        break;
                    }
                }
            }
        }

        if let Ok(result) = self.result_rx.try_recv() {
            if result.is_success {
                for i in 0..7 {
                    self.template.update_step(i, ProcessStepStatus::Completed, 100);
                }
                self.template
                    .add_info(format!("生成した財務諸表: {}件", result.statement_count));
                if let Some(is_consistent) = result.is_consistent {
                    self.template
                        .add_info(format!("整合性: {}", if is_consistent { "OK" } else { "NG" }));
                }
                if let Some(passed) = result.cross_check_passed {
                    self.template.add_info(format!(
                        "クロスチェック: {}",
                        if passed { "成功" } else { "失敗" }
                    ));
                }
                self.template.add_info(format!("承認状態: {}", result.approval_status));
            } else if let Some(error) = result.error_message {
                for i in 0..7 {
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
