/// Version-1 actions retained only for backward-compatible config execution.
#[derive(Debug, Clone)]
pub struct Preset {
    pub id: &'static str,
    /// "launch" | "uri" | "beep"
    pub kind: &'static str,
    /// Fallback command/URI when no installed executable is found.
    pub command: &'static str,
    /// Candidate install paths (env vars expanded); first that exists wins.
    pub exe_hints: &'static [&'static str],
}

pub const ALL: &[Preset] = &[
    Preset {
        id: "teams",
        kind: "launch",
        command: "msteams:",
        exe_hints: &[
            "%LOCALAPPDATA%\\Microsoft\\Teams\\current\\Teams.exe",
            "%LOCALAPPDATA%\\Microsoft\\Teams\\Teams.exe",
            "C:\\Program Files\\Microsoft\\Teams\\current\\Teams.exe",
        ],
    },
    Preset {
        id: "zoom",
        kind: "launch",
        command: "zoommtg:",
        exe_hints: &[
            "%APPDATA%\\Zoom\\bin\\Zoom.exe",
            "%LOCALAPPDATA%\\Zoom\\bin\\Zoom.exe",
            "C:\\Program Files\\Zoom\\bin\\Zoom.exe",
            "C:\\Program Files (x86)\\Zoom\\bin\\Zoom.exe",
        ],
    },
    Preset {
        id: "webex",
        kind: "launch",
        command: "webex:",
        exe_hints: &[
            "%LOCALAPPDATA%\\CiscoSpark\\CiscoCollabHost.exe",
            "C:\\Program Files (x86)\\Cisco Spark\\CiscoCollabHost.exe",
            "C:\\Program Files\\Cisco WebEx\\CiscoCollabHost.exe",
        ],
    },
    Preset {
        id: "googlemeet",
        kind: "uri",
        command: "https://meet.google.com/new",
        exe_hints: &[],
    },
    Preset {
        id: "skype",
        kind: "launch",
        command: "skype:",
        exe_hints: &[
            "%LOCALAPPDATA%\\Microsoft\\Skype for Desktop\\Skype.exe",
            "C:\\Program Files\\Microsoft\\Skype for Desktop\\Skype.exe",
        ],
    },
    Preset {
        id: "chatgpt_codex",
        kind: "launch",
        // Fallback to the Codex web page when the desktop app is not installed.
        command: "https://chatgpt.com/codex",
        exe_hints: &["%LOCALAPPDATA%\\Programs\\ChatGPT\\ChatGPT.exe"],
    },
    Preset {
        id: "claude",
        kind: "launch",
        // Fallback to Claude on the web when the desktop app is not installed.
        command: "https://claude.ai/",
        exe_hints: &["%LOCALAPPDATA%\\AnthropicClaude\\claude.exe"],
    },
    Preset {
        id: "beep",
        kind: "beep",
        command: "",
        exe_hints: &[],
    },
    Preset {
        id: "custom",
        kind: "launch",
        command: "",
        exe_hints: &[],
    },
];

pub fn find(id: &str) -> Option<&'static Preset> {
    ALL.iter().find(|p| p.id == id)
}
