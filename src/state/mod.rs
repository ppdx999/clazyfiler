pub mod app_state;
pub mod view_states;

pub use app_state::AppState;
pub use view_states::{
    SearchInputViewState, FileListViewState, FileDetailViewState,
    NavigationData, FuzzyFindData
};

// Re-export FileEntry for backward compatibility
pub use app_state::FileEntry;