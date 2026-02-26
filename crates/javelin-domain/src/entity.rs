// Entity - 業務主体
// 識別: ID必須
// 変更: プロパティ変更可
// 同一性: ID基準
// フィールド型: ValueObject中心に構成
// ルール: すべてのEntityはEntityトレイトを実装しなければならない

use std::fmt::Debug;

/// すべてのEntityが実装しなければならない基底トレイト
pub trait Entity: Debug {
    type Id: EntityId;

    fn id(&self) -> &Self::Id;
    fn equals(&self, other: &Self) -> bool {
        self.id().value() == other.id().value()
    }
}

/// EntityのID型が実装しなければならないトレイト
pub trait EntityId: Debug + Clone + PartialEq + Eq {
    fn value(&self) -> &str;
}
