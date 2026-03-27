// RiskRadarPageState - Top-level risk radar screen
// Route-level切替で Home と RiskRadar を切り替え可能にする

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use javelin_application::query_service::{ComplianceRiskSnapshot, RiskMeasurement};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::compliance_risk_radar::RiskRadarScreen,
};

pub struct RiskRadarPageState {
    snapshot: ComplianceRiskSnapshot,
}

impl Default for RiskRadarPageState {
    fn default() -> Self {
        Self::new()
    }
}

impl RiskRadarPageState {
    pub fn new() -> Self {
        Self { snapshot: Self::default_snapshot() }
    }

    fn default_snapshot() -> ComplianceRiskSnapshot {
        let measurements = vec![
            RiskMeasurement {
                indicator_type: "JournalIntegrity".to_string(),
                name: "仕訳行為区分違反".to_string(),
                score: "15.0".to_string(),
                level: "Low".to_string(),
                details: "仕訳差異が最小です".to_string(),
                count: 1,
            },
            RiskMeasurement {
                indicator_type: "JudgmentLogDeficiency".to_string(),
                name: "判断ログ欠如".to_string(),
                score: "25.0".to_string(),
                level: "Medium".to_string(),
                details: "ログが欠落しています".to_string(),
                count: 2,
            },
            RiskMeasurement {
                indicator_type: "CarryingAmountDiscrepancy".to_string(),
                name: "帳簿価額不整合".to_string(),
                score: "8.0".to_string(),
                level: "Low".to_string(),
                details: "残高不整合は僅少です".to_string(),
                count: 0,
            },
            RiskMeasurement {
                indicator_type: "MaterialityExceeded".to_string(),
                name: "重要性超過".to_string(),
                score: "35.0".to_string(),
                level: "High".to_string(),
                details: "重要性閾値を超えています".to_string(),
                count: 3,
            },
            RiskMeasurement {
                indicator_type: "IFRS15Risk".to_string(),
                name: "IFRS15リスク".to_string(),
                score: "20.0".to_string(),
                level: "Medium".to_string(),
                details: "収益認識処理を確認".to_string(),
                count: 1,
            },
            RiskMeasurement {
                indicator_type: "ECLStageDrift".to_string(),
                name: "ECL段階遷移".to_string(),
                score: "45.0".to_string(),
                level: "High".to_string(),
                details: "ステージ遷移を要監視".to_string(),
                count: 4,
            },
            RiskMeasurement {
                indicator_type: "PostLockAdjustment".to_string(),
                name: "締後補正連鎖".to_string(),
                score: "55.0".to_string(),
                level: "Critical".to_string(),
                details: "締後補正が多発".to_string(),
                count: 5,
            },
            RiskMeasurement {
                indicator_type: "IAS21Compliance".to_string(),
                name: "IAS21外貨".to_string(),
                score: "12.0".to_string(),
                level: "Low".to_string(),
                details: "為替換算が安定".to_string(),
                count: 1,
            },
        ];

        ComplianceRiskSnapshot {
            snapshot_id: "risk-radar-example-001".to_string(),
            period_year: 2026,
            period_month: 3,
            measurements,
            overall_risk_level: "High".to_string(),
            average_risk_score: "28.8".to_string(),
            maximum_risk_score: "55.0".to_string(),
            critical_count: 1,
            captured_at: "2026-03-27T12:00:00Z".to_string(),
            reviewed_by: Some("監査担当者".to_string()),
            reviewed_at: Some("2026-03-27T12:10:00Z".to_string()),
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        RiskRadarScreen::render(frame, &self.snapshot);
    }
}

impl PageState for RiskRadarPageState {
    fn route(&self) -> Route {
        Route::RiskRadar
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(NavAction::Back),
                    KeyCode::Char('h') => return Ok(NavAction::Go(Route::Home)),
                    KeyCode::Char('r') => return Ok(NavAction::None),
                    _ => {}
                }
            }
        }
    }
}
