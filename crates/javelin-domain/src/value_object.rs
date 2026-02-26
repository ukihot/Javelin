// ValueObject - 値意味保持
// 識別: 値一致
// 不変性: 必須
// 検証: コンストラクタ内実施
// 演算: 業務意味単位で提供
// ルール: すべてのValueObjectはValueObjectトレイトを実装しなければならない

use std::fmt::Debug;

use crate::error::DomainResult;

/// すべてのValueObjectが実装しなければならない基底トレイト
pub trait ValueObject: Debug + Clone + PartialEq + Eq {
    /// 値の検証を行う（コンストラクタ内で呼び出される）
    fn validate(&self) -> DomainResult<()>;
}
