// DomainService - 横断処理
// 対象: 複数Entity横断処理
// 使用条件: Entityへ属させると不自然な処理
// 制約: Entity貧血防止

pub struct DomainService;

impl Default for DomainService {
    fn default() -> Self {
        Self::new()
    }
}

impl DomainService {
    pub fn new() -> Self {
        Self
    }
}
