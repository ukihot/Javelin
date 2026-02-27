// Application - アプリケーション本体
// 責務: ナビゲーションループの実行

use std::sync::Arc;

use javelin_adapter::{
    HomePageState, MaintenanceHomePageState, NavigationStack, PresenterRegistry,
    navigation::Controllers, views::terminal_manager::TerminalManager,
};
use tokio::sync::mpsc;

use crate::{app_error::AppResult, app_resolver::PageStateResolver};

/// アプリケーション全体の構成
pub struct Application {
    nav_stack: NavigationStack,
    controllers: Arc<Controllers>,
    terminal_manager: TerminalManager,
    resolver: PageStateResolver,
    // インフラエラー通知用
    infra_error_receiver: mpsc::UnboundedReceiver<String>,
    initial_route: javelin_adapter::navigation::Route,
}

/// 様々な依存オブジェクトをまとめた引数構造体
pub struct ApplicationConfig {
    pub controllers: Controllers,
    pub presenter_registry: Arc<PresenterRegistry>,
    pub terminal_manager: TerminalManager,
    pub infra_error_receiver: mpsc::UnboundedReceiver<String>,
    pub initial_route: javelin_adapter::navigation::Route,
}

impl Application {
    /// 新しいApplicationを作成
    pub fn new(config: ApplicationConfig) -> Self {
        let controllers_arc = Arc::new(config.controllers);
        let resolver = PageStateResolver::new(Arc::clone(&config.presenter_registry));

        Self {
            nav_stack: NavigationStack::new(),
            controllers: controllers_arc,
            terminal_manager: config.terminal_manager,
            resolver,
            infra_error_receiver: config.infra_error_receiver,
            initial_route: config.initial_route,
        }
    }

    /// アプリケーションを実行
    pub fn run(mut self) -> AppResult<()> {
        println!("\n◆ アプリケーション起動 ◆");
        println!("  Navigation: Stack-based architecture");
        println!("  Controllers: 準備完了");
        println!("  PresenterRegistry: 準備完了");
        println!("\n✓ すべてのコンポーネントが正常に初期化されました");
        println!("  メインメニューを起動します...\n");

        // Push initial page based on startup mode
        match self.initial_route {
            javelin_adapter::navigation::Route::Home => {
                self.nav_stack.push(Box::new(HomePageState::new()));
            }
            javelin_adapter::navigation::Route::MaintenanceHome => {
                self.nav_stack.push(Box::new(MaintenanceHomePageState::new()));
            }
            _ => {
                // default to Home for any other route
                self.nav_stack.push(Box::new(HomePageState::new()));
            }
        }

        // Main navigation loop
        loop {
            // インフラエラーをポーリングしてイベントログに表示
            while let Ok(error_message) = self.infra_error_receiver.try_recv() {
                if let Some(page) = self.nav_stack.current() {
                    page.on_navigation_error(&error_message);
                }
            }

            // Get current page
            let current_page = match self.nav_stack.current() {
                Some(page) => page,
                None => break, // Exit when stack is empty
            };

            // Run page event loop
            let nav_action =
                match current_page.run(self.terminal_manager.terminal_mut(), &self.controllers) {
                    Ok(action) => action,
                    Err(e) => {
                        let error_message = format!("Page error: {}", e);
                        current_page.on_navigation_error(&error_message);
                        javelin_adapter::NavAction::Back
                    }
                };

            // Handle navigation action
            match nav_action {
                javelin_adapter::NavAction::Go(route) => {
                    match self.resolver.resolve(route.clone()) {
                        Ok(new_page) => {
                            self.nav_stack.push(new_page);
                        }
                        Err(e) => {
                            let error_message = format!("Navigation error: {:?} - {}", route, e);
                            if let Some(page) = self.nav_stack.current() {
                                page.on_navigation_error(&error_message);
                            }
                        }
                    }
                }
                javelin_adapter::NavAction::Back => {
                    self.nav_stack.pop();
                }
                javelin_adapter::NavAction::None => {
                    // Continue on current page
                }
            }
        }

        println!("\n◆ アプリケーション終了 ◆");
        println!("  すべてのコンポーネントを正常にシャットダウンしました");

        Ok(())
    }
}
