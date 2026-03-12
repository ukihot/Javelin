// ProjectorRegistry - Projector管理
// すべてのProjectorを登録し、イベントを適切なProjectorに振り分ける
// 静的ディスパッチ（ジェネリクス）を使用してゼロコスト抽象化を実現

use std::sync::Arc;

use super::Projector;
use crate::{error::InfrastructureResult, event_stream::StoredEvent};

/// Projectorレジストリ（静的ディスパッチ版）
///
/// すべてのProjectorを管理し、イベントを適切なProjectorに振り分ける。
/// 各Projectorは独立して動作し、異なるRead Modelを構築する。
///
/// ジェネリクスを使用することで、コンパイル時に型が確定し、
/// 動的ディスパッチのオーバーヘッドを回避する。
pub struct ProjectorRegistry<J, A, L, T>
where
    J: Projector + 'static,
    A: Projector + 'static,
    L: Projector + 'static,
    T: Projector + 'static,
{
    journal_entry_projector: Arc<J>,
    account_master_projector: Arc<A>,
    ledger_projector: Arc<L>,
    trial_balance_projector: Arc<T>,
}

impl<J, A, L, T> ProjectorRegistry<J, A, L, T>
where
    J: Projector + 'static,
    A: Projector + 'static,
    L: Projector + 'static,
    T: Projector + 'static,
{
    /// 新しいProjectorRegistryを作成
    ///
    /// すべてのProjectorを登録する。
    pub fn new(
        journal_entry_projector: Arc<J>,
        account_master_projector: Arc<A>,
        ledger_projector: Arc<L>,
        trial_balance_projector: Arc<T>,
    ) -> Self {
        Self {
            journal_entry_projector,
            account_master_projector,
            ledger_projector,
            trial_balance_projector,
        }
    }

    /// イベントを処理
    ///
    /// すべてのProjectorに対してイベントを処理させる。
    /// 各Projectorは自身が処理すべきイベントかどうかを判定し、
    /// 処理対象の場合のみProjectionを更新する。
    ///
    /// # Arguments
    /// * `event` - 処理するイベント
    ///
    /// # Returns
    /// * `Ok(())` - 処理成功
    /// * `Err(InfrastructureError)` - 処理失敗
    pub async fn process_event(&self, event: &StoredEvent) -> InfrastructureResult<()> {
        // 各Projectorに対してイベントを処理（静的ディスパッチ）
        if self.journal_entry_projector.should_process(event) {
            self.journal_entry_projector.project(event).await?;
        }

        if self.account_master_projector.should_process(event) {
            self.account_master_projector.project(event).await?;
        }

        if self.ledger_projector.should_process(event) {
            self.ledger_projector.project(event).await?;
        }

        if self.trial_balance_projector.should_process(event) {
            self.trial_balance_projector.project(event).await?;
        }

        Ok(())
    }

    /// 登録されているProjectorの数を取得
    pub fn projector_count(&self) -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read::{
        infrastructure::db::ProjectionDb,
        projectors::{
            AccountMasterProjector, JournalEntryProjector, LedgerProjector, TrialBalanceProjector,
        },
    };

    #[tokio::test]
    async fn test_projector_registry_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let projection_db = Arc::new(ProjectionDb::new(temp_dir.path()).await.unwrap());

        let journal_entry_projector =
            Arc::new(JournalEntryProjector::new(Arc::clone(&projection_db)));
        let account_master_projector =
            Arc::new(AccountMasterProjector::new(Arc::clone(&projection_db)));
        let ledger_projector = Arc::new(LedgerProjector::new(Arc::clone(&projection_db)));
        let trial_balance_projector =
            Arc::new(TrialBalanceProjector::new(Arc::clone(&projection_db)));

        let registry = ProjectorRegistry::new(
            journal_entry_projector,
            account_master_projector,
            ledger_projector,
            trial_balance_projector,
        );

        assert_eq!(registry.projector_count(), 4);
    }
}
