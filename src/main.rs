#![windows_subsystem = "windows"]

mod actions;
mod autostart;
mod config;
mod i18n;
mod installed_apps;
mod platform;
mod presets;
mod ui;

use std::sync::{Arc, Mutex};

fn main() {
    // Declare DPI awareness first, so every window (incl. the "already running"
    // MessageBox shown below) is crisp on high-DPI displays.
    platform::set_dpi_aware();

    let config = Arc::new(Mutex::new(config::Config::load()));
    let lang = i18n::Lang::resolve(&config.lock().unwrap().language);

    // Single running instance (named mutex on Windows).
    #[cfg(windows)]
    if !platform::windows::ensure_single_instance() {
        platform::alert(i18n::t(lang, "single_instance"));
        return;
    }

    // Slint UI event loop (blocks until quit). The UI wires up the OS-specific
    // resident monitor (raw input + tray) itself, forwarding events into the
    // Slint event loop so the app stays event-driven (no busy polling).
    ui::run(config, lang);
}
