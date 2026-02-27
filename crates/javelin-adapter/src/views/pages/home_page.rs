// HomePage - ホーム画面（業務メニュー + システムマスタメニュー）
// 責務: 業務メニューとシステムマスタメニューの表示、h/lで枠切り替え、j/kで内部フォーカス移動

use ratatui::Frame;

use crate::{
    navigation::Route,
    views::{
        components::{ListItemData, ListSelector},
        layouts::MenuLayout,
    },
};

/// メニュータイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuType {
    Business, // 業務メニュー
    System,   // システムマスタメニュー
}

pub struct HomePage {
    layout: MenuLayout,
    business_menu_selector: ListSelector,
    system_menu_selector: ListSelector,
    active_menu: MenuType,
}

impl HomePage {
    pub fn new() -> Self {
        let mut layout = MenuLayout::new("財務会計システム JAVELIN", "主計部", "担当者");
        layout.event_viewer_mut().add_info("システム起動完了");
        layout.event_viewer_mut().add_info("バージョン: 1.0.0");

        let business_menu_items = vec![
            ListItemData::new("A", "原始記録登録", "仕訳・キャッシュログ入力"),
            ListItemData::new("B", "元帳管理", "総勘定元帳・補助元帳"),
            ListItemData::new("C", "固定資産・リース", "資産台帳・減価償却・リース管理"),
            ListItemData::new("D", "月次決算", "Close Calendar - 締準備から財務諸表生成"),
            ListItemData::new("E", "財務諸表", "BS・PL・SCF・SCE・注記"),
            ListItemData::new("F", "管理会計", "業況表・KPI・差異分析"),
            ListItemData::new("G", "判断ログ・監査証跡", "会計判断記録・監査ログ"),
        ];

        let system_menu_items =
            vec![ListItemData::new("H", "マスタ管理", "勘定科目・補助科目・取引先")];

        let business_menu_selector = ListSelector::new("業務メニュー", business_menu_items);
        let system_menu_selector = ListSelector::new("システムマスタ", system_menu_items);

        Self {
            layout,
            business_menu_selector,
            system_menu_selector,
            active_menu: MenuType::Business,
        }
    }

    /// メニュー枠を切り替え（h/l）
    pub fn switch_menu(&mut self) {
        self.active_menu = match self.active_menu {
            MenuType::Business => {
                self.layout.event_viewer_mut().add_info("システムマスタメニューに切替");
                MenuType::System
            }
            MenuType::System => {
                self.layout.event_viewer_mut().add_info("業務メニューに切替");
                MenuType::Business
            }
        };
    }

    /// エラーメッセージをイベントログに追加
    pub fn add_error(&mut self, message: &str) {
        self.layout.event_viewer_mut().add_error(message);
    }

    /// 選択を上に移動
    pub fn select_previous(&mut self) {
        match self.active_menu {
            MenuType::Business => self.business_menu_selector.select_previous(),
            MenuType::System => self.system_menu_selector.select_previous(),
        }
    }

    /// 選択を下に移動
    pub fn select_next(&mut self) {
        match self.active_menu {
            MenuType::Business => self.business_menu_selector.select_next(),
            MenuType::System => self.system_menu_selector.select_next(),
        }
    }

    /// 選択された項目に対応するルートを取得
    pub fn get_selected_route(&self) -> Option<Route> {
        match self.active_menu {
            MenuType::Business => {
                self.business_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(Route::PrimaryRecordsMenu),
                    1 => Some(Route::LedgerMenu),
                    2 => Some(Route::FixedAssetsMenu),
                    3 => Some(Route::ClosingMenu),
                    4 => Some(Route::FinancialStatementsMenu),
                    5 => Some(Route::ManagementAccountingMenu),
                    6 => Some(Route::JudgmentLogList),
                    _ => None,
                })
            }
            MenuType::System => {
                self.system_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(Route::MasterManagementMenu),
                    _ => None,
                })
            }
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};

        let active_menu = self.active_menu;
        let business_selector = &mut self.business_menu_selector;
        let system_selector = &mut self.system_menu_selector;

        self.layout.render(frame, |frame, area| {
            // メインエリアを上下分割: 業務メニュー(上) + システムマスタ(下)
            let menu_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
                .split(area);

            // 業務メニュー（枠なし、ListSelectorが自分で枠を描画）
            let is_business_active = active_menu == MenuType::Business;
            business_selector.set_active(is_business_active);
            business_selector.render(frame, menu_chunks[0]);

            // システムマスタメニュー（枠なし、ListSelectorが自分で枠を描画）
            let is_system_active = active_menu == MenuType::System;
            system_selector.set_active(is_system_active);
            system_selector.render(frame, menu_chunks[1]);
        });
    }
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}
