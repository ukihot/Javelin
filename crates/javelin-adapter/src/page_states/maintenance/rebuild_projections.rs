// RebuildProjectionsPageState - Projection再構築画面
// 責務: Projection再構築処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// Projection再構築画面
pub struct RebuildProjectionsPageState {
    template: BatchExecutionTemplate,
    is_running: bool,
    progress_rx: mpsc::UnboundedReceiver<String>,
}

impl RebuildProjectionsPageState {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("Projection再構築処理");

        let steps = vec![
            ProcessStep::new("イベントストア読み込み"),
            ProcessStep::new("Projection削除"),
            ProcessStep::new("勘定科目マスタ再構築"),
            ProcessStep::new("仕訳データ再構築"),
            ProcessStep::new("元帳データ再構築"),
            ProcessStep::new("完了"),
        ];
        template.set_steps(steps);

        let (progress_tx, progress_rx) = mpsc::unbounded_channel();
        std::mem::forget(progress_tx);

        Self { template, is_running: false, progress_rx }
    }

    fn start_rebuild(&mut self, _controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("Projection再構築を開始します");
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        // TODO: 実際の再構築処理を実装
        // projection_builder.rebuild_all_projections().await
    }

    fn update(&mut self) {
        while let Ok(message) = self.progress_rx.try_recv() {
            self.template.add_info(message);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl PageState for RebuildProjectionsPageState {
    fn route(&self) -> Route {
        Route::MaintenanceRebuildProjections
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
                    KeyCode::Enter => {
                        self.start_rebuild(controllers);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for RebuildProjectionsPageState {
    fn default() -> Self {
        Self::new()
    }
}
