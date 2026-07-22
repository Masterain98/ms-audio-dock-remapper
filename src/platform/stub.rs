use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::platform::MonitorEvent;

/// Placeholder monitor for non-Windows platforms. The architecture isolates
/// all OS input code here; a real Linux (evdev/hidraw) or macOS (IOKit)
/// backend would replace this, while `main`/`ui` stay unchanged.
pub fn start_monitor(
    _on_event: impl Fn(MonitorEvent) + Send + 'static,
    _config: Arc<Mutex<Config>>,
) {
    eprintln!(
        "[ms-audio-dock-remapper] Raw-input monitoring is only implemented on Windows so far. \
         Build/run on Windows to use the Dock Teams-key remapper."
    );
}
