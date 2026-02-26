// Templates module - 汎用画面テンプレート
// 責務: 共通画面パターンの再利用可能なテンプレート

pub mod batch_execution_template;
pub mod batch_history_template;
pub mod master_list_template;
pub mod settings_template;

pub use batch_execution_template::{
    BatchExecutionItem, BatchExecutionTemplate, LoadingState as BatchExecutionLoadingState,
    ProcessStep, ProcessStepStatus,
};
pub use batch_history_template::{
    BatchHistoryItem, BatchHistoryTemplate, LoadingState as BatchHistoryLoadingState,
};
pub use master_list_template::{
    LoadingState as MasterListLoadingState, MasterListItem, MasterListTemplate,
};
pub use settings_template::{LoadingState as SettingsLoadingState, SettingsItem, SettingsTemplate};
