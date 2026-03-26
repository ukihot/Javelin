// JournalEntryLine Entity - 仕訳明細エンティティ
//
// 子エンティティ：JournalEntry 集約内でのみインスタンス化可能（new は pub(super)）
// ただしstructと gettersは pub で、集約内他モジュール（domain_events等）からアクセス可能

use crate::{
    chart_of_accounts::values::AccountCode,
    common::Money,
    error::DomainResult,
    journal_entry::values::{
        DebitCredit, DepartmentCode, Description, ExternalName, LineNumber, SubAccountCode,
        TaxType, TrackingNumber,
    },
    partner::PartnerId,
    value_object::ValueObject,
};

/// 仕訳明細
///
/// 子エンティティ：ルート集約（JournalEntry）経由でのみ操作される
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntryLine {
    line_number: LineNumber,
    side: DebitCredit,
    account_code: AccountCode,
    sub_account_code: Option<SubAccountCode>,
    department_code: Option<DepartmentCode>,
    amount: Money,
    tax_type: TaxType,
    tax_amount: Money,
    description: Option<Description>,
    partner_id: Option<PartnerId>, // BS科目の場合、取引先ID（一過性の場合はNone）
    external_name: Option<ExternalName>, // 一過性取引先の場合の外部名称（摘要欄に記載）
    tracking_number: Option<TrackingNumber>, // BalanceTrackingとの紐付けキー
}

impl JournalEntryLine {
    /// 仕訳明細を新規作成
    ///
    /// # 制限
    /// - この関数は pub(super) - 集約内でのみ呼び出し可能
    /// - ドメイン上の整合性検証（validate）を必ず行う
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        line_number: LineNumber,
        side: DebitCredit,
        account_code: AccountCode,
        sub_account_code: Option<SubAccountCode>,
        department_code: Option<DepartmentCode>,
        amount: Money,
        tax_type: TaxType,
        tax_amount: Money,
        description: Option<Description>,
        partner_id: Option<PartnerId>,
        external_name: Option<ExternalName>,
        tracking_number: Option<TrackingNumber>,
    ) -> DomainResult<Self> {
        let line = Self {
            line_number,
            side,
            account_code,
            sub_account_code,
            department_code,
            amount,
            tax_type,
            tax_amount,
            description,
            partner_id,
            external_name,
            tracking_number,
        };
        line.validate()?;
        Ok(line)
    }

    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }

    pub fn side(&self) -> &DebitCredit {
        &self.side
    }

    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }

    pub fn sub_account_code(&self) -> Option<&SubAccountCode> {
        self.sub_account_code.as_ref()
    }

    pub fn department_code(&self) -> Option<&DepartmentCode> {
        self.department_code.as_ref()
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn tax_type(&self) -> &TaxType {
        &self.tax_type
    }

    pub fn tax_amount(&self) -> &Money {
        &self.tax_amount
    }

    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    pub fn partner_id(&self) -> Option<&PartnerId> {
        self.partner_id.as_ref()
    }

    pub fn external_name(&self) -> Option<&ExternalName> {
        self.external_name.as_ref()
    }

    pub fn tracking_number(&self) -> Option<&TrackingNumber> {
        self.tracking_number.as_ref()
    }

    pub fn is_debit(&self) -> bool {
        matches!(self.side, DebitCredit::Debit)
    }

    pub fn is_credit(&self) -> bool {
        matches!(self.side, DebitCredit::Credit)
    }
}

impl ValueObject for JournalEntryLine {
    fn validate(&self) -> DomainResult<()> {
        // Money は常に有効な金額を保持するため、個別の検証は不要
        // 金額が正であることを確認
        if !self.amount.is_positive() {
            return Err(crate::error::DomainError::InvalidAmount(
                "Journal entry line amount must be positive".to_string(),
            ));
        }

        if self.amount.currency() != self.tax_amount.currency() {
            return Err(crate::error::DomainError::InvalidAmount(
                "Amount and tax amount must have the same currency".to_string(),
            ));
        }

        // 一過性取引先（ExternalName）と取引先マスタ（PartnerId）の競合を禁止する。
        // - ExternalName は「諸口（Generic）」相当の一過性名として使う想定
        // - partner_id があるのに external_name
        //   がある状態は、どちらが税務上の名宛人か曖昧になるため禁止
        if self.external_name.is_some() && self.partner_id.is_some() {
            return Err(crate::error::DomainError::ValidationError(
                "external_name cannot be set when partner_id is provided".to_string(),
            ));
        }

        // 残高（BalanceTracking）の追跡には、少なくとも「誰宛てか」を外部名称で保持する必要がある。
        // partner_id がない（＝一過性）場合は、tracking_number と external_name
        // をセットで必須化する。
        if self.partner_id.is_none()
            && self.tracking_number.is_some()
            && self.external_name.is_none()
        {
            return Err(crate::error::DomainError::ValidationError(
                "external_name is required when tracking_number is set for a temporary partner"
                    .to_string(),
            ));
        }

        // tracking_number 自体は BS/PL の判定（科目区分）をここでは行わないため、
        // “あってはいけない組み合わせ”だけを禁止する（必要なら Application/DomainService
        // で拡張する）。
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Currency, Money};

    fn base_new(
        partner_id: Option<PartnerId>,
        external_name: Option<ExternalName>,
        tracking_number: Option<TrackingNumber>,
    ) -> DomainResult<JournalEntryLine> {
        let line_number = LineNumber::new(1)?;
        let side = DebitCredit::Debit;
        let account_code = AccountCode::new("1000")?;
        let amount = Money::from_i64(100, Currency::JPY)?;
        let tax_type = TaxType::NonTaxable;
        let tax_amount = Money::from_i64(0, Currency::JPY)?;

        JournalEntryLine::new(
            line_number,
            side,
            account_code,
            None, // sub_account_code
            None, // department_code
            amount,
            tax_type,
            tax_amount,
            None, // description
            partner_id,
            external_name,
            tracking_number,
        )
    }

    #[test]
    fn external_name_and_partner_id_conflict() {
        let partner_id = Some(PartnerId::new("partner-1".to_string()));
        let external_name = Some(
            ExternalName::new("セブンイレブン".to_string()).expect("ExternalName should be valid"),
        );

        let result = base_new(partner_id, external_name, None);
        assert!(result.is_err());
    }

    #[test]
    fn tracking_number_without_external_name_is_rejected_for_temporary_partner() {
        let tracking_number = Some(
            TrackingNumber::new("20260326-001".to_string())
                .expect("TrackingNumber should be valid"),
        );

        let result = base_new(None, None, tracking_number);
        assert!(result.is_err());
    }

    #[test]
    fn tracking_number_with_external_name_and_no_partner_id_is_allowed() {
        let external_name = Some(
            ExternalName::new("〇〇デザイン事務所".to_string())
                .expect("ExternalName should be valid"),
        );
        let tracking_number = Some(
            TrackingNumber::new("20260326-001".to_string())
                .expect("TrackingNumber should be valid"),
        );

        let result = base_new(None, external_name, tracking_number);
        assert!(result.is_ok());
    }

    #[test]
    fn tracking_number_with_partner_id_and_without_external_name_is_allowed() {
        let partner_id = Some(PartnerId::new("partner-1".to_string()));
        let tracking_number = Some(
            TrackingNumber::new("20260326-001".to_string())
                .expect("TrackingNumber should be valid"),
        );

        let result = base_new(partner_id, None, tracking_number);
        assert!(result.is_ok());
    }
}
