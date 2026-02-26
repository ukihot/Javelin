// 伝票番号生成サービスの実装

use std::sync::Arc;

use javelin_domain::{
    error::{DomainError, DomainResult},
    financial_close::journal_entry::services::VoucherNumberDomainService,
};
use tokio::sync::Mutex;

/// 伝票番号生成サービスの実装
///
/// 年度単位で連番を管理し、エンドユーザが理解しやすい形式で伝票番号を生成する。
/// フォーマット: "V-{年度}-{5桁連番}"
/// 例: V-2024-00001, V-2024-00002, ...
pub struct VoucherNumberDomainServiceImpl {
    /// 年度ごとの最新番号を保持（実運用ではDBに永続化）
    counters: Arc<Mutex<std::collections::HashMap<u32, u32>>>,
}

impl VoucherNumberDomainServiceImpl {
    pub fn new() -> Self {
        Self { counters: Arc::new(Mutex::new(std::collections::HashMap::new())) }
    }
}

impl Default for VoucherNumberDomainServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(async_fn_in_trait)]
impl VoucherNumberDomainService for VoucherNumberDomainServiceImpl {
    async fn generate_next(&self, fiscal_year: u32) -> DomainResult<String> {
        if !(2000..=2100).contains(&fiscal_year) {
            return Err(DomainError::InvalidAmount(format!(
                "Invalid fiscal year: {}",
                fiscal_year
            )));
        }

        let mut counters = self.counters.lock().await;
        let counter = counters.entry(fiscal_year).or_insert(0);
        *counter += 1;

        Ok(format!("V-{}-{:05}", fiscal_year, *counter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_sequential_numbers() {
        let generator = VoucherNumberDomainServiceImpl::new();

        let num1 = generator.generate_next(2024).await.unwrap();
        assert_eq!(num1, "V-2024-00001");

        let num2 = generator.generate_next(2024).await.unwrap();
        assert_eq!(num2, "V-2024-00002");

        let num3 = generator.generate_next(2024).await.unwrap();
        assert_eq!(num3, "V-2024-00003");
    }

    #[tokio::test]
    async fn test_generate_different_fiscal_years() {
        let generator = VoucherNumberDomainServiceImpl::new();

        let num1 = generator.generate_next(2024).await.unwrap();
        assert_eq!(num1, "V-2024-00001");

        let num2 = generator.generate_next(2025).await.unwrap();
        assert_eq!(num2, "V-2025-00001");

        let num3 = generator.generate_next(2024).await.unwrap();
        assert_eq!(num3, "V-2024-00002");
    }

    #[tokio::test]
    async fn test_invalid_fiscal_year() {
        let generator = VoucherNumberDomainServiceImpl::new();

        let result = generator.generate_next(1999).await;
        assert!(result.is_err());

        let result = generator.generate_next(2101).await;
        assert!(result.is_err());
    }
}
