// 取引先集約のドメインサービス

/// 取引先ドメインサービス
pub struct PartnerDomainService;

impl PartnerDomainService {
    /// 取引先名で検索（大文字小文字を区別しない）
    pub fn find_by_name_case_insensitive<'a>(
        partners: &'a [super::entities::Partner],
        name: &str,
    ) -> Vec<&'a super::entities::Partner> {
        partners
            .iter()
            .filter(|p| p.name().to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }
}
