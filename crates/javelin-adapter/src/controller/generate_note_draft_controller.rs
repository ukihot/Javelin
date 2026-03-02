// GenerateNoteDraftController - 注記草案生成コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{GenerateNoteDraftRequest, GenerateNoteDraftResponse},
    input_ports::GenerateNoteDraftUseCase,
};

use crate::error::AdapterResult;

/// 注記草案生成コントローラ
pub struct GenerateNoteDraftController<U>
where
    U: GenerateNoteDraftUseCase,
{
    use_case: Arc<U>,
}

impl<U> GenerateNoteDraftController<U>
where
    U: GenerateNoteDraftUseCase,
{
    pub fn new(use_case: Arc<U>) -> Self {
        Self { use_case }
    }

    /// 注記草案生成処理
    pub async fn generate_note_draft(
        &self,
        request: GenerateNoteDraftRequest,
    ) -> AdapterResult<GenerateNoteDraftResponse> {
        self.use_case
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
