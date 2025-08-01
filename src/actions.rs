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
    SearchEnter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModeSwitchAction {
    EnterExploreMode,
    EnterSearchMode,
}

