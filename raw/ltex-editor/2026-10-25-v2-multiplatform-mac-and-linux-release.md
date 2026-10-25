# LaTeX Editor Version 2.0 Multi-Platform Release (macOS and Linux)

> Source: GitHub Actions Build #40197301450 and GitHub Release v2.0.0
> Collected: 2026-10-25
> Published: 2026-10-25

LaTeX Editor Version 2.0 (v2.0.0) successfully compiled and published native desktop installers for macOS (Apple Silicon M-Series) and Linux (.deb, .AppImage, .rpm) via GitHub Actions workflow run 40197301450, completing the multi-platform Version 2.0 release.

Official Published Multi-Platform Packages (v2.0.0):
- Windows 10 / 11:
  - `LaTeX-Editor-Setup-v2.0.0.exe` (26.6 MB / 27,932,441 bytes)
  - `LaTeX-Editor-Native-Setup.exe` (26.6 MB / 27,932,441 bytes)
- macOS (Apple Silicon M1/M2/M3/M4):
  - `LaTeX.Editor_2.0.0_aarch64.dmg` (45.7 MB / 45,698,825 bytes)
  - `LaTeX-Editor_2.0.0_aarch64.app.tar.gz` (45.6 MB / 45,640,914 bytes)
- Linux (64-bit):
  - `LaTeX.Editor_2.0.0_amd64.deb` (38.8 MB / 38,820,748 bytes)
  - `LaTeX.Editor_2.0.0_amd64.AppImage` (111.9 MB / 111,905,272 bytes)
  - `LaTeX.Editor-2.0.0-1.x86_64.rpm` (38.8 MB / 38,820,647 bytes)

Workflow Enhancements:
- Added explicit release asset publishing via `gh release upload` with `--clobber` support to guarantee direct attachment of large binary payloads to GitHub releases.
- Automated packaging of macOS `.app` bundle directory into compressed `.tar.gz` archive.
- Attached build outputs as GitHub Actions workflow artifacts (`latex-editor-macos-latest` and `latex-editor-ubuntu-22.04`).
