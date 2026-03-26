// TrackingNumber - 追跡番号（BalanceTracking用）

use crate::value_object::ValueObject;

/// BalanceTrackingとの紐付けための追跡番号
/// 一過性取引先の場合、請求番号、振込参照番号、など個別の追跡キーを持つ
/// BSの残高を「いつ、誰に、いくら」で個別追跡するために使用
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrackingNumber(String);

impl TrackingNumber {
    pub fn new(number: String) -> crate::error::DomainResult<Self> {
        if number.trim().is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "追跡番号は空にできません".to_string(),
            ));
        }

        if number.len() > 50 {
            return Err(crate::error::DomainError::ValidationError(
                "追跡番号は50文字以下である必要があります".to_string(),
            ));
        }

        Ok(Self(number))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for TrackingNumber {
    fn validate(&self) -> crate::error::DomainResult<()> {
        if self.0.is_empty() || self.0.len() > 50 {
            return Err(crate::error::DomainError::ValidationError(
                "追跡番号は1文字以上50文字以下である必要があります".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking_number_creation() {
        let number = TrackingNumber::new("20260326-001".to_string()).unwrap();
        assert_eq!(number.value(), "20260326-001");
    }

    #[test]
    fn test_tracking_number_empty_error() {
        assert!(TrackingNumber::new("".to_string()).is_err());
    }

    #[test]
    fn test_tracking_number_too_long_error() {
        let long_number = "a".repeat(51);
        assert!(TrackingNumber::new(long_number).is_err());
    }
}
