// 仕訳照会リクエストDTO

/// 仕訳一覧照会クエリ
#[derive(Debug, Clone)]
pub struct ListJournalEntriesQuery {
    pub status: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 仕訳詳細照会クエリ
#[derive(Debug, Clone)]
pub struct GetJournalEntryQuery {
    pub entry_id: String,
}
