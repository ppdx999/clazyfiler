use std::path::PathBuf;

// Actions (Flux Pattern)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    MoveSelection(isize),
    EnterDirectory,
    Back,
    LoadDirectory(PathBuf),
    Refresh,
    Quit,
    EnterSearchMode,
    ExitSearchMode,
    UpdateSearchQuery(String),
    SearchSelectFirst,
}