// AssetRegistrationPageState - 資産登録実行画面
// 責務: 固定資産の新規登録処理

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 資産登録実行画面
pub struct AssetRegistrationPageState {
    template: BatchExecutionTemplate,
}

impl AssetRegistrationPageState {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("資産登録実行");

        let steps = vec![
            ProcessStep::new("資産情報検証"),
            ProcessStep::new("資産台帳登録"),
            ProcessStep::new("仕訳生成"),
        ];
        template.set_steps(steps);

        Self { template }
    }

    fn execute_registration(&mut self, _controllers: &Controllers) {
        self.template.update_step(0, ProcessStepStatus::Running, 0);
        self.template.update_step(0, ProcessStepStatus::Completed, 100);

        self.template.update_step(1, ProcessStepStatus::Running, 0);
        self.template.update_step(1, ProcessStepStatus::Completed, 100);

        self.template.update_step(2, ProcessStepStatus::Running, 0);
        self.template.update_step(2, ProcessStepStatus::Completed, 100);
    }
}

impl PageState for AssetRegistrationPageState {
    fn route(&self) -> Route {
        Route::AssetRegistration
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        self.execute_registration(controllers);

        loop {
            self.template.tick();

            terminal
                .draw(|frame| {
                    self.template.render(frame);
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

                if key.code == KeyCode::Esc {
                    return Ok(NavAction::Back);
                }
            }
        }
    }
}

impl Default for AssetRegistrationPageState {
    fn default() -> Self {
        Self::new()
    }
}
