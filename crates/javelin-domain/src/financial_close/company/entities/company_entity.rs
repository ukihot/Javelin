// 法人エンティティ

use crate::{
    entity::Entity,
    error::DomainResult,
    financial_close::company::values::{
        ClosingCycle, CompanyId, CompanyName, CompanyNameKana, FiscalYearEnd, RepresentativeName,
        RepresentativeNameKana, RepresentativeTitle,
    },
};

/// 法人エンティティ
///
/// 会計システムの基盤となる法人の定性情報を管理する。
/// 法人名、代表者情報、決算日、締め周期などの基本情報を保持する。
#[derive(Debug, Clone)]
pub struct Company {
    /// 法人ID
    id: CompanyId,
    /// 法人名
    name: CompanyName,
    /// 法人名カナ
    name_kana: CompanyNameKana,
    /// 代表者名
    representative_name: RepresentativeName,
    /// 代表者名カナ
    representative_name_kana: RepresentativeNameKana,
    /// 代表者役職
    representative_title: RepresentativeTitle,
    /// 決算日（月日）
    fiscal_year_end: FiscalYearEnd,
    /// 締め周期
    closing_cycle: ClosingCycle,
}

impl Entity for Company {
    type Id = CompanyId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Company {
    /// 新しい法人を作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: CompanyId,
        name: CompanyName,
        name_kana: CompanyNameKana,
        representative_name: RepresentativeName,
        representative_name_kana: RepresentativeNameKana,
        representative_title: RepresentativeTitle,
        fiscal_year_end: FiscalYearEnd,
        closing_cycle: ClosingCycle,
    ) -> DomainResult<Self> {
        Ok(Self {
            id,
            name,
            name_kana,
            representative_name,
            representative_name_kana,
            representative_title,
            fiscal_year_end,
            closing_cycle,
        })
    }

    /// 法人名を更新
    pub fn update_name(&mut self, name: CompanyName, name_kana: CompanyNameKana) {
        self.name = name;
        self.name_kana = name_kana;
    }

    /// 代表者情報を更新
    pub fn update_representative(
        &mut self,
        name: RepresentativeName,
        name_kana: RepresentativeNameKana,
        title: RepresentativeTitle,
    ) {
        self.representative_name = name;
        self.representative_name_kana = name_kana;
        self.representative_title = title;
    }

    /// 決算日を更新
    pub fn update_fiscal_year_end(&mut self, fiscal_year_end: FiscalYearEnd) {
        self.fiscal_year_end = fiscal_year_end;
    }

    /// 締め周期を更新
    pub fn update_closing_cycle(&mut self, closing_cycle: ClosingCycle) {
        self.closing_cycle = closing_cycle;
    }

    // Getters
    pub fn name(&self) -> &CompanyName {
        &self.name
    }

    pub fn name_kana(&self) -> &CompanyNameKana {
        &self.name_kana
    }

    pub fn representative_name(&self) -> &RepresentativeName {
        &self.representative_name
    }

    pub fn representative_name_kana(&self) -> &RepresentativeNameKana {
        &self.representative_name_kana
    }

    pub fn representative_title(&self) -> &RepresentativeTitle {
        &self.representative_title
    }

    pub fn fiscal_year_end(&self) -> FiscalYearEnd {
        self.fiscal_year_end
    }

    pub fn closing_cycle(&self) -> ClosingCycle {
        self.closing_cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_company() -> Company {
        let id = CompanyId::new("company-001".to_string());
        let name = CompanyName::new("株式会社テスト".to_string()).unwrap();
        let name_kana = CompanyNameKana::new("カブシキガイシャテスト".to_string()).unwrap();
        let rep_name = RepresentativeName::new("山田太郎".to_string()).unwrap();
        let rep_name_kana = RepresentativeNameKana::new("ヤマダタロウ".to_string()).unwrap();
        let rep_title = RepresentativeTitle::new("代表取締役社長".to_string()).unwrap();
        let fiscal_year_end = FiscalYearEnd::new(3, 31).unwrap();
        let closing_cycle = ClosingCycle::Monthly;

        Company::new(
            id,
            name,
            name_kana,
            rep_name,
            rep_name_kana,
            rep_title,
            fiscal_year_end,
            closing_cycle,
        )
        .unwrap()
    }

    #[test]
    fn test_new_company() {
        let company = create_test_company();

        assert_eq!(company.name().value(), "株式会社テスト");
        assert_eq!(company.name_kana().value(), "カブシキガイシャテスト");
        assert_eq!(company.representative_name().value(), "山田太郎");
        assert_eq!(company.representative_name_kana().value(), "ヤマダタロウ");
        assert_eq!(company.representative_title().value(), "代表取締役社長");
        assert_eq!(company.fiscal_year_end().month(), 3);
        assert_eq!(company.fiscal_year_end().day(), 31);
        assert_eq!(company.closing_cycle(), ClosingCycle::Monthly);
    }

    #[test]
    fn test_update_name() {
        let mut company = create_test_company();

        let new_name = CompanyName::new("株式会社新テスト".to_string()).unwrap();
        let new_name_kana = CompanyNameKana::new("カブシキガイシャシンテスト".to_string()).unwrap();

        company.update_name(new_name, new_name_kana);

        assert_eq!(company.name().value(), "株式会社新テスト");
        assert_eq!(company.name_kana().value(), "カブシキガイシャシンテスト");
    }

    #[test]
    fn test_update_representative() {
        let mut company = create_test_company();

        let new_name = RepresentativeName::new("佐藤花子".to_string()).unwrap();
        let new_name_kana = RepresentativeNameKana::new("サトウハナコ".to_string()).unwrap();
        let new_title = RepresentativeTitle::new("代表取締役会長".to_string()).unwrap();

        company.update_representative(new_name, new_name_kana, new_title);

        assert_eq!(company.representative_name().value(), "佐藤花子");
        assert_eq!(company.representative_name_kana().value(), "サトウハナコ");
        assert_eq!(company.representative_title().value(), "代表取締役会長");
    }

    #[test]
    fn test_update_fiscal_year_end() {
        let mut company = create_test_company();

        let new_end = FiscalYearEnd::new(12, 31).unwrap();
        company.update_fiscal_year_end(new_end);

        assert_eq!(company.fiscal_year_end().month(), 12);
        assert_eq!(company.fiscal_year_end().day(), 31);
    }

    #[test]
    fn test_update_closing_cycle() {
        let mut company = create_test_company();

        company.update_closing_cycle(ClosingCycle::Quarterly);

        assert_eq!(company.closing_cycle(), ClosingCycle::Quarterly);
    }
}
