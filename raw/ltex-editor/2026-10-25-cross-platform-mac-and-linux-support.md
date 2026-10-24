# Cross-Platform Support for macOS and Linux

> Source: Local project repository (src-tauri/src/lib.rs, setup-pandoc.js, src-tauri/tauri.conf.json, .github/workflows/release.yml, README.md, and GitHub Release v1.1.0)
> Collected: 2026-10-25
> Published: 2026-10-25

LaTeX Editor expands its native desktop support beyond Windows to include macOS (Universal / Apple Silicon / Intel) and Linux (.deb and .AppImage).

Key Cross-Platform Architecture:
- TeX Environment Discovery: macOS GUI applications do not inherit interactive terminal shell PATH. Added automated PATH resolution in Rust (`create_system_command`) targeting MacTeX (`/Library/TeX/texbin`), Homebrew (`/opt/homebrew/bin`, `/usr/local/bin`), and Linux system directories (`/usr/local/bin`, `/usr/bin`).
- Dynamic Pandoc Resolution: Updated `get_pandoc_path()` to dynamically locate `pandoc` (on macOS and Linux) and `pandoc.exe` (on Windows), probing application bundle resources (including macOS `.app/Contents/Resources`), local directories, and system PATH.
- Cross-Platform Binary Setup: Modernized `setup-pandoc.js` to detect platform and architecture (`darwin` arm64/x64, `linux` amd64/arm64, `win32` x64), download the matching Pandoc release archive (tar.gz or zip), unpack it with built-in `tar`, and apply executable permissions (`0o755`) on Unix.
- Multi-Platform Packaging: Configured `tauri.conf.json` with multi-platform icon assets (`icon.icns` for macOS, multi-resolution PNGs for Linux, and `icon.ico` for Windows), bundle resources (`binaries/*`), and Debian packaging dependencies (`libwebkit2gtk-4.1-0 | libwebkit2gtk-4.0-37`, `libgtk-3-0`).
- Automated CI/CD Release Pipeline: Created `.github/workflows/release.yml` with a multi-runner matrix (`macos-latest` and `ubuntu-22.04`) using Tauri Action to build and publish `.dmg`, `.deb`, `.AppImage`, and `.rpm` binaries on tag push or manual workflow dispatch.
- Official Published Multi-Platform Packages (v1.1.0):
  - macOS Disk Image: `LaTeX.Editor_1.0.0_aarch64.dmg` (45.6 MB)
  - Linux Debian package: `LaTeX.Editor_1.0.0_amd64.deb` (38.7 MB)
  - Linux AppImage: `LaTeX.Editor_1.0.0_amd64.AppImage` (111.8 MB)
  - Linux RPM package: `LaTeX.Editor-1.0.0-1.x86_64.rpm` (38.7 MB)
  - Windows Installer: `LaTeX-Editor-Native-Setup.exe` (26.6 MB)
