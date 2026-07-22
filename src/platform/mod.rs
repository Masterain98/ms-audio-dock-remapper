use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::config::Config;

// App icon, embedded at compile time and materialized to a temp file at runtime
// so both the Slint window (ICO) and the in-app header (PNG) can load it.
const APP_ICON_ICO: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/app-icon.ico"));
// Small, pre-anti-aliased (supersampled) icon for the in-app header / menu.
// Using this instead of the 512px PNG avoids runtime shrink artifacts (jaggies).
const APP_ICON_HEADER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/public/app-icon-header.png"
));

/// Writes `bytes` to a stable temp file (once) and returns its path. Returns
/// `None` only if writing fails, in which case callers fall back to defaults.
fn ensure_icon_file(bytes: &[u8], name: &str) -> Option<PathBuf> {
    let path = std::env::temp_dir().join(name);
    let needs_write = match std::fs::read(&path) {
        Ok(existing) => existing.len() != bytes.len(),
        Err(_) => true,
    };
    if needs_write && std::fs::write(&path, bytes).is_err() {
        return None;
    }
    Some(path)
}

/// Path to the small, pre-anti-aliased PNG used for the in-app header icon.
/// Keeping it small (64px, supersampled) avoids jagged runtime downscaling from
/// the full 512px asset. `None` where no custom icon is shipped.
#[cfg(windows)]
pub fn header_icon_path() -> Option<PathBuf> {
    ensure_icon_file(APP_ICON_HEADER, "ms-audio-dock-remapper-icon-header.png")
}
#[cfg(not(windows))]
pub fn header_icon_path() -> Option<PathBuf> {
    None
}

/// Path to the multi-size ICO for the Windows tray icon.
/// `None` on platforms where no custom icon is shipped.
#[cfg(windows)]
pub fn tray_icon_path() -> Option<PathBuf> {
    ensure_icon_file(APP_ICON_ICO, "ms-audio-dock-remapper-icon.ico")
}
#[cfg(not(windows))]
pub fn tray_icon_path() -> Option<PathBuf> {
    None
}

#[cfg(not(windows))]
pub mod stub;
#[cfg(windows)]
pub mod windows;

/// Events delivered from the OS-specific resident monitor thread to the UI
/// thread. Keeping this enum platform-agnostic lets `main`/`ui` stay identical
/// across Windows / Linux / macOS.
#[derive(Debug, Clone)]
pub enum MonitorEvent {
    /// A Teams-key press was detected (stats update).
    Press,
    /// Candidate Dock collections currently registered.
    Status(u32),
    /// Tray icon requested the settings window.
    TrayShow,
}

/// Starts the OS-specific resident monitor (raw-input listening + tray).
/// Implementations live in `windows.rs` / `stub.rs`, selected by cfg.
pub fn start_monitor(tx: Sender<MonitorEvent>, config: Arc<Mutex<Config>>) {
    #[cfg(windows)]
    crate::platform::windows::start_monitor(tx, config);
    #[cfg(not(windows))]
    crate::platform::stub::start_monitor(tx, config);
}

/// Shows a modal alert (Windows MessageBox; otherwise stderr). Used for fatal
/// startup errors and action failures so they are never silent.
pub fn alert(message: &str) {
    #[cfg(windows)]
    crate::platform::windows::alert(message);
    #[cfg(not(windows))]
    {
        eprintln!("[ms-audio-dock-remapper] {message}");
    }
}

/// Asks the resident monitor thread to exit. On Windows this posts `WM_QUIT` to
/// the monitor thread; on other platforms it is a no-op (the process exits via
/// the Slint event loop returning).
pub fn request_quit() {
    #[cfg(windows)]
    crate::platform::windows::request_quit();
    #[cfg(not(windows))]
    {
        // No resident monitor thread to stop; the UI loop exit ends the process.
    }
}

/// Declares the process DPI-aware so windows render crisply on high-DPI
/// displays (notably the "already running" MessageBox shown before the UI loop).
pub fn set_dpi_aware() {
    #[cfg(windows)]
    crate::platform::windows::set_dpi_aware();
    #[cfg(not(windows))]
    {}
}
