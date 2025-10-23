mod config;
mod hotkey;
mod keyboard;
mod tray;
mod window;

use config::Config;
use hotkey::HotkeyHandler;
use keyboard::KeyboardSimulator;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tray::TrayManager;
use window::WindowMonitor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hide console window on release builds
    #[cfg(not(debug_assertions))]
    hide_console_window();

    println!("WinAutoSave starting...");

    // Load configuration
    let config = Config::load()?;
    println!(
        "Loaded config: interval={}s, target_apps={:?}",
        config.interval_seconds, config.target_apps
    );

    // Shared pause state
    let paused = Arc::new(Mutex::new(false));

    // Initialize components
    let keyboard = KeyboardSimulator::new();
    let window_monitor = WindowMonitor::new();

    // Setup hotkey handler
    if config.hotkey_enabled {
        let hotkey_handler = HotkeyHandler::new(Arc::clone(&paused));
        hotkey_handler.register()?;
        hotkey_handler.start_listener();
        println!("Hotkey registered: Ctrl+Shift+P to pause/resume");
    }

    // Setup system tray
    let tray_manager = TrayManager::new(Arc::clone(&paused));
    if let Err(e) = tray_manager.create_tray() {
        eprintln!("Warning: Failed to create system tray: {}", e);
    } else {
        println!("System tray created");
    }

    println!("AutoSave running. Press Ctrl+Shift+P to pause/resume.");

    // Main loop
    let interval = Duration::from_secs(config.interval_seconds);
    loop {
        std::thread::sleep(interval);

        // Check if paused
        if *paused.lock() {
            continue;
        }

        // Check if current window is a target app
        if !window_monitor.is_target_app(&config.target_apps) {
            continue;
        }

        // Send Ctrl+S
        match keyboard.send_ctrl_s() {
            Ok(_) => {
                if let Ok(exe) = window_monitor.get_active_window_exe() {
                    println!("Saved: {}", exe);
                }
            }
            Err(e) => {
                eprintln!("Error sending Ctrl+S: {}", e);
            }
        }
    }
}

#[cfg(not(debug_assertions))]
fn hide_console_window() {
    use windows::Win32::System::Console::GetConsoleWindow;
    use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};

    unsafe {
        let window = GetConsoleWindow();
        if !window.0.is_null() {
            let _ = ShowWindow(window, SW_HIDE);
        }
    }
}
