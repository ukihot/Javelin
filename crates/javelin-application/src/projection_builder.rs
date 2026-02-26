// ProjectionBuilder - ReadModel生成インターフェース
// 責務: Event → Projection変換の抽象化
// 再構築: 全イベント再生で生成可能

use crate::error::ApplicationResult;

/// ProjectionBuilderトレイト
///
/// イベントストリームからRead Modelを構築するコンポーネントのインターフェース。
/// 具象実装はInfrastructure層で提供される。
///
/// 要件: 2.1, 2.2
#[async_trait::async_trait]
pub trait ProjectionBuilder: Send + Sync {
    /// イベントストリームから全Projectionを再構築
    ///
    /// EventStoreから全イベントを取得し、各イベントを順次処理して
    /// ProjectionDBを再構築する。
    ///
    /// 要件: 2.1, 2.8
    ///
    /// # Returns
    /// 成功時はOk(())、失敗時はエラー
    async fn rebuild_all_projections(&self) -> ApplicationResult<()>;

    /// 単一イベントからProjectionを更新
    ///
    /// イベント種別に応じて適切なProjection更新メソッドを呼び出す。
    ///
    /// 要件: 2.2
    ///
    /// # Arguments
    /// * `event_data` - 処理するイベントのバイト列
    ///
    /// # Returns
    /// 成功時はOk(())、失敗時はエラー
    async fn process_event(&self, event_data: &[u8]) -> ApplicationResult<()>;
}
