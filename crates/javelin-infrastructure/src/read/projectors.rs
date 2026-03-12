// Projectors - イベントストリームからRead Modelを構築
// 各ProjectorはEventStoreの変更を購読し、対応するProjectionDBを更新する

pub mod account_master_projector;
pub mod journal_entry_projector;
pub mod ledger_projector;
pub mod registry;
pub mod trial_balance_projector;

pub use account_master_projector::AccountMasterProjector;
pub use journal_entry_projector::JournalEntryProjector;
pub use ledger_projector::LedgerProjector;
pub use registry::ProjectorRegistry;
pub use trial_balance_projector::TrialBalanceProjector;

use crate::{error::InfrastructureResult, event_stream::StoredEvent};

/// Projectorトレイト
///
/// イベントを受け取り、対応するProjectionを更新する責務を持つ。
/// 各Projectorは特定のRead Modelに対応し、関連するイベントのみを処理する。
///
/// # CQRS原則
/// - Command側（Write Model）とQuery側（Read Model）を分離
/// - Projectorはイベントストリームを購読し、Read Modelを非同期に更新
/// - 各Projectorは独立して動作し、異なるRead Modelを構築可能
pub trait Projector: Send + Sync {
    /// このProjectorが処理対象とするイベントタイプのリスト
    fn event_types(&self) -> Vec<&'static str>;

    /// イベントを処理してProjectionを更新
    ///
    /// # Arguments
    /// * `event` - 処理するイベント
    ///
    /// # Returns
    /// * `Ok(())` - 処理成功
    /// * `Err(InfrastructureError)` - 処理失敗
    fn project(
        &self,
        event: &StoredEvent,
    ) -> impl std::future::Future<Output = InfrastructureResult<()>> + Send;

    /// このProjectorが指定されたイベントを処理すべきか判定
    ///
    /// デフォルト実装では、event_types()に含まれるイベントタイプのみを処理する。
    fn should_process(&self, event: &StoredEvent) -> bool {
        self.event_types().contains(&event.event_type.as_str())
    }
}
