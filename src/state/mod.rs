pub mod app_state;
pub mod navigation_state;
pub mod search_state;

pub use app_state::AppState;
pub use navigation_state::NavigationState;
pub use search_state::SearchState;

// Re-export FileEntry for backward compatibility
pub use app_state::FileEntry;