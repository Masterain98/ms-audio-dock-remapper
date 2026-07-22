//! Login autostart. Windows writes the HKCU Run registry value (no admin
//! needed). Other platforms write the OS-appropriate launcher file; for now
//! they are stubbed with a clear TODO so the architecture is in place.

pub fn is_enabled() -> bool {
    #[cfg(windows)]
    {
        crate::platform::windows::autostart_enabled()
    }
    #[cfg(not(windows))]
    {
        false
    }
}

pub fn set_enabled(enable: bool) {
    #[cfg(windows)]
    {
        crate::platform::windows::set_autostart(enable);
    }
    #[cfg(not(windows))]
    {
        let _ = enable;
        // TODO(linux): write ~/.config/autostart/ms-audio-dock-remapper.desktop
        // TODO(macos): write ~/Library/LaunchAgents/...plist
    }
}
