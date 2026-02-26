// Adapter Layer - 外部入出力変換
// 依存方向: → Application

pub mod controller;
pub mod error;
pub mod input_mode;
pub mod navigation;
pub mod page_states;
pub mod presenter;
pub mod views;

// Re-export for convenience
pub use input_mode::{InputMode, JjEscapeDetector, ModifyInputType};
pub use navigation::{
    Controllers, NavAction, NavigationStack, PageState, PresenterRegistry, Route,
};
pub use page_states::*;
pub use views::*;
