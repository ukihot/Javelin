// Navigation module - Core navigation types and utilities
// Provides stack-based navigation with NavAction pattern

pub mod controllers;
pub mod nav_action;
pub mod navigation_stack;
pub mod page_state;
pub mod presenter_registry;
pub mod route;

#[cfg(test)]
mod end_to_end_tests;
#[cfg(test)]
mod memory_leak_tests;
#[cfg(test)]
mod navigation_integration_tests;
#[cfg(test)]
mod navigation_property_tests;

pub use controllers::Controllers;
pub use nav_action::NavAction;
pub use navigation_stack::NavigationStack;
pub use page_state::PageState;
pub use presenter_registry::PresenterRegistry;
pub use route::Route;
