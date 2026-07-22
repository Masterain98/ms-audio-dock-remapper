# Build the release binary, then rename the .exe to a human-readable,
# space-containing process name ("MS Audio Dock Remapper"). Cargo does not allow
# spaces in a crate/bin name, so the rename is done here as a post-build step.
# The app launches via its full path (current_exe, quoted for autostart), so the
# renamed filename is fully supported — no other code depends on the old name.

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$outDir = Join-Path $root 'target/release'
$oldExe = Join-Path $outDir 'ms-audio-dock-remapper.exe'
$newExe = Join-Path $outDir 'MS Audio Dock Remapper.exe'

# Stop any running instance so the file is not locked during the rename.
Get-Process -Name 'ms-audio-dock-remapper' -ErrorAction SilentlyContinue | Stop-Process -Force
Get-Process -Name 'MS Audio Dock Remapper' -ErrorAction SilentlyContinue | Stop-Process -Force

cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if (Test-Path $oldExe) {
    if (Test-Path $newExe) { Remove-Item -LiteralPath $newExe -Force }
    Move-Item -LiteralPath $oldExe -Destination $newExe -Force
    Write-Output "Built: $newExe"
} else {
    Write-Output "Nothing to rename (already built / $oldExe missing)."
}
