// HomePage - ホーム画面（販売管理 + 財務会計 + システムメニュー）
// 責務: 販売管理、財務会計、システムメニューの表示、h/lで枠切り替え、j/kで内部フォーカス移動

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
    Sales,   // 販売管理
    Finance, // 財務会計
    Setup,   // 導入処理
}

pub struct HomePage {
    layout: MenuLayout,
    sales_menu_selector: ListSelector,
    finance_menu_selector: ListSelector,
    setup_menu_selector: ListSelector,
    active_menu: MenuType,
}

impl HomePage {
    pub fn new() -> Self {
        let mut layout = MenuLayout::new("財務会計システム JAVELIN", "主計部", "担当者");
        layout.event_viewer_mut().add_info("システム起動完了");
        layout.event_viewer_mut().add_info("バージョン: 1.0.0");
        layout
            .event_viewer_mut()
            .add_info("[TOP] h/l: メニュー切替, r: RiskRadar(トップタブ)");

        // 販売管理メニュー
        let sales_menu_items = vec![ListItemData::new("A", "請求書発行", "請求書印刷・発行管理")];

        // 財務会計メニュー
        let finance_menu_items = vec![
            ListItemData::new("B", "原始記録登録", "仕訳・キャッシュログ入力"),
            ListItemData::new("C", "元帳管理", "総勘定元帳・補助元帳"),
            ListItemData::new("D", "固定資産・リース", "資産台帳・減価償却・リース管理"),
            ListItemData::new("E", "月次決算", "Close Calendar - 締準備から財務諸表生成"),
            ListItemData::new("F", "財務諸表", "BS・PL・SCF・SCE・注記"),
            ListItemData::new("G", "管理会計", "業況表・KPI・差異分析"),
            ListItemData::new("H", "判断ログ・監査証跡", "会計判断記録・監査ログ"),
        ];

        // 導入処理メニュー
        let setup_menu_items = vec![
            ListItemData::new("I", "勘定科目", "勘定科目マスタ管理"),
            ListItemData::new("J", "補助科目", "補助科目マスタ管理"),
            ListItemData::new("K", "取引先", "取引先マスタ管理"),
            ListItemData::new("L", "組織", "組織体制・権限管理"),
        ];

        let sales_menu_selector = ListSelector::new("販売管理", sales_menu_items);
        let finance_menu_selector = ListSelector::new("財務会計", finance_menu_items);
        let setup_menu_selector = ListSelector::new("導入処理", setup_menu_items);

        Self {
            layout,
            sales_menu_selector,
            finance_menu_selector,
            setup_menu_selector,
            active_menu: MenuType::Sales,
        }
    }

    /// メニュー枠を切り替え（h/l）
    pub fn switch_menu(&mut self) {
        self.active_menu = match self.active_menu {
            MenuType::Sales => {
                self.layout.event_viewer_mut().add_info("財務会計メニューに切替");
                MenuType::Finance
            }
            MenuType::Finance => {
                self.layout.event_viewer_mut().add_info("導入処理メニューに切替");
                MenuType::Setup
            }
            MenuType::Setup => {
                self.layout.event_viewer_mut().add_info("販売管理メニューに切替");
                MenuType::Sales
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
            MenuType::Sales => self.sales_menu_selector.select_previous(),
            MenuType::Finance => self.finance_menu_selector.select_previous(),
            MenuType::Setup => self.setup_menu_selector.select_previous(),
        }
    }

    /// 選択を下に移動
    pub fn select_next(&mut self) {
        match self.active_menu {
            MenuType::Sales => self.sales_menu_selector.select_next(),
            MenuType::Finance => self.finance_menu_selector.select_next(),
            MenuType::Setup => self.setup_menu_selector.select_next(),
        }
    }

    /// 選択された項目に対応するルートを取得
    pub fn get_selected_route(&self) -> Option<Route> {
        match self.active_menu {
            MenuType::Sales => {
                self.sales_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(Route::InvoicePrint),
                    _ => None,
                })
            }
            MenuType::Finance => {
                self.finance_menu_selector.selected_index().and_then(|idx| match idx {
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
            MenuType::Setup => {
                self.setup_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(Route::ChartOfAccounts),
                    1 => Some(Route::SubsidiaryAccounts),
                    2 => Some(Route::BusinessPartners),
                    3 => Some(Route::OrganizationManagement),
                    _ => None,
                })
            }
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};

        let active_menu = self.active_menu;
        let sales_selector = &mut self.sales_menu_selector;
        let finance_selector = &mut self.finance_menu_selector;
        let setup_selector = &mut self.setup_menu_selector;

        self.layout.render(frame, |frame, area| {
            // メインエリアを上中下3分割: 販売管理(上) + 財務会計(中) + システム(下)
            let menu_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15), // 販売管理
                    Constraint::Percentage(60), // 財務会計
                    Constraint::Percentage(25), // 導入処理
                ])
                .split(area);

            // 販売管理メニュー
            let is_sales_active = active_menu == MenuType::Sales;
            sales_selector.set_active(is_sales_active);
            sales_selector.render(frame, menu_chunks[0]);

            // 財務会計メニュー
            let is_finance_active = active_menu == MenuType::Finance;
            finance_selector.set_active(is_finance_active);
            finance_selector.render(frame, menu_chunks[1]);

            // 導入処理メニュー
            let is_setup_active = active_menu == MenuType::Setup;
            setup_selector.set_active(is_setup_active);
            setup_selector.render(frame, menu_chunks[2]);
        });
    }
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}
