// TerminalManager - ターミナルの初期化とクリーンアップ
// 責務: ターミナルのライフサイクル管理

use ratatui::DefaultTerminal;

use crate::error::AdapterResult;

pub struct TerminalManager {
    terminal: DefaultTerminal,
}

impl TerminalManager {
    /// ターミナルを初期化
    pub fn new() -> AdapterResult<Self> {
        let terminal = ratatui::init();
        Ok(Self { terminal })
    }

    /// ターミナルへの参照を取得
    pub fn terminal_mut(&mut self) -> &mut DefaultTerminal {
        &mut self.terminal
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        // クリーンアップ
        ratatui::restore();
    }
}
