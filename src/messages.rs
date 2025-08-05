/// Messages sent from handlers to App for global processing
#[derive(Debug)]
pub enum AppMessage {
    Quit,
    OpenFile,
    SwitchMode(SwitchAction),
    Error(String),
}

/// Actions for switching between different application modes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchAction {
    EnterExploreMode,
    EnterSearchMode,
}