// 重要性基準（Materiality）- 第3章 3.1

use crate::{error::DomainResult, value_object::ValueObject};

/// 重要性区分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialityType {
    Quantitative, // 金額的重要性
    Qualitative,  // 質的重要性
    Estimate,     // 見積重要性
}

/// 重要性判定結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialityJudgment {
    materiality_type: MaterialityType,
    requires_adjustment: bool,
    control_method: String,
}

impl ValueObject for MaterialityJudgment {
    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

impl MaterialityJudgment {
    pub fn new(
        materiality_type: MaterialityType,
        requires_adjustment: bool,
        control_method: String,
    ) -> DomainResult<Self> {
        let judgment = Self { materiality_type, requires_adjustment, control_method };
        judgment.validate()?;
        Ok(judgment)
    }
}
