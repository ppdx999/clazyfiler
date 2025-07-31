# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Clazyfiler is a fully customizable TUI (Terminal User Interface) file manager written in Rust. The project is currently in development and uses a clean modular architecture built around the Strategy pattern for mode management.

## Build and Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Build for release
cargo build --release

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

## Architecture

### Core Components

- **main.rs**: Application entry point that initializes terminal and runs the main app loop
- **app.rs**: Main App struct that coordinates between modes, state, and rendering. Contains the primary event loop logic
- **state.rs**: AppState struct for maintaining application state (currently minimal but designed for expansion)
- **actions.rs**: Centralized Action enum for all application actions (currently only contains Quit)

### Mode System (Strategy Pattern)

The application uses a clean Strategy pattern implementation for different modes:

- **modes/interface.rs**: ModeBehavior trait defining the interface for all modes (handle_key, dispatch, render)
- **modes/mod.rs**: Mode enum that delegates to concrete mode implementations
- **modes/explore.rs**: ExploreMode for file browsing (skeleton implementation)
- **modes/search.rs**: SearchMode for searching functionality (skeleton implementation)

Each mode implements the ModeBehavior trait and handles its own:
- Key input processing
- Action dispatching
- UI rendering

### Configuration System

The application supports TOML-based configuration:
- Configuration files should be placed at `~/.config/clazyfiler/config.toml`
- See `examples/config.toml` and `examples/config-alternative.toml` for reference configurations
- Supports customizable keymaps, UI settings, external commands, and general preferences

### Dependencies

- **ratatui 0.29**: TUI framework for rendering
- **crossterm 0.28**: Cross-platform terminal manipulation
- **serde 1.0**: Serialization/deserialization (with derive feature)
- **toml 0.8**: TOML configuration parsing
- **dirs 5.0**: Platform-specific directory handling

## Key Architecture Patterns

1. **Strategy Pattern**: Modes are implemented as separate strategies that can be swapped at runtime
2. **Command Pattern**: Actions represent commands that can be dispatched and handled by modes
3. **Separation of Concerns**: Clear separation between input handling, state management, and rendering
4. **Configuration-Driven**: Extensive customization through TOML configuration files

## Development Notes

- The project is in early development with skeleton implementations for most modes
- The architecture is designed for extensibility - new modes can be easily added by implementing ModeBehavior
- Configuration system is designed to be highly flexible with HashMap-based key mappings
- Terminal handling is abstracted through the terminal.rs module