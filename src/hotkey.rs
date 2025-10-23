use parking_lot::Mutex;
use std::sync::Arc;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_CONTROL, MOD_SHIFT, VK_P,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG, WM_HOTKEY,
};

const HOTKEY_ID: i32 = 1;

pub struct HotkeyHandler {
    paused: Arc<Mutex<bool>>,
}

impl HotkeyHandler {
    pub fn new(paused: Arc<Mutex<bool>>) -> Self {
        HotkeyHandler { paused }
    }

    /// Registers Ctrl+Shift+P as the pause/resume hotkey
    pub fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            RegisterHotKey(
                None,
                HOTKEY_ID,
                MOD_CONTROL | MOD_SHIFT,
                VK_P.0 as u32,
            )?;
        }
        Ok(())
    }

    /// Unregisters the hotkey
    pub fn unregister(&self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            UnregisterHotKey(None, HOTKEY_ID)?;
        }
        Ok(())
    }

    /// Starts listening for hotkey events in a separate thread
    pub fn start_listener(&self) {
        let paused = Arc::clone(&self.paused);

        std::thread::spawn(move || {
            unsafe {
                let mut msg = MSG::default();

                while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    if msg.message == WM_HOTKEY && msg.wParam.0 == HOTKEY_ID as usize {
                        // Toggle pause state
                        let mut paused_lock = paused.lock();
                        *paused_lock = !*paused_lock;
                        let is_paused = *paused_lock;
                        drop(paused_lock);

                        println!(
                            "AutoSave {}",
                            if is_paused { "PAUSED" } else { "RESUMED" }
                        );
                    }

                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                }
            }
        });
    }
}

impl Drop for HotkeyHandler {
    fn drop(&mut self) {
        let _ = self.unregister();
    }
}
