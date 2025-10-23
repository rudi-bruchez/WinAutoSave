use parking_lot::Mutex;
use std::sync::Arc;
use tray_item::{IconSource, TrayItem};

pub struct TrayManager {
    paused: Arc<Mutex<bool>>,
}

impl TrayManager {
    pub fn new(paused: Arc<Mutex<bool>>) -> Self {
        TrayManager { paused }
    }

    pub fn create_tray(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tray = TrayItem::new("WinAutoSave", IconSource::Resource("app-icon"))?;

        let paused_clone = Arc::clone(&self.paused);
        tray.add_menu_item("Pause/Resume", move || {
            let mut paused_lock = paused_clone.lock();
            *paused_lock = !*paused_lock;
            let is_paused = *paused_lock;
            drop(paused_lock);

            println!(
                "AutoSave {}",
                if is_paused { "PAUSED" } else { "RESUMED" }
            );
        })?;

        tray.add_menu_item("Exit", || {
            std::process::exit(0);
        })?;

        // Keep tray alive by leaking it (runs for lifetime of app)
        std::mem::forget(tray);

        Ok(())
    }
}
