// Javelin - 主計部業務バッチシステム
// Clean Architecture + Event Sourcing + CQRS

use javelin::{app_builder::ApplicationBuilder, app_error::AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    // color-eyreの初期化
    color_eyre::install().map_err(|e| {
        javelin::app_error::AppError::Unknown(format!("color-eyre initialization failed: {}", e))
    })?;

    // アプリケーション構築
    let app = ApplicationBuilder::new().build().await?;

    // アプリケーション実行
    app.run()?;

    Ok(())
}
