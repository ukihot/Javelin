// ClosingRepository - 月次決算専用リポジトリ

use super::RepositoryBase;
use crate::financial_close::closing_events::ClosingEvent;

/// 月次決算リポジトリトレイト
///
/// ClosingEventを扱う専用リポジトリ。
/// RepositoryBaseを継承し、イベントソーシングの基本機能を提供。
#[allow(async_fn_in_trait)]
pub trait ClosingRepository: RepositoryBase<Event = ClosingEvent> + Send + Sync {
    // 必要に応じて月次決算固有のメソッドを追加可能
}
