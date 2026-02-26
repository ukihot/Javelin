// 仕訳関連のドメインサービス

use crate::{
    error::{DomainError, DomainResult},
    financial_close::journal_entry::{entities::JournalEntryLine, values::EntryNumber},
};

/// 伝票番号存在確認サービス
///
/// 伝票番号の重複チェックを行うためのトレイト。
/// Infrastructure層で実装され、DIコンテナから注入される。
#[allow(async_fn_in_trait)]
pub trait EntryNumberExistenceDomainService: Send + Sync {
    /// 指定された伝票番号が既に存在するかを確認
    ///
    /// # Arguments
    /// * `entry_number` - 確認する伝票番号
    ///
    /// # Returns
    /// 存在する場合はtrue、存在しない場合はfalse
    async fn exists(&self, entry_number: &EntryNumber) -> DomainResult<bool>;
}

/// 伝票番号生成サービス
///
/// 伝票番号（VoucherNumber）は、エンドユーザが理解しやすいように
/// 年度単位で連番を付与する。これは証憑番号ではなく、伝票コードである。
#[allow(async_fn_in_trait)]
pub trait VoucherNumberDomainService: Send + Sync {
    /// 指定された会計年度の次の伝票番号を生成する
    ///
    /// # Arguments
    /// * `fiscal_year` - 会計年度（例: 2024）
    ///
    /// # Returns
    /// 生成された伝票番号（例: "V-2024-00001"）
    async fn generate_next(&self, fiscal_year: u32) -> DomainResult<String>;
}

// テスト用のモック実装
#[cfg(test)]
pub mod mock {
    use mockall::mock;

    use super::*;

    mock! {
        pub EntryNumberExistenceDomainService {}

        #[allow(async_fn_in_trait)]
        impl EntryNumberExistenceDomainService for EntryNumberExistenceDomainService {
            async fn exists(&self, entry_number: &EntryNumber) -> DomainResult<bool>;
        }
    }

    mock! {
        pub VoucherNumberDomainService {}

        #[allow(async_fn_in_trait)]
        impl VoucherNumberDomainService for VoucherNumberDomainService {
            async fn generate_next(&self, fiscal_year: u32) -> DomainResult<String>;
        }
    }
}

#[cfg(test)]
pub use mock::{MockEntryNumberExistenceDomainService, MockVoucherNumberDomainService};

/// 仕訳ドメインサービス
///
/// 仕訳に関する横断的なビジネスロジックを提供する
pub struct JournalEntryDomainService;

impl JournalEntryDomainService {
    /// 伝票番号の重複を検証
    ///
    /// # Arguments
    /// * `entry_number` - 確認する伝票番号
    /// * `checker` - 伝票番号存在確認サービス
    ///
    /// # Returns
    /// 重複がない場合はOk(())、重複がある場合はErr
    pub async fn validate_entry_number_uniqueness<C>(
        entry_number: &EntryNumber,
        checker: &C,
    ) -> DomainResult<()>
    where
        C: EntryNumberExistenceDomainService,
    {
        if checker.exists(entry_number).await? {
            return Err(DomainError::ValidationError(format!(
                "伝票番号 {} は既に使用されています",
                entry_number.value()
            )));
        }
        Ok(())
    }

    /// 借方合計と貸方合計が一致することを検証
    pub fn validate_balance(lines: &[JournalEntryLine]) -> DomainResult<()> {
        let debit_total: f64 = lines
            .iter()
            .filter(|line| line.side().is_debit())
            .map(|line| line.amount().value())
            .sum();

        let credit_total: f64 = lines
            .iter()
            .filter(|line| line.side().is_credit())
            .map(|line| line.amount().value())
            .sum();

        if (debit_total - credit_total).abs() > 0.01 {
            return Err(DomainError::InvalidAmount(format!(
                "借方合計と貸方合計が一致しません: 借方={}, 貸方={}",
                debit_total, credit_total
            )));
        }

        Ok(())
    }

    /// 反転仕訳明細を作成（取消仕訳・反対仕訳用）
    ///
    /// 借方と貸方を入れ替えた明細を生成する
    pub fn create_reversal_lines(
        original_lines: &[JournalEntryLine],
    ) -> DomainResult<Vec<JournalEntryLine>> {
        use crate::financial_close::journal_entry::values::DebitCredit;

        let mut reversed_lines = Vec::new();

        for line in original_lines {
            let reversed_side = match line.side() {
                DebitCredit::Debit => DebitCredit::Credit,
                DebitCredit::Credit => DebitCredit::Debit,
            };

            let reversed_line = JournalEntryLine::new(
                line.line_number().clone(),
                reversed_side,
                line.account_code().clone(),
                line.sub_account_code().cloned(),
                line.department_code().cloned(),
                line.amount().clone(),
                line.tax_type().clone(),
                line.tax_amount().clone(),
                line.description().cloned(),
            )?;

            reversed_lines.push(reversed_line);
        }

        Ok(reversed_lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::{
        AccountCode,
        journal_entry::values::{Amount, Currency, DebitCredit, Description, LineNumber, TaxType},
    };

    // テスト用のモック実装
    struct MockEntryNumberChecker {
        existing_numbers: Vec<String>,
    }

    impl EntryNumberExistenceDomainService for MockEntryNumberChecker {
        async fn exists(&self, entry_number: &EntryNumber) -> DomainResult<bool> {
            Ok(self.existing_numbers.contains(&entry_number.value().to_string()))
        }
    }

    #[test]
    fn test_validate_entry_number_uniqueness_success() {
        let checker = MockEntryNumberChecker { existing_numbers: vec!["JE-2024-001".to_string()] };

        let new_number = EntryNumber::new("JE-2024-002".to_string()).unwrap();
        let result = tokio_test::block_on(
            JournalEntryDomainService::validate_entry_number_uniqueness(&new_number, &checker),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_entry_number_uniqueness_duplicate() {
        let checker = MockEntryNumberChecker { existing_numbers: vec!["JE-2024-001".to_string()] };

        let duplicate_number = EntryNumber::new("JE-2024-001".to_string()).unwrap();
        let result =
            tokio_test::block_on(JournalEntryDomainService::validate_entry_number_uniqueness(
                &duplicate_number,
                &checker,
            ));

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("既に使用されています"));
    }

    #[test]
    fn test_validate_balance_success() {
        let lines = vec![
            create_test_line(1, DebitCredit::Debit, 1000.0),
            create_test_line(2, DebitCredit::Credit, 1000.0),
        ];

        let result = JournalEntryDomainService::validate_balance(&lines);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_balance_failure() {
        let lines = vec![
            create_test_line(1, DebitCredit::Debit, 1000.0),
            create_test_line(2, DebitCredit::Credit, 500.0),
        ];

        let result = JournalEntryDomainService::validate_balance(&lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_reversal_lines() {
        let original_lines = vec![
            create_test_line(1, DebitCredit::Debit, 1000.0),
            create_test_line(2, DebitCredit::Credit, 1000.0),
        ];

        let reversed = JournalEntryDomainService::create_reversal_lines(&original_lines).unwrap();

        assert_eq!(reversed.len(), 2);
        assert!(reversed[0].side().is_credit());
        assert!(reversed[1].side().is_debit());
    }

    // ヘルパー関数
    fn create_test_line(line_num: u32, side: DebitCredit, amount: f64) -> JournalEntryLine {
        JournalEntryLine::new(
            LineNumber::new(line_num).unwrap(),
            side,
            AccountCode::new("1000".to_owned()).unwrap(),
            None,
            None,
            Amount::new(amount, Currency::JPY).unwrap(),
            TaxType::OutOfScope,
            Amount::zero(Currency::JPY),
            Some(Description::new("テスト".to_string()).unwrap()),
        )
        .unwrap()
    }

    // Property-based tests
    #[cfg(test)]
    mod property_tests {
        use proptest::prelude::*;

        use super::*;

        // 有効な金額生成戦略（小数点以下2桁まで）
        fn valid_amount_strategy() -> impl Strategy<Value = f64> {
            (0i64..=100_000_000_i64).prop_map(|cents| cents as f64 / 100.0)
        }

        proptest! {
            // プロパティ1: 借方合計と貸方合計が等しい場合、常に検証成功
            #[test]
            fn prop_balanced_entries_always_valid(amount in valid_amount_strategy()) {
                let lines = vec![
                    create_test_line(1, DebitCredit::Debit, amount),
                    create_test_line(2, DebitCredit::Credit, amount),
                ];

                let result = JournalEntryDomainService::validate_balance(&lines);
                prop_assert!(result.is_ok());
            }

            // プロパティ2: 反転仕訳は元の仕訳と同じ行数
            #[test]
            fn prop_reversal_preserves_line_count(line_count in 1usize..10usize) {
                let mut lines = Vec::new();
                let amount_per_line = 100.0;

                for i in 0..line_count {
                    let side = if i % 2 == 0 { DebitCredit::Debit } else { DebitCredit::Credit };
                    lines.push(create_test_line((i + 1) as u32, side, amount_per_line));
                }

                let reversed = JournalEntryDomainService::create_reversal_lines(&lines).unwrap();
                prop_assert_eq!(reversed.len(), lines.len());
            }

            // プロパティ3: 反転仕訳の借方と貸方は元の仕訳と逆
            #[test]
            fn prop_reversal_inverts_sides(amount in valid_amount_strategy()) {
                let original = vec![
                    create_test_line(1, DebitCredit::Debit, amount),
                    create_test_line(2, DebitCredit::Credit, amount),
                ];

                let reversed = JournalEntryDomainService::create_reversal_lines(&original).unwrap();

                prop_assert!(reversed[0].side().is_credit());
                prop_assert!(reversed[1].side().is_debit());
            }

            // プロパティ4: 反転仕訳を2回適用すると元に戻る
            #[test]
            fn prop_double_reversal_returns_original(amount in valid_amount_strategy()) {
                let original = vec![
                    create_test_line(1, DebitCredit::Debit, amount),
                ];

                let reversed_once = JournalEntryDomainService::create_reversal_lines(&original).unwrap();
                let reversed_twice = JournalEntryDomainService::create_reversal_lines(&reversed_once).unwrap();

                prop_assert_eq!(original[0].side(), reversed_twice[0].side());
            }
        }
    }
}
