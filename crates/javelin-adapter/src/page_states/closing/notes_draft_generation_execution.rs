// NotesDraftGenerationExecutionPageState - 注記草案生成実行画面
// 責務: 注記草案生成処理の実行と結果表示

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::layouts::templates::{BatchExecutionTemplate, ProcessStep, ProcessStepStatus},
};

/// 注記草案生成実行画面
pub struct NotesDraftGenerationExecutionPageState {
    template: BatchExecutionTemplate,
    is_running: bool,
    progress_rx: mpsc::UnboundedReceiver<String>,
    result_rx: mpsc::UnboundedReceiver<GenerateNoteDraftResult>,
}

#[derive(Debug, Clone)]
struct GenerateNoteDraftResult {
    success: bool,
    message: String,
    note_count: usize,
}

impl NotesDraftGenerationExecutionPageState {
    pub fn new() -> Self {
        let mut template = BatchExecutionTemplate::new("注記草案生成実行処理");

        let steps = vec![
            ProcessStep::new("財務データ取得"),
            ProcessStep::new("会計方針注記生成"),
            ProcessStep::new("重要な会計上の見積り生成"),
            ProcessStep::new("セグメント情報生成"),
            ProcessStep::new("関連当事者取引生成"),
            ProcessStep::new("注記草案保存"),
        ];
        template.set_steps(steps);

        let (progress_tx, progress_rx) = mpsc::unbounded_channel();
        let (result_tx, result_rx) = mpsc::unbounded_channel();

        std::mem::forget(progress_tx);
        std::mem::forget(result_tx);

        Self { template, is_running: false, progress_rx, result_rx }
    }

    fn start_generation(&mut self, controllers: &Controllers) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.template.add_info("注記草案生成処理を開始します");
        self.template.update_step(0, ProcessStepStatus::Running, 0);

        let controller = controllers.generate_note_draft.clone();

        tokio::spawn(async move {
            use javelin_application::dtos::GenerateNoteDraftRequest;

            let request = GenerateNoteDraftRequest { fiscal_year: 2024, period: 1 };

            match controller.generate_note_draft(request).await {
                Ok(_response) => {}
                Err(e) => {
                    eprintln!("注記草案生成エラー: {}", e);
                }
            }
        });
    }

    fn update(&mut self) {
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
            if result.success {
                for i in 0..6 {
                    self.template.update_step(i, ProcessStepStatus::Completed, 100);
                }
                self.template.add_info(format!("生成完了: {}注記", result.note_count));
                self.template.add_info(result.message);
            } else {
                for i in 0..6 {
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

impl PageState for NotesDraftGenerationExecutionPageState {
    fn route(&self) -> Route {
        Route::NotesDraftGenerationExecution
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
                        self.start_generation(controllers);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for NotesDraftGenerationExecutionPageState {
    fn default() -> Self {
        Self::new()
    }
}
