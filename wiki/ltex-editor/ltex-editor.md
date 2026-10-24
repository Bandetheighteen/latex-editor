# Ltex Editor

> Sources: Local project repository, 2026-10-08; Local project repository, 2026-10-24; Local project repository, 2026-10-25
> Raw: [ltex-editor-project-context](../../raw/ltex-editor/ltex-editor-project-context.md); [2026-10-24-v1-native-release-and-word-export](../../raw/ltex-editor/2026-10-24-v1-native-release-and-word-export.md); [2026-10-25-cross-platform-mac-and-linux-support](../../raw/ltex-editor/2026-10-25-cross-platform-mac-and-linux-support.md)
> Updated: 2026-10-25

## Overview

Ltex Editor is a local-first, 100% offline native desktop LaTeX editor designed to eliminate reliance on internet-dependent cloud tools like Overleaf. Built around an Apple HIG-inspired dual-pane layout (CodeMirror on the left, live PDF.js preview on the right), it provides crash-free compilation, bidirectional SyncTeX navigation, and native Microsoft Word (`.docx`) export across Windows, macOS, and Linux.

## Architecture

The project is structured as a cross-platform native desktop application powered by Tauri v2:

- **Frontend:** CodeMirror 5 editor (syntax highlighting, active line spotlight), offline PDF.js engine, and Split.js resizable dual panes. Keyboard shortcuts natively map to both `Ctrl` (Windows/Linux) and `Cmd`/`metaKey` (macOS).
- **Native Backend (Tauri v2 & Rust):** Rust child process management handles `pdflatex` compilation and `synctex` lookups. Background processes enforce strict 10-second timeouts to prevent runaway loops or CPU freezing.
- **Cross-Platform PATH Resolution:** GUI applications launched on macOS do not inherit terminal shell PATH. A native Rust command builder (`create_system_command`) augments execution PATH on macOS (`/Library/TeX/texbin`, `/opt/homebrew/bin`, `/usr/local/bin`, `/usr/bin`) and Linux (`/usr/local/bin`, `/usr/bin`).
- **Dynamic Pandoc Converter:** Standalone Pandoc binary (version 3.12.1) bundled into application resources, converting LaTeX files directly into Microsoft Word (`.docx`) format with native editable Equation Editor (OMML) objects. `get_pandoc_path()` dynamically discovers `pandoc.exe` on Windows and `pandoc` on macOS and Linux across resource directories, app bundles (`.app/Contents/Resources`), and system PATH.
- **Cross-Platform Binary Setup:** `setup-pandoc.js` downloads and extracts platform-specific binaries (`darwin` arm64/x64, `linux` amd64/arm64, `win32` x64) using system `tar` and enforces Unix executable permissions (`0o755`).
- **Automated CI/CD Pipeline:** Multi-runner GitHub Actions workflow (`.github/workflows/release.yml`) builds `.dmg` for macOS, as well as `.deb`, `.AppImage`, and `.rpm` for Linux on tag push or manual workflow dispatch.

> **Status: Outdated** (2026-10-24)
> The Express/Node.js web server (`server.js`) and web distribution package have been completely removed from the project in favor of the unified Tauri native desktop architecture.

## Core Features

- **Side-by-Side Live Workflow:** Instant local rendering via PDF.js with responsive zoom stepper controls (60% to 300%).
- **Precision Bidirectional SyncTeX:** Clicking or double-clicking anywhere on the rendered PDF preview resolves the mouse position to PDF points, probes vertical offsets (`0.0, -4.0, +4.0, -8.0, +8.0, -14.0, +14.0`) to avoid margin gaps, selects the corresponding line in CodeMirror, smoothly centers it in the viewport, and flashes it with a luminous green highlight.
- **Auto-Compilation Control:** Header toggle button (`ON` / `OFF`) defaulting to OFF. Users draft without interruptions and compile on demand (`Ctrl+S` / `Ctrl+Enter` / `Cmd+S`), or toggle auto-compilation on (1200ms debounce). Setting persists across sessions in `localStorage`.
- **Unified Export Menu:** Integrated dropdown providing 1-click export to both PDF (`.pdf`) and Microsoft Word (`.docx`).

## Prerequisites & Installation

- **System Requirement:** Requires a local TeX distribution installed on the host system to supply `pdflatex` and `synctex` executables in PATH:
  - **Windows:** **MiKTeX** (`winget install MiKTeX.MiKTeX`) or **TeX Live**. Run installer setup wizard. Windows SmartScreen info notice bypass via "More info" -> "Run anyway".
  - **macOS:** **MacTeX** (`brew install --cask mactex-no-gui`) or **BasicTeX** (`brew install --cask basictex`). Mount DMG, drag to `/Applications`. Gatekeeper first-launch bypass via Right-click -> Open -> Open.
  - **Linux:** **TeX Live** (`sudo apt install texlive-latex-base texlive-latex-extra texlive-binaries` or Fedora `sudo dnf install texlive-scheme-basic texlive-synctex`). Standalone AppImage execution (`chmod +x LaTeX.Editor_1.0.0_amd64.AppImage`), Debian install (`sudo dpkg -i LaTeX.Editor_1.0.0_amd64.deb`), or RPM install (`sudo dnf install ./LaTeX.Editor-1.0.0-1.x86_64.rpm`).
- **Distribution:** Hosted on GitHub at [Bandetheighteen/ltex-editor](https://github.com/Bandetheighteen/ltex-editor). Multi-platform releases publish installers for:
  - Windows: `LaTeX-Editor-Native-Setup.exe` (26.6 MB)
  - macOS: `LaTeX.Editor_1.0.0_aarch64.dmg` (45.6 MB) and `LaTeX.Editor_aarch64.app.tar.gz` (45.5 MB)
  - Linux: `LaTeX.Editor_1.0.0_amd64.deb` (38.7 MB), `LaTeX.Editor_1.0.0_amd64.AppImage` (111.8 MB), and `LaTeX.Editor-1.0.0-1.x86_64.rpm` (38.7 MB).

## Authorship & Legal Licensing

- **Author:** Created and designed by **Shounak Saha** (*S-H-O-U-N-A-K  S-A-H-A*), a non-coder who hardcoded and vibe-coded the entire software from scratch to resolve personal research friction. The project maintains an active call for open-source contributors and collaborators.
- **License:** Distributed under the **MIT License** &copy; 2026 Shounak Saha, granting full open-source usage rights while legally protecting the author through disclaimer of warranty and mandatory attribution.
