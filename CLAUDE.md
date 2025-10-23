# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

WinAutoSave is a Windows utility application that automatically sends Ctrl+S keypress events at configurable intervals to save work in active applications.

Features : 

- Auto save only in selected apps.
- Runs without prints and terminal.
- system tray icon
- configuration file
- hotkey to pause/resume

## Technology Stack

- **Language**: Rust
- **Platform**: Windows-specific (uses Windows API for keyboard simulation)
- **Build System**: Cargo

## Development Commands

### Build
```bash
cargo build              # Debug build
cargo build --release    # Release build with optimizations
```

### Run
```bash
cargo run                # Run debug version
cargo run --release      # Run release version
```

### Testing
```bash
cargo test               # Run all tests
cargo test <test_name>   # Run specific test
cargo test -- --nocapture # Run tests with stdout visible
```

### Linting and Formatting
```bash
cargo fmt                # Format code
cargo clippy             # Run linter
cargo check              # Fast compile check without generating executable
```

## Architecture Considerations

### Windows API Integration
This application requires Windows-specific APIs for:
- **Keyboard Input Simulation**: Use `windows` crate or `winapi` crate to send keyboard events (VK_CONTROL + VK_S)
- **Window Management**: May need to detect active window or run in background
- **System Tray**: Consider implementing system tray icon for configuration and control

### Core Components (Expected)
- **Timer/Interval Manager**: Controls the frequency of Ctrl+S keypresses
- **Keyboard Event Generator**: Interfaces with Windows API to simulate keypresses
- **Configuration**: User-configurable interval settings (possibly via config file or system tray)
- **Background Service**: Runs continuously while minimized or in system tray

### Key Implementation Details
- Must handle proper cleanup on exit to avoid stuck key states
- Should respect application focus (may need to pause when certain apps are active)
- Consider adding hotkey to enable/disable functionality
- May need elevated permissions depending on target applications

## Dependencies to Consider
- `windows` or `winapi`: Windows API bindings
- `serde`: For configuration serialization
- `tokio` or `async-std`: If using async runtime for timers
- System tray crates: `tray-item` or `ksni` for GUI integration
