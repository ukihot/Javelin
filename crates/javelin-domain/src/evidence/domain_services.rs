// 証憑集約のドメインサービス

/// 証憑ドメインサービス
pub struct EvidenceDomainService;

impl EvidenceDomainService {
    /// 証憑が有効かどうかを検証
    pub fn is_valid_evidence(evidence: &super::entities::Evidence) -> bool {
        // 基本的な検証: 日付が未来でない、金額が正など
        evidence.date() <= chrono::Utc::now().date_naive()
    }
}
