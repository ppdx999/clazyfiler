/// Unified action type that encompasses all action categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Select,
    Back,
    Refresh,
    SwitchMode(ModeSwitchAction),
    SearchInput(char),
    SearchClear,
    SearchPop,
    SearchDeleteWord,    // Ctrl+W - delete word backward
    SearchDeleteToEnd,   // Ctrl+K - delete to end of line
    SearchHome,          // Ctrl+A - move to beginning
    SearchEnd,           // Ctrl+E - move to end
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModeSwitchAction {
    EnterExploreMode,
    EnterSearchMode,
}

