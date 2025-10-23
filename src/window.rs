use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub struct WindowMonitor;

impl WindowMonitor {
    pub fn new() -> Self {
        WindowMonitor
    }

    /// Gets the executable name of the foreground (active) window
    pub fn get_active_window_exe(&self) -> Result<String, Box<dyn std::error::Error>> {
        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.0.is_null() {
                return Err("No foreground window found".into());
            }

            let mut process_id: u32 = 0;
            let thread_id = GetWindowThreadProcessId(hwnd, Some(&mut process_id));

            if thread_id == 0 {
                return Err("Failed to get process ID".into());
            }

            let h_process = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                false,
                process_id,
            )?;

            let mut exe_path: Vec<u16> = vec![0; MAX_PATH as usize];

            // Get the process executable path
            let result = windows::Win32::System::ProcessStatus::GetModuleFileNameExW(
                h_process,
                None,
                &mut exe_path,
            );

            windows::Win32::Foundation::CloseHandle(h_process)?;

            if result == 0 {
                return Err("Failed to get executable name".into());
            }

            // Convert to String
            let null_pos = exe_path.iter().position(|&c| c == 0).unwrap_or(exe_path.len());
            let os_string = OsString::from_wide(&exe_path[..null_pos]);
            let path_str = os_string.to_string_lossy().to_string();

            // Extract just the filename
            if let Some(filename) = path_str.split('\\').last() {
                Ok(filename.to_string())
            } else {
                Ok(path_str)
            }
        }
    }

    /// Checks if the active window matches any of the target applications
    pub fn is_target_app(&self, target_apps: &[String]) -> bool {
        if target_apps.is_empty() {
            return true; // If no specific apps configured, target all apps
        }

        match self.get_active_window_exe() {
            Ok(exe_name) => {
                let exe_lower = exe_name.to_lowercase();
                target_apps.iter().any(|app| {
                    app.to_lowercase() == exe_lower
                })
            }
            Err(_) => false,
        }
    }
}
