// UserSettings - ユーザ設定ドメイン

use super::company_master::CompanyCode;
use crate::{error::DomainResult, value_object::ValueObject};

/// ユーザ設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSettings {
    default_company_code: Option<CompanyCode>,
    language: Language,
    decimal_places: DecimalPlaces,
    date_format: DateFormat,
}

impl UserSettings {
    pub fn new(
        default_company_code: Option<CompanyCode>,
        language: Language,
        decimal_places: DecimalPlaces,
        date_format: DateFormat,
    ) -> Self {
        Self { default_company_code, language, decimal_places, date_format }
    }

    pub fn default_company_code(&self) -> Option<&CompanyCode> {
        self.default_company_code.as_ref()
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn decimal_places(&self) -> &DecimalPlaces {
        &self.decimal_places
    }

    pub fn date_format(&self) -> &DateFormat {
        &self.date_format
    }

    pub fn update_default_company_code(&mut self, company_code: Option<CompanyCode>) {
        self.default_company_code = company_code;
    }

    pub fn update_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn update_decimal_places(&mut self, decimal_places: DecimalPlaces) {
        self.decimal_places = decimal_places;
    }

    pub fn update_date_format(&mut self, date_format: DateFormat) {
        self.date_format = date_format;
    }

    pub fn validate(&self) -> DomainResult<()> {
        if let Some(company_code) = &self.default_company_code {
            company_code.validate()?;
        }
        self.language.validate()?;
        self.decimal_places.validate()?;
        self.date_format.validate()?;
        Ok(())
    }
}

/// 言語設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language(String);

impl Language {
    pub fn new(language: impl Into<String>) -> DomainResult<Self> {
        let language = language.into();
        if language.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "言語設定は空にできません".to_string(),
            ));
        }
        Ok(Self(language))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Language {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "言語設定は空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

/// 小数点以下桁数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecimalPlaces(u8);

impl DecimalPlaces {
    pub fn new(decimal_places: u8) -> DomainResult<Self> {
        if decimal_places > 6 {
            return Err(crate::error::DomainError::ValidationError(
                "小数点以下桁数は0〜6の範囲で指定してください".to_string(),
            ));
        }
        Ok(Self(decimal_places))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl ValueObject for DecimalPlaces {
    fn validate(&self) -> DomainResult<()> {
        if self.0 > 6 {
            return Err(crate::error::DomainError::ValidationError(
                "小数点以下桁数は0〜6の範囲で指定してください".to_string(),
            ));
        }
        Ok(())
    }
}

/// 日付フォーマット
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateFormat(String);

impl DateFormat {
    pub fn new(format: impl Into<String>) -> DomainResult<Self> {
        let format = format.into();
        if format.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "日付フォーマットは空にできません".to_string(),
            ));
        }
        Ok(Self(format))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for DateFormat {
    fn validate(&self) -> DomainResult<()> {
        if self.0.is_empty() {
            return Err(crate::error::DomainError::ValidationError(
                "日付フォーマットは空にできません".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = Language::new("ja");
        assert!(lang.is_ok());
        assert_eq!(lang.unwrap().value(), "ja");

        assert!(Language::new("").is_err());
    }

    #[test]
    fn test_decimal_places() {
        let places = DecimalPlaces::new(2);
        assert!(places.is_ok());
        assert_eq!(places.unwrap().value(), 2);

        assert!(DecimalPlaces::new(7).is_err());
    }

    #[test]
    fn test_date_format() {
        let format = DateFormat::new("YYYY-MM-DD");
        assert!(format.is_ok());
        assert_eq!(format.unwrap().value(), "YYYY-MM-DD");

        assert!(DateFormat::new("").is_err());
    }

    // Property-based tests
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            // プロパティ1: 非空文字列の言語設定は常に作成可能
            #[test]
            fn prop_valid_language(lang in "[a-z]{2,10}") {
                let language = Language::new(lang.clone());
                prop_assert!(language.is_ok());
                let language = language.unwrap();
                prop_assert_eq!(language.value(), lang.as_str());
            }

            // プロパティ2: 空文字列の言語設定は常にエラー
            #[test]
            fn prop_empty_language_fails(_unit in Just(())) {
                let language = Language::new("");
                prop_assert!(language.is_err());
            }

            // プロパティ3: 有効範囲内の小数点以下桁数は常に作成可能
            #[test]
            fn prop_valid_decimal_places(places in 0u8..=6u8) {
                let decimal_places = DecimalPlaces::new(places);
                prop_assert!(decimal_places.is_ok());
                prop_assert_eq!(decimal_places.unwrap().value(), places);
            }

            // プロパティ4: 範囲外の小数点以下桁数は常にエラー
            #[test]
            fn prop_invalid_decimal_places(places in 7u8..=255u8) {
                let decimal_places = DecimalPlaces::new(places);
                prop_assert!(decimal_places.is_err());
            }

            // プロパティ5: 非空文字列の日付フォーマットは常に作成可能
            #[test]
            fn prop_valid_date_format(format in "[YMDHms/: ]{1,30}") {
                let date_format = DateFormat::new(format.clone());
                prop_assert!(date_format.is_ok());
                let date_format = date_format.unwrap();
                prop_assert_eq!(date_format.value(), format.as_str());
            }

            // プロパティ6: 空文字列の日付フォーマットは常にエラー
            #[test]
            fn prop_empty_date_format_fails(_unit in Just(())) {
                let date_format = DateFormat::new("");
                prop_assert!(date_format.is_err());
            }

            // プロパティ7: ユーザー設定の検証
            #[test]
            fn prop_user_settings_validation(
                lang in "[a-z]{2,10}",
                places in 0u8..=6u8,
                format in "[YMDHms/: ]{1,30}"
            ) {
                let language = Language::new(lang).unwrap();
                let decimal_places = DecimalPlaces::new(places).unwrap();
                let date_format = DateFormat::new(format).unwrap();

                let settings = UserSettings::new(
                    None,
                    language,
                    decimal_places,
                    date_format
                );

                prop_assert!(settings.validate().is_ok());
            }
        }
    }
}
