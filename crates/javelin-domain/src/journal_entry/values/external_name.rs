// ExternalName - 外部名称（一過性取引先用）

use crate::value_object::ValueObject;

/// 一過性の取引先の名称
/// 摘要欄に記載する相手方名（コンビニ名など）
/// 諸口（Generic Partner）の場合に使用される
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalName(String);

impl ExternalName {
    pub fn new(name: String) -> crate::error::DomainResult<Self> {
        if name.trim().is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "外部名称は空にできません".to_string(),
            ));
        }

        if name.len() > 100 {
            return Err(crate::error::DomainError::ValidationError(
                "外部名称は100文字以下である必要があります".to_string(),
            ));
        }

        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for ExternalName {
    fn validate(&self) -> crate::error::DomainResult<()> {
        if self.0.is_empty() || self.0.len() > 100 {
            return Err(crate::error::DomainError::ValidationError(
                "外部名称は1文字以上100文字以下である必要があります".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_name_creation() {
        let name = ExternalName::new("セブンイレブン".to_string()).unwrap();
        assert_eq!(name.value(), "セブンイレブン");
    }

    #[test]
    fn test_external_name_empty_error() {
        assert!(ExternalName::new("".to_string()).is_err());
    }

    #[test]
    fn test_external_name_too_long_error() {
        let long_name = "a".repeat(101);
        assert!(ExternalName::new(long_name).is_err());
    }
}
