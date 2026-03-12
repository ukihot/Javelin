// ClosingLockExecutionPage - 締ロック実行画面
// 責務: 締ロック処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 締ロック実行画面
pub struct ClosingLockExecutionPageState {
    template: BatchExecutionTemplate,
    is_running: bool,
    progress_rx: mpsc::UnboundedReceiver<String>,
    result_rx: mpsc::UnboundedReceiver<LockClosingResult>,
}

#[derive(Debug, Clone)]
struct LockClosingResult {
    success: bool,
    message: String,
    locked_period: String,
}

impl ClosingLockExecutionPageState {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("締ロック実行処理");

        let steps = vec![
            ProcessStep::new("締対象期間検証"),
            ProcessStep::new("未承認仕訳チェック"),
            ProcessStep::new("整合性検証"),
            ProcessStep::new("ロック実行"),
            ProcessStep::new("ロック状態保存"),
        ];
        template.set_steps(steps);

        let (progress_tx, progress_rx) = mpsc::unbounded_channel();
        let (result_tx, result_rx) = mpsc::unbounded_channel();

        std::mem::forget(progress_tx);
        std::mem::forget(result_tx);

        Self { template, is_running: false, progress_rx, result_rx }
    }

    fn start_lock(&mut self, _controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("締ロック処理を開始します");
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        // DISABLED: LockClosingPeriodController not available
        self.template.add_error("締ロック機能は現在無効化されています");
        self.is_running = false;

        // Original implementation - disabled
        // let controller = controllers.lock_closing_period.clone();
        //
        // tokio::spawn(async move {
        // use javelin_application::dtos::LockClosingPeriodRequest;
        //
        // let request = LockClosingPeriodRequest {
        // fiscal_year: 2024,
        // period: 1,
        // locked_by: "System".to_string(),
        // };
        //
        // match controller.lock_closing_period(request).await {
        // Ok(_response) => {}
        // Err(e) => {
        // eprintln!("締ロックエラー: {}", e);
        // }
        // }
        // });
    }

    fn update(&mut self) {
        while let Ok(message) = self.progress_rx.try_recv() {
            self.template.add_info(message);
            if self.is_running {
                for i in 0..5 {
                    if self.template.get_step_status(i) == ProcessStepStatus::Waiting {
                        self.template.update_step(i, ProcessStepStatus::Running, 50);
                        break;
                    }
                }
            }
        }

        if let Ok(result) = self.result_rx.try_recv() {
            if result.success {
                for i in 0..5 {
                    self.template.update_step(i, ProcessStepStatus::Completed, 100);
                }
                self.template.add_info(format!("ロック完了: {}", result.locked_period));
                self.template.add_info(result.message);
            } else {
                for i in 0..5 {
                    if self.template.get_step_status(i) == ProcessStepStatus::Running {
                        self.template.update_step(
                            i,
                            ProcessStepStatus::Error(result.message.clone()),
                            0,
                        );
                        break;
                    }
                }
                self.template.add_error(result.message);
            }
            self.is_running = false;
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for ClosingLockExecutionPageState {
    fn route(&self) -> Route {
        Route::ClosingLockExecution
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            self.template.tick();
            self.update();

            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.template.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.template.select_previous();
                    }
                    KeyCode::Enter => {
                        self.start_lock(controllers);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for ClosingLockExecutionPageState {
    fn default() -> Self {
        Self::new()
    }
}
