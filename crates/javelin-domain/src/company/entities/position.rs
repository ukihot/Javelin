// Position Entity - 役職エンティティ
//
// 組織内の役職（職位）を定義する。
// レベルにより上下関係を表現し、承認フローや権限判定に使用される。

use crate::{
    company::values::{PositionId, PositionLevel, PositionName},
    entity::Entity,
};

/// 役職
#[derive(Debug, Clone)]
pub struct Position {
    id: PositionId,
    name: PositionName,
    level: PositionLevel,
}

impl Entity for Position {
    type Id = PositionId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Position {
    pub fn new(id: PositionId, name: PositionName, level: PositionLevel) -> Self {
        Self { id, name, level }
    }

    pub fn name(&self) -> &PositionName {
        &self.name
    }

    pub fn level(&self) -> &PositionLevel {
        &self.level
    }

    pub fn rename(&mut self, name: PositionName) {
        self.name = name;
    }

    pub fn change_level(&mut self, level: PositionLevel) {
        self.level = level;
    }

    /// この役職が指定役職以上のレベルかどうか
    pub fn is_at_least(&self, other: &Position) -> bool {
        self.level.is_at_least(&other.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(
            PositionId::generate(),
            PositionName::new("部長").unwrap(),
            PositionLevel::new(5).unwrap(),
        );
        assert_eq!(pos.name().value(), "部長");
        assert_eq!(pos.level().value(), 5);
    }

    #[test]
    fn test_position_level_comparison() {
        let manager = Position::new(
            PositionId::generate(),
            PositionName::new("部長").unwrap(),
            PositionLevel::new(5).unwrap(),
        );
        let staff = Position::new(
            PositionId::generate(),
            PositionName::new("一般社員").unwrap(),
            PositionLevel::new(1).unwrap(),
        );
        assert!(manager.is_at_least(&staff));
        assert!(!staff.is_at_least(&manager));
    }
}
