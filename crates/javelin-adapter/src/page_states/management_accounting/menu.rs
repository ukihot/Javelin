// ManagementAccountingMenuPageState - F-01: Management Accounting Menu

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::MenuPage,
};

pub struct ManagementAccountingMenuPageState {
    page: MenuPage,
}

impl ManagementAccountingMenuPageState {
    pub fn new() -> Self {
        let menu_items = vec![
            ("Management Accounting Conversion", "管理会計変換実行"),
            ("Business Status Report", "月次業況表"),
            ("Flux Analysis", "差異分析"),
            ("KPI Trends", "KPI推移"),
            ("Financial Safety Report", "財務安全性レポート"),
            ("Profitability Report", "収益性・投資効率レポート"),
        ];

        Self { page: MenuPage::new("F. Management Accounting", &menu_items) }
    }
}

impl Default for ManagementAccountingMenuPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl PageState for ManagementAccountingMenuPageState {
    fn route(&self) -> Route {
        Route::ManagementAccountingMenu
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => return Ok(NavAction::Back),
                    KeyCode::Char('j') | KeyCode::Down => self.page.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.page.select_previous(),
                    KeyCode::Enter => {
                        let route = match self.page.selected_index() {
                            0 => Route::ManagementAccountingConversionExecution,
                            1 => Route::BusinessStatusReport,
                            2 => Route::FluxAnalysis,
                            3 => Route::KpiTrends,
                            4 => Route::FinancialSafetyReport,
                            5 => Route::ProfitabilityReport,
                            _ => continue,
                        };
                        return Ok(NavAction::Go(route));
                    }
                    _ => {}
                }
            }
        }
    }
}
