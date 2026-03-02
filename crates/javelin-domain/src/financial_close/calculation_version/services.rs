// 計算バージョンのドメインサービス

use chrono::{DateTime, Utc};

use super::{entities::CalculationLogicVersion, values::VersionNumber};
use crate::error::DomainResult;

/// 計算バージョンドメインサービス
pub struct CalculationVersionService;

impl CalculationVersionService {
    /// 指定日時に有効なバージョンを取得
    pub fn find_effective_version<'a>(
        versions: &'a [CalculationLogicVersion],
        logic_name: &str,
        at_date: DateTime<Utc>,
    ) -> Option<&'a CalculationLogicVersion> {
        versions
            .iter()
            .filter(|v| v.logic_name() == logic_name && v.is_effective_at(at_date))
            .max_by_key(|v| v.version_number())
    }

    /// 最新バージョンを取得
    pub fn find_latest_version<'a>(
        versions: &'a [CalculationLogicVersion],
        logic_name: &str,
    ) -> Option<&'a CalculationLogicVersion> {
        versions
            .iter()
            .filter(|v| v.logic_name() == logic_name)
            .max_by_key(|v| v.version_number())
    }

    /// バージョン間の差分を検出
    pub fn detect_changes(
        old_version: &CalculationLogicVersion,
        new_version: &CalculationLogicVersion,
    ) -> VersionChanges {
        let mut changes = Vec::new();

        if old_version.logic_hash() != new_version.logic_hash() {
            changes.push("Logic hash changed".to_string());
        }

        if old_version.description() != new_version.description() {
            changes.push("Description changed".to_string());
        }

        VersionChanges { has_changes: !changes.is_empty(), changes }
    }

    /// 次のバージョン番号を提案
    pub fn suggest_next_version(
        current_version: &VersionNumber,
        change_type: ChangeType,
    ) -> VersionNumber {
        match change_type {
            ChangeType::Major => current_version.increment_major(),
            ChangeType::Minor => current_version.increment_minor(),
            ChangeType::Patch => current_version.increment_patch(),
        }
    }

    /// バージョンの整合性を検証
    pub fn verify_version_consistency(
        versions: &[CalculationLogicVersion],
        logic_name: &str,
    ) -> DomainResult<ConsistencyReport> {
        let logic_versions: Vec<_> =
            versions.iter().filter(|v| v.logic_name() == logic_name).collect();

        let mut issues = Vec::new();

        // 有効期間の重複チェック
        for (i, v1) in logic_versions.iter().enumerate() {
            for v2 in logic_versions.iter().skip(i + 1) {
                if Self::has_overlapping_period(v1, v2) {
                    issues.push(format!(
                        "Overlapping effective periods: {} and {}",
                        v1.version_number(),
                        v2.version_number()
                    ));
                }
            }
        }

        // バージョン番号の重複チェック
        let mut version_numbers: Vec<_> =
            logic_versions.iter().map(|v| v.version_number()).collect();
        version_numbers.sort();
        version_numbers.dedup();

        if version_numbers.len() != logic_versions.len() {
            issues.push("Duplicate version numbers found".to_string());
        }

        Ok(ConsistencyReport { is_consistent: issues.is_empty(), issues })
    }

    fn has_overlapping_period(v1: &CalculationLogicVersion, v2: &CalculationLogicVersion) -> bool {
        let v1_start = v1.effective_from();
        let v1_end = v1.effective_to().unwrap_or(DateTime::<Utc>::MAX_UTC);
        let v2_start = v2.effective_from();
        let v2_end = v2.effective_to().unwrap_or(DateTime::<Utc>::MAX_UTC);

        v1_start < v2_end && v2_start < v1_end
    }

    /// 再計算を実行（過去時点のロジックを使用）
    pub fn recalculate_with_version(
        version: &CalculationLogicVersion,
        _input_data: &str,
    ) -> DomainResult<RecalculationResult> {
        // 実際の再計算ロジックはここに実装
        // バージョンのロジックハッシュを使用して適切な計算ロジックを選択

        Ok(RecalculationResult {
            version_used: version.version_number().to_string(),
            logic_hash: version.logic_hash().to_string(),
            result_value: 0, // Placeholder
            calculation_date: Utc::now(),
        })
    }
}

/// 変更タイプ
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    /// メジャー変更（互換性のない変更）
    Major,
    /// マイナー変更（後方互換性のある機能追加）
    Minor,
    /// パッチ（バグ修正）
    Patch,
}

/// バージョン変更
#[derive(Debug, Clone)]
pub struct VersionChanges {
    pub has_changes: bool,
    pub changes: Vec<String>,
}

/// 整合性レポート
#[derive(Debug, Clone)]
pub struct ConsistencyReport {
    pub is_consistent: bool,
    pub issues: Vec<String>,
}

/// 再計算結果
#[derive(Debug, Clone)]
pub struct RecalculationResult {
    pub version_used: String,
    pub logic_hash: String,
    pub result_value: i64,
    pub calculation_date: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::financial_close::calculation_version::values::CalculationVersionId;

    fn create_test_version(
        version_number: VersionNumber,
        logic_name: &str,
    ) -> CalculationLogicVersion {
        let id = CalculationVersionId::new();
        CalculationLogicVersion::new(
            id,
            version_number,
            logic_name.to_string(),
            "Test version".to_string(),
            "hash123".to_string(),
            "USER001".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_find_effective_version() {
        let v1 = create_test_version(VersionNumber::new(1, 0, 0), "ECL");
        let v2 = create_test_version(VersionNumber::new(2, 0, 0), "ECL");

        let versions = vec![v1, v2];
        let result =
            CalculationVersionService::find_effective_version(&versions, "ECL", Utc::now());

        assert!(result.is_some());
        assert_eq!(result.unwrap().version_number(), &VersionNumber::new(2, 0, 0));
    }

    #[test]
    fn test_find_latest_version() {
        let v1 = create_test_version(VersionNumber::new(1, 0, 0), "ECL");
        let v2 = create_test_version(VersionNumber::new(1, 1, 0), "ECL");
        let v3 = create_test_version(VersionNumber::new(2, 0, 0), "ECL");

        let versions = vec![v1, v2, v3];
        let result = CalculationVersionService::find_latest_version(&versions, "ECL");

        assert!(result.is_some());
        assert_eq!(result.unwrap().version_number(), &VersionNumber::new(2, 0, 0));
    }

    #[test]
    fn test_detect_changes() {
        let v1 = create_test_version(VersionNumber::new(1, 0, 0), "ECL");
        let v2 = create_test_version(VersionNumber::new(2, 0, 0), "ECL");

        // No changes initially (same hash)
        let changes = CalculationVersionService::detect_changes(&v1, &v2);
        assert!(!changes.has_changes);
    }

    #[test]
    fn test_suggest_next_version() {
        let current = VersionNumber::new(1, 2, 3);

        let major = CalculationVersionService::suggest_next_version(&current, ChangeType::Major);
        assert_eq!(major, VersionNumber::new(2, 0, 0));

        let minor = CalculationVersionService::suggest_next_version(&current, ChangeType::Minor);
        assert_eq!(minor, VersionNumber::new(1, 3, 0));

        let patch = CalculationVersionService::suggest_next_version(&current, ChangeType::Patch);
        assert_eq!(patch, VersionNumber::new(1, 2, 4));
    }

    #[test]
    fn test_verify_version_consistency() {
        let v1 = create_test_version(VersionNumber::new(1, 0, 0), "ECL");
        let v2 = create_test_version(VersionNumber::new(2, 0, 0), "ECL");

        let versions = vec![v1, v2];
        let report =
            CalculationVersionService::verify_version_consistency(&versions, "ECL").unwrap();

        // Should have overlapping periods issue
        assert!(!report.is_consistent);
    }
}
