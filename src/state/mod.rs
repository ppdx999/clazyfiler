pub mod app_state;
pub mod navigation_state;
pub mod search_state;
pub mod fuzzy_find_state;

pub use app_state::AppState;
pub use navigation_state::NavigationState;
pub use search_state::SearchState;
pub use fuzzy_find_state::FuzzyFindState;

// Re-export FileEntry for backward compatibility
pub use app_state::FileEntry;