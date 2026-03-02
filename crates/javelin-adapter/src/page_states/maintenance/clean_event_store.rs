// CleanEventStorePageState - イベントストアクリーンアップ画面
// 責務: イベントストアのクリーンアップ処理

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// イベントストアクリーンアップ画面
pub struct CleanEventStorePageState {
    template: BatchExecutionTemplate,
    is_running: bool,
    progress_rx: mpsc::UnboundedReceiver<String>,
}

impl CleanEventStorePageState {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("イベントストアクリーンアップ");

        let steps = vec![
            ProcessStep::new("イベント数確認"),
            ProcessStep::new("古いイベント検出"),
            ProcessStep::new("スナップショット作成"),
            ProcessStep::new("イベント削除"),
            ProcessStep::new("ストレージ最適化"),
        ];
        template.set_steps(steps);

        let (progress_tx, progress_rx) = mpsc::unbounded_channel();
        std::mem::forget(progress_tx);

        Self { template, is_running: false, progress_rx }
    }

    fn start_cleanup(&mut self, _controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("クリーンアップを開始します");
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        // TODO: 実際のクリーンアップ処理を実装
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

impl PageState for CleanEventStorePageState {
    fn route(&self) -> Route {
        Route::MaintenanceCleanEventStore
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
                        self.start_cleanup(controllers);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for CleanEventStorePageState {
    fn default() -> Self {
        Self::new()
    }
}
