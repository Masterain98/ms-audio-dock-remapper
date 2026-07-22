# Contributing to MS Audio Dock Teams Key Remapper

Thank you for improving the project. This document describes the supported
development environment, local build and test commands, architectural rules,
and the expected contribution workflow.

## Supported development platform

The application currently provides a functional backend only on Windows. Use
an x64 Windows 10 or Windows 11 development machine for implementation and
verification. A physical Microsoft Audio Dock is required for end-to-end HID
testing, but most UI, configuration, filtering, and launch behavior can be
developed without the device.

## Development dependencies

Install the following tools:

1. **Git for Windows**
2. **Rust through rustup**, using the stable MSVC toolchain
3. **Visual Studio 2022 Build Tools** or Visual Studio 2022 with:
   - Desktop development with C++
   - A Windows 10 or Windows 11 SDK
4. **PowerShell 7** is recommended for the documented commands

The Windows SDK is required because `build.rs` uses `rc.exe` to embed the
application icon and Windows version resources.

For local installer builds, also install **Inno Setup 6.7.1**. It is not needed
for normal development, tests, or portable builds.

## Creating the development environment

Clone the repository and enter its directory:

```powershell
git clone <repository-url>
Set-Location ms-audio-dock-teams-remapper
```

Install and select the stable x64 MSVC toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup override set stable-x86_64-pc-windows-msvc
rustup component add rustfmt clippy
```

Fetch the locked dependency set and compile a development build:

```powershell
cargo fetch --locked
cargo build --locked
```

The development executable is created at:

```text
target\debug\ms-audio-dock-remapper.exe
```

## Running a development build

Run the application directly through Cargo:

```powershell
cargo run --locked
```

The application uses the Windows GUI subsystem, so it does not open a console
window. Fatal startup and action errors are displayed through the application
UI.

Local settings are read from and written to:

```text
%APPDATA%\ms-audio-dock-remapper\config.json
```

Back up this file before tests that intentionally change saved settings.

## Project structure

| Path | Responsibility |
| --- | --- |
| `src/main.rs` | Process startup, single-instance enforcement, monitor startup, and UI lifecycle |
| `src/ui.rs` | Slint UI definition, callbacks, application filtering, and UI state |
| `src/config.rs` | Backward-compatible JSON configuration model and persistence |
| `src/installed_apps.rs` | `shell:AppsFolder` enumeration, Shell icons, target recovery, and application activation |
| `src/actions.rs` | Custom command execution and registered-application actions |
| `src/i18n.rs` | English and Chinese UI strings |
| `src/platform/windows.rs` | Windows Raw Input, HID matching, tray behavior, registry autostart, and single-instance mutex |
| `src/platform/stub.rs` | Non-Windows placeholder backend |
| `public/` | Icons and other assets embedded at build time |
| `installer/` | Inno Setup installer definition |
| `.github/workflows/` | Manual validation, packaging, and release automation |

## Development rules

### Keep device access read-only

The project listens to Raw Input reports. Do not add firmware writes, HID output
reports, driver replacement, or administrator-only device access as part of a
normal feature change.

### Keep the input path responsive

The Raw Input window procedure must do the minimum work required to identify a
press and enqueue an action. File access, Shell activation, process launching,
and other potentially blocking work must run outside the window procedure.

Never hold the shared configuration mutex while invoking external code. Copy
the configuration needed for an action, release the lock, and then execute the
action. This prevents save/action deadlocks.

### Preserve configuration compatibility

Existing `config.json` files must continue to load after an update. New fields
should have sensible defaults and use the existing Serde defaulting strategy.
Do not silently replace a saved application target simply because it is
temporarily missing from `shell:AppsFolder`.

### Preserve application identity during filtering

Search results are a view over the registered application list. Selection and
saving must continue to resolve to the original AppsFolder target, not merely
to a filtered row index or display name.

### Keep platform-specific code isolated

Windows APIs belong in `src/platform/windows.rs` or another clearly scoped
Windows module. Platform-neutral UI and configuration code should not depend
directly on Windows handles or messages. Non-Windows builds must continue to
compile against the stub backend unless a complete backend is being added.

### Keep the UI localized

User-visible strings must be added to both the English and Chinese tables in
`src/i18n.rs`. Do not embed new user-facing text directly in UI callbacks or
the Slint markup when it should be translated.

### Write code comments in English

All source-code, build-script, workflow, and installer comments must be in
English. Comments should explain constraints and non-obvious decisions rather
than restating the code.

### Keep generated files out of commits

Do not commit `target/`, `dist/`, logs, IDE state, locally generated installers,
or portable archives. Do commit `Cargo.lock`; this is an application and its
release dependency graph must remain reproducible.

## Formatting and automated tests

Run the same checks used by the release workflow before opening a pull request:

```powershell
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

While editing, `cargo fmt --all` can apply Rust formatting automatically:

```powershell
cargo fmt --all
```

Add focused unit tests for behavior that can be tested without physical
hardware, especially configuration migration, AppsFolder selection, search
filtering, and index-to-target mapping.

## Manual verification

Changes that affect input handling, action execution, configuration, or UI
state should also complete this Windows test pass:

1. Start the app with the Dock disconnected and confirm it remains responsive.
2. Connect the Dock and confirm the status changes without restarting the app.
3. Select a registered application, use **Test action**, save, and press the
   Teams button.
4. Change the selected action, save again, and press the Teams button multiple
   times. The app must not freeze or launch the previous target.
5. Filter the application list, choose a result, save, and confirm the correct
   application launches.
6. Test a custom executable or URL with and without arguments.
7. Close the window, reopen it from the tray, and exit from the application
   menu.
8. If autostart code changed, enable and disable it and inspect the current
   user's Windows Run entry.
9. If UI code changed, verify English and Chinese at normal and high-DPI scale
   settings.

Include screenshots with pull requests that make visible UI changes.

## Building release artifacts locally

Create the optimized executable with:

```powershell
cargo build --release --locked
```

The Cargo output is:

```text
target\release\ms-audio-dock-remapper.exe
```

`build.ps1` is an optional convenience script that builds the release and
renames the executable to `MS Audio Dock Remapper.exe`. It force-stops running
instances before renaming, so save your work and exit the app before using it:

```powershell
.\build.ps1
```

To compile a local installer after the release executable has been built:

```powershell
New-Item -ItemType Directory -Path dist -Force | Out-Null
& "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe" `
  "/DMyAppVersion=0.1.0.0" `
  "installer\ms-audio-dock-remapper.iss"
```

Use a four-part numeric version for local installer builds because that value
is also written to the Windows executable version resource. Generated files in
`dist/` are intentionally ignored by Git.

## Release process

Production releases are created only through the **Manual Windows Release**
workflow in GitHub Actions. The workflow:

1. Runs formatting, tests, Clippy, and the optimized build.
2. Calculates a Beijing-time tag in the form `YYYY.MM.DD.N`.
3. Creates a portable ZIP and an Inno Setup installer.
4. Generates a SHA-256 checksum file for both artifacts.
5. Creates the tag and GitHub Release and uploads all three files.

Do not create a competing release tag manually while the workflow is running.
The workflow serializes release jobs so same-day counters remain unique.

## Pull requests

- Keep each pull request focused on one problem or feature.
- Explain the user-visible behavior and the reason for the change.
- List the automated and manual checks you completed.
- Call out configuration or compatibility effects explicitly.
- Update README files when user-facing behavior changes.
- Update this guide when build, test, or release requirements change.

By contributing, you agree that your contribution is provided under the
project's [MIT License](LICENSE).
