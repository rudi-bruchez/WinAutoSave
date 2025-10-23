# WinAutoSave

A Windows utility application that automatically sends Ctrl+S keypress events at configurable intervals to save work in active applications.

## Features

- **Auto-save for selected apps**: Configure which applications should receive auto-save commands
- **Runs in background**: No console window in release mode
- **System tray integration**: Easy access to pause/resume and exit functions
- **Global hotkey**: Press `Ctrl+Shift+P` to pause/resume auto-save
- **JSON configuration**: Simple config file for customization
- **Lightweight**: Built with Rust for minimal resource usage

## Installation

### Build from Source

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Clone this repository
3. Build the release version:

```bash
cargo build --release
```

4. The executable will be in `target/release/winautosave.exe`

## Configuration

On first run, a `config.json` file will be created in the same directory as the executable with default settings:

```json
{
  "interval_seconds": 60,
  "target_apps": [
    "notepad.exe",
    "Code.exe",
    "WINWORD.EXE",
    "EXCEL.EXE"
  ],
  "hotkey_enabled": true
}
```

### Configuration Options

- **interval_seconds**: Time in seconds between auto-save keypresses (default: 60)
- **target_apps**: List of executable names to auto-save. Leave empty `[]` to auto-save all applications
- **hotkey_enabled**: Enable/disable the global hotkey (Ctrl+Shift+P)

## Usage

1. Run `winautosave.exe`
2. The application will start in the background
3. A system tray icon will appear (if supported)
4. Auto-save will trigger every configured interval for target applications

### Controls

- **Pause/Resume**: Press `Ctrl+Shift+P` or use the system tray menu
- **Exit**: Right-click the system tray icon and select "Exit"

### Debug Mode

To run in debug mode with console output:

```bash
cargo run
```

This will show log messages including which applications are being saved.

## How It Works

1. The application runs continuously in the background
2. Every configured interval, it checks if the active window matches a target application
3. If it's a target app and not paused, it sends a Ctrl+S keypress
4. The hotkey listener runs in a separate thread to respond to pause/resume commands

## Architecture

- **config.rs**: Configuration file handling with JSON serialization
- **keyboard.rs**: Windows API keyboard input simulation
- **window.rs**: Active window detection and process identification
- **hotkey.rs**: Global hotkey registration and event handling
- **tray.rs**: System tray icon and menu
- **main.rs**: Main application loop and component initialization

## Requirements

- Windows operating system
- Rust 2021 edition or later (for building)

## Development

### Build Debug Version

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Run Linter

```bash
cargo clippy
```

## License

This project is open source and available for personal and commercial use.

## Troubleshooting

**System tray icon not appearing**: Some systems may not support the tray-item library. The application will still work via the hotkey.

**Auto-save not working**:
- Check that the application name in config.json matches exactly (case-sensitive)
- Verify the application is not paused (press Ctrl+Shift+P)
- Some applications may require elevated permissions

**Hotkey not working**:
- Ensure another application isn't using Ctrl+Shift+P
- Check that `hotkey_enabled` is `true` in config.json
