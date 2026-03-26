// 仕訳登録ユースケース - Request DTOs

use javelin_domain::{
    chart_of_accounts::AccountCode,
    common::{Currency, Money},
    journal_entry::{
        entities::JournalEntryLine,
        values::{
            DebitCredit, DepartmentCode, Description, ExternalName, LineNumber, SubAccountCode,
            TaxType, TrackingNumber,
        },
    },
    partner::PartnerId,
};

use crate::error::ApplicationError;

/// 仕訳明細DTO
#[derive(Debug, Clone)]
pub struct JournalEntryLineDto {
    pub line_number: u32,
    pub side: String, // "Debit" or "Credit"
    pub account_code: String,
    pub sub_account_code: Option<String>,
    pub department_code: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub tax_type: String,
    pub tax_amount: f64,
    pub description: Option<String>,
    pub partner_id: Option<String>,
    pub external_name: Option<String>,
    pub tracking_number: Option<String>,
}

impl TryFrom<&JournalEntryLineDto> for JournalEntryLine {
    type Error = ApplicationError;

    fn try_from(dto: &JournalEntryLineDto) -> Result<Self, Self::Error> {
        let line_number = LineNumber::new(dto.line_number).map_err(|e| {
            ApplicationError::ValidationFailed(vec![format!("Invalid line number: {:?}", e)])
        })?;

        let side = dto
            .side
            .parse::<DebitCredit>()
            .map_err(|e| ApplicationError::ValidationFailed(vec![e]))?;

        let account_code = AccountCode::new(dto.account_code.clone()).map_err(|e| {
            ApplicationError::ValidationFailed(vec![format!("Invalid account code: {:?}", e)])
        })?;

        let sub_account_code = dto
            .sub_account_code
            .as_ref()
            .map(|code| SubAccountCode::new(code.clone()))
            .transpose()
            .map_err(|e| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Invalid sub account code: {:?}",
                    e
                )])
            })?;

        let department_code = dto
            .department_code
            .as_ref()
            .map(|code| DepartmentCode::new(code.clone()))
            .transpose()
            .map_err(|e| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Invalid department code: {:?}",
                    e
                )])
            })?;

        let currency = dto.currency.parse::<Currency>().map_err(ApplicationError::DomainError)?;

        let amount = Money::from_str(&dto.amount.to_string(), currency).map_err(|e| {
            ApplicationError::ValidationFailed(vec![format!("Invalid amount: {:?}", e)])
        })?;

        let tax_type = dto
            .tax_type
            .parse::<TaxType>()
            .map_err(|e| ApplicationError::ValidationFailed(vec![e]))?;

        let tax_amount = Money::from_str(&dto.tax_amount.to_string(), currency).map_err(|e| {
            ApplicationError::ValidationFailed(vec![format!("Invalid tax amount: {:?}", e)])
        })?;

        let description = dto
            .description
            .as_ref()
            .map(|desc| Description::new(desc.clone()))
            .transpose()
            .map_err(|e| {
                ApplicationError::ValidationFailed(vec![format!("Invalid description: {:?}", e)])
            })?;

        let partner_id = dto.partner_id.as_ref().map(|id| PartnerId::new(id.clone()));

        let external_name = dto
            .external_name
            .as_ref()
            .map(|name| ExternalName::new(name.clone()))
            .transpose()
            .map_err(|e| {
                ApplicationError::ValidationFailed(vec![format!("Invalid external_name: {:?}", e)])
            })?;

        let tracking_number = dto
            .tracking_number
            .as_ref()
            .map(|n| TrackingNumber::new(n.clone()))
            .transpose()
            .map_err(|e| {
                ApplicationError::ValidationFailed(vec![format!(
                    "Invalid tracking_number: {:?}",
                    e
                )])
            })?;

        JournalEntryLine::new(
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
        )
        .map_err(ApplicationError::DomainError)
    }
}

impl TryFrom<&javelin_domain::journal_entry::domain_events::JournalEntryLineDto>
    for JournalEntryLineDto
{
    type Error = ApplicationError;

    fn try_from(
        domain_dto: &javelin_domain::journal_entry::domain_events::JournalEntryLineDto,
    ) -> Result<Self, Self::Error> {
        Ok(JournalEntryLineDto {
            line_number: domain_dto.line_number,
            side: domain_dto.side.clone(),
            account_code: domain_dto.account_code.clone(),
            sub_account_code: domain_dto.sub_account_code.clone(),
            department_code: domain_dto.department_code.clone(),
            amount: domain_dto.amount,
            currency: domain_dto.currency.clone(),
            tax_type: domain_dto.tax_type.clone(),
            tax_amount: domain_dto.tax_amount,
            description: domain_dto.description.clone(),
            partner_id: domain_dto.partner_id.clone(),
            external_name: domain_dto.external_name.clone(),
            tracking_number: domain_dto.tracking_number.clone(),
        })
    }
}

/// 仕訳登録リクエスト（下書き作成）
#[derive(Debug, Clone)]
pub struct RegisterJournalEntryRequest {
    pub transaction_date: String,
    pub voucher_number: String,
    pub lines: Vec<JournalEntryLineDto>,
    pub user_id: String,
}

/// 承認申請リクエスト
#[derive(Debug, Clone)]
pub struct SubmitForApprovalRequest {
    pub entry_id: String,
    pub user_id: String,
}

/// 承認リクエスト
#[derive(Debug, Clone)]
pub struct ApproveJournalEntryRequest {
    pub entry_id: String,
    pub approver_id: String,
}

/// 差戻しリクエスト
#[derive(Debug, Clone)]
pub struct RejectJournalEntryRequest {
    pub entry_id: String,
    pub reason: String,
    pub rejected_by: String,
}

/// 取消リクエスト
#[derive(Debug, Clone)]
pub struct ReverseJournalEntryRequest {
    pub entry_id: String,
    pub reason: String,
    pub user_id: String,
}

/// 修正リクエスト
#[derive(Debug, Clone)]
pub struct CorrectJournalEntryRequest {
    pub reversed_entry_id: String,
    pub new_lines: Vec<JournalEntryLineDto>,
    pub reason: String,
    pub user_id: String,
}

/// 下書き更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateDraftJournalEntryRequest {
    pub entry_id: String,
    pub transaction_date: Option<String>,
    pub voucher_number: Option<String>,
    pub lines: Option<Vec<JournalEntryLineDto>>,
    pub user_id: String,
}

/// 下書き削除リクエスト
#[derive(Debug, Clone)]
pub struct DeleteDraftJournalEntryRequest {
    pub entry_id: String,
    pub user_id: String,
}

/// 取消仕訳登録リクエスト
#[derive(Debug, Clone)]
pub struct CancelJournalEntryRequest {
    pub reference_entry_id: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub user_id: String,
}

/// 反対仕訳登録リクエスト
#[derive(Debug, Clone)]
pub struct CreateReversalEntryRequest {
    pub reference_entry_id: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub user_id: String,
}

/// 追加仕訳登録リクエスト
#[derive(Debug, Clone)]
pub struct CreateAdditionalEntryRequest {
    pub reference_entry_id: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub lines: Vec<JournalEntryLineDto>,
    pub user_id: String,
}

/// 再分類仕訳登録リクエスト
#[derive(Debug, Clone)]
pub struct CreateReclassificationEntryRequest {
    pub reference_entry_id: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub lines: Vec<JournalEntryLineDto>,
    pub user_id: String,
}

/// 洗替仕訳登録リクエスト
#[derive(Debug, Clone)]
pub struct CreateReplacementEntryRequest {
    pub reference_entry_id: String,
    pub transaction_date: String,
    pub voucher_number: String,
    pub lines: Vec<JournalEntryLineDto>,
    pub user_id: String,
}
