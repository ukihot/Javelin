// Javelin - 主計部業務バッチシステム
// Clean Architecture + Event Sourcing + CQRS

use clap::Parser;
use javelin::{app_builder::ApplicationBuilder, app_error::AppResult};

/// Simple CLI to select mode
#[derive(Parser)]
#[command(about = "Javelin application runner")]
struct Cli {
    /// Run in maintenance mode
    #[arg(long = "maintenance")]
    maintenance: bool,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // color-eyreの初期化
    color_eyre::install().map_err(|e| {
        javelin::app_error::AppError::Unknown(format!("color-eyre initialization failed: {}", e))
    })?;

    let cli = Cli::parse();
    let initial_route = if cli.maintenance {
        javelin_adapter::navigation::Route::MaintenanceHome
    } else {
        javelin_adapter::navigation::Route::Home
    };

    // アプリケーション構築
    let app = ApplicationBuilder::new().with_initial_route(initial_route).build().await?;

    // アプリケーション実行
    app.run()?;

    Ok(())
}
