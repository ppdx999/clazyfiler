# Flux Architecture Migration Guide

## Current Implementation

We've implemented a gradual migration to Flux architecture with these components:

### 1. Actions (Completed)
```rust
pub enum Action {
    // Navigation actions
    MoveSelection(isize),
    EnterDirectory,
    LoadDirectory(PathBuf),
    
    // UI actions
    Refresh,
    Quit,
}
```

### 2. Dispatcher (Completed)
```rust
fn dispatch(&mut self, action: Action) -> Result<(), String> {
    // Single point for all state changes
    match action { ... }
}
```

### 3. Key Mapping (Completed)
```rust
fn key_to_action(key: KeyCode, config: &Config) -> Option<Action> {
    // Translates keyboard input to actions
}
```

## Next Steps for Full Flux Migration

### 4. Action History (Future)
```rust
struct AppStore {
    state: App,
    history: Vec<Action>,
    history_index: usize,
}

impl AppStore {
    fn dispatch(&mut self, action: Action) -> Result<(), String> {
        let result = self.state.dispatch(action.clone());
        if result.is_ok() {
            // Add to history for undo/redo
            self.history.truncate(self.history_index);
            self.history.push(action);
            self.history_index += 1;
        }
        result
    }
    
    fn undo(&mut self) -> Result<(), String> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.replay_from_beginning()
        } else {
            Err("Nothing to undo".to_string())
        }
    }
}
```

### 5. Future Actions Examples
```rust
pub enum Action {
    // Current actions...
    
    // Search functionality
    StartSearch,
    UpdateSearchQuery(String),
    ExecuteSearch,
    ClearSearch,
    
    // File operations
    CreateFile(String),
    CreateDirectory(String),
    DeleteFile(PathBuf),
    RenameFile(PathBuf, String),
    CopyFile(PathBuf, PathBuf),
    
    // Bookmarks
    AddBookmark(PathBuf),
    RemoveBookmark(PathBuf),
    GoToBookmark(usize),
    
    // External commands
    OpenWithEditor(PathBuf),
    OpenWithFuzzyFinder,
    RunExternalCommand(String, Vec<PathBuf>),
    
    // UI state
    ToggleHiddenFiles,
    ChangePanelRatio(u8),
    ToggleBorders,
    ShowHelp,
    HideHelp,
    
    // Multiple panels/tabs
    CreateTab,
    CloseTab(usize),
    SwitchTab(usize),
    SplitPanel,
}
```

### 6. State Management (Future)
```rust
#[derive(Debug, Clone)]
struct AppState {
    // Current state
    current_dir: PathBuf,
    files: Vec<FileEntry>,
    selected_index: usize,
    
    // UI state
    search_query: Option<String>,
    search_results: Vec<FileEntry>,
    show_hidden: bool,
    help_visible: bool,
    
    // Multi-panel state
    tabs: Vec<TabState>,
    active_tab: usize,
    panels: Vec<PanelState>,
    
    // External state
    bookmarks: Vec<PathBuf>,
}
```

## Benefits of This Approach

### Immediate Benefits (Current Implementation)
- ✅ **Centralized state changes**: All mutations go through `dispatch()`
- ✅ **Testable actions**: Can test actions independently
- ✅ **Clear separation**: UI → Actions → State changes
- ✅ **Extensible**: Easy to add new actions without changing core logic

### Future Benefits (With Full Migration)
- 📋 **Undo/Redo**: Action history enables time travel
- 📋 **Debugging**: Can log/replay all actions
- 📋 **Testing**: Can test complex sequences of actions
- 📋 **Multiple UIs**: Could add web interface using same actions
- 📋 **Plugins**: External code can dispatch actions

## Migration Strategy

### Phase 1 ✅ (Complete)
- Define basic Action enum
- Add dispatch method
- Refactor key handling

### Phase 2 📋 (When needed)
- Add action history for undo/redo
- Extract AppStore from App
- Add middleware for logging/debugging

### Phase 3 📋 (Advanced features)
- Add complex actions (search, file ops)
- Implement async action handling
- Add plugin system via actions

## Example: Adding Search Feature

With current Flux foundation, adding search would be:

```rust
// 1. Add action
enum Action {
    // existing...
    StartSearch,
    UpdateSearchQuery(String),
    ExecuteSearch,
}

// 2. Add to dispatcher
fn dispatch(&mut self, action: Action) -> Result<(), String> {
    match action {
        // existing...
        Action::StartSearch => {
            self.search_mode = true;
            self.search_query = String::new();
            Ok(())
        }
        Action::UpdateSearchQuery(query) => {
            self.search_query = query;
            Ok(())
        }
        Action::ExecuteSearch => {
            // Perform search, update filtered files
            self.filter_files_by_search();
            Ok(())
        }
    }
}

// 3. Add key mapping
fn key_to_action(key: KeyCode, config: &Config) -> Option<Action> {
    match key {
        // existing...
        KeyCode::Char('/') => Some(Action::StartSearch),
        KeyCode::Esc if in_search_mode => Some(Action::ClearSearch),
        // ...
    }
}
```

## Conclusion

The current implementation provides a solid foundation for Flux architecture while maintaining simplicity. As features grow, the migration path is clear and incremental.