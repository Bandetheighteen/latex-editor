# LaTeX Editor

> Sources: Local project repository, 2026-10-08; Local project repository, 2026-10-24; Local project repository, 2026-10-25; GitHub Actions and Release v2.0.0, 2026-10-25
> Raw: [ltex-editor-project-context](../../raw/ltex-editor/ltex-editor-project-context.md); [2026-10-24-v1-native-release-and-word-export](../../raw/ltex-editor/2026-10-24-v1-native-release-and-word-export.md); [2026-10-25-cross-platform-mac-and-linux-support](../../raw/ltex-editor/2026-10-25-cross-platform-mac-and-linux-support.md); [2026-10-25-v2-projects-dashboard-and-autocomplete](../../raw/ltex-editor/2026-10-25-v2-projects-dashboard-and-autocomplete.md); [2026-10-25-v2-multiplatform-mac-and-linux-release](../../raw/ltex-editor/2026-10-25-v2-multiplatform-mac-and-linux-release.md)
> Updated: 2026-10-25

## Overview

LaTeX Editor is a local-first, 100% offline native desktop LaTeX editor and IDE designed to eliminate reliance on internet-dependent cloud tools like Overleaf. Built around an Apple HIG-inspired dual-pane interface (CodeMirror on the left, live PDF.js preview on the right), it provides crash-free compilation, bidirectional SyncTeX navigation, a Welcome and Projects Dashboard, direct disk file saving, OS file associations, Overleaf-style autocomplete, and native Microsoft Word (`.docx`) export across Windows, macOS, and Linux.

## Architecture

The project is structured as a cross-platform native desktop application powered by Tauri v2:

- **Frontend:** CodeMirror 5 editor (syntax highlighting, active line spotlight, and offline `show-hint` autocompletion), offline PDF.js engine, and Split.js resizable dual panes. Supports a two-view architecture (`#welcome-view` and `#editor-view`) switching without full page reload. Keyboard shortcuts natively map to both `Ctrl` (Windows/Linux) and `Cmd`/`metaKey` (macOS).
- **Native Backend (Tauri v2 & Rust):** Rust child process management handles `pdflatex` compilation, `synctex` lookups, and persistent storage of up to 30 recent projects in user app data (`recent_projects.json`). Background processes enforce strict 10-second timeouts to prevent runaway loops or CPU freezing.
- **Direct Disk Saving & OS File Associations:** Rust command `save_file_direct` executes atomic file writes to prevent corruption. Command line argument inspection (`get_launch_file`) detects OS "Open With..." launches via Windows Explorer for registered extensions (`.tex`, `.latex`, `.ltx`, `.txt`, `.md`).
- **Project-Relative Asset Compilation:** Rust `compile_latex` dynamically sets `cmd.current_dir` to the document's parent directory, resolving external figures (`\includegraphics`) and child files (`\input`) without copying parent folders to temporary storage.
- **Offline Autocomplete Engine:** Custom CodeMirror 5 hint provider (`latex-autocomplete.js`) with dynamic environment expansion (`\begin`), standard LaTeX macros, Greek math symbols, and active document `\label` parsing.
- **Cross-Platform PATH Resolution:** GUI applications launched on macOS do not inherit terminal shell PATH. A native Rust command builder (`create_system_command`) augments execution PATH on macOS (`/Library/TeX/texbin`, `/opt/homebrew/bin`, `/usr/local/bin`, `/usr/bin`) and Linux (`/usr/local/bin`, `/usr/bin`).
- **Dynamic Pandoc Converter:** Standalone Pandoc binary (version 3.12.1) bundled into application resources, converting LaTeX files directly into Microsoft Word (`.docx`) format with native editable Equation Editor (OMML) objects. `get_pandoc_path()` dynamically discovers `pandoc.exe` on Windows and `pandoc` on macOS and Linux across resource directories, app bundles (`.app/Contents/Resources`), and system PATH.
- **Cross-Platform Binary Setup:** `setup-pandoc.js` downloads and extracts platform-specific binaries (`darwin` arm64/x64, `linux` amd64/arm64, `win32` x64) using system `tar` and enforces Unix executable permissions (`0o755`).
- **Automated CI/CD Pipeline:** Multi-runner GitHub Actions workflow (`.github/workflows/release.yml`) builds `.dmg` for macOS, as well as `.deb`, `.AppImage`, and `.rpm` for Linux on tag push or manual workflow dispatch.

> **Status: Outdated** (2026-10-24)
> The Express/Node.js web server (`server.js`) and web distribution package have been completely removed from the project in favor of the unified Tauri native desktop architecture.

## Core Features

- **Projects & Workspaces Dashboard (v2.0):** Clean dashboard displaying recent projects with timestamps and file sizes, real-time search filtering, pre-configured Quick Starters (Research Article, Beamer Presentation, Resume / CV, Math Cheatsheet), and a fresh blank canvas on "+ New Project".
- **Direct Disk Saving & OS File Associations (v2.0):** Double-click or right-click `.tex`, `.latex`, `.ltx`, `.txt`, and `.md` files to open directly in the editor; `Ctrl+S` / `Cmd+S` saves edits directly to disk with atomic safety.
- **Overleaf-Style Autocomplete & Suggestions (v2.0):** Type `\begin` for instant dropdown suggestions with auto-closing tags and snippet templates, plus math symbols, commands, and document `\label` referencing.
- **Side-by-Side Live Workflow:** Instant local rendering via PDF.js with responsive zoom stepper controls (60% to 300%).
- **Precision Bidirectional SyncTeX:** Clicking or double-clicking anywhere on the rendered PDF preview resolves the mouse position to PDF points, probes vertical offsets (`0.0, -4.0, +4.0, -8.0, +8.0, -14.0, +14.0`) to avoid margin gaps, selects the corresponding line in CodeMirror, smoothly centers it in the viewport, and flashes it with a luminous green highlight.
- **Auto-Compilation Control:** Header toggle button (`ON` / `OFF`) defaulting to OFF. Users draft without interruptions and compile on demand (`Ctrl+S` / `Ctrl+Enter` / `Cmd+S`), or toggle auto-compilation on (1200ms debounce). Setting persists across sessions in `localStorage`.
- **Unified Export Menu:** Integrated dropdown providing 1-click export to both PDF (`.pdf`) and Microsoft Word (`.docx`).

## Prerequisites & Installation

- **System Requirement:** Requires a local TeX distribution installed on the host system to supply `pdflatex` and `synctex` executables in PATH:
  - **Windows:** **MiKTeX** (`winget install MiKTeX.MiKTeX`) or **TeX Live**. Run installer setup wizard. Windows SmartScreen info notice bypass via "More info" -> "Run anyway".
  - **macOS:** **MacTeX** (`brew install --cask mactex-no-gui`) or **BasicTeX** (`brew install --cask basictex`). Mount DMG, drag to `/Applications`. Gatekeeper first-launch bypass via Right-click -> Open -> Open.
  - **Linux:** **TeX Live** (`sudo apt install texlive-latex-base texlive-latex-extra texlive-binaries` or Fedora `sudo dnf install texlive-scheme-basic texlive-synctex`). Standalone AppImage execution (`chmod +x LaTeX.Editor_1.0.0_amd64.AppImage`), Debian install (`sudo dpkg -i LaTeX.Editor_1.0.0_amd64.deb`), or RPM install (`sudo dnf install ./LaTeX.Editor-1.0.0-1.x86_64.rpm`).
- **Distribution:** Hosted on GitHub at [Bandetheighteen/latex-editor](https://github.com/Bandetheighteen/latex-editor). Multi-platform releases publish installers for:
  - Windows: `LaTeX-Editor-Setup-v2.0.0.exe` (26.6 MB / 27,932,441 bytes) and `LaTeX-Editor-Native-Setup.exe` (26.6 MB / 27,932,441 bytes)
  - macOS: `LaTeX.Editor_2.0.0_aarch64.dmg` (45.7 MB / 45,698,825 bytes) and `LaTeX-Editor_2.0.0_aarch64.app.tar.gz` (45.6 MB / 45,640,914 bytes)
  - Linux: `LaTeX.Editor_2.0.0_amd64.deb` (38.8 MB / 38,820,748 bytes), `LaTeX.Editor_2.0.0_amd64.AppImage` (111.9 MB / 111,905,272 bytes), and `LaTeX.Editor-2.0.0-1.x86_64.rpm` (38.8 MB / 38,820,647 bytes).

## Authorship & Legal Licensing

- **Author:** Created and designed by **Shounak Saha** (*S-H-O-U-N-A-K  S-A-H-A*), a non-coder who hardcoded and vibe-coded the entire software from scratch to resolve personal research friction. The project maintains an active call for open-source contributors and collaborators.
- **License:** Dual-licensed under the **MIT License** and **Apache License, Version 2.0** &copy; 2026 Shounak Saha, granting full open-source usage and patent rights while legally protecting the author through disclaimer of warranty and mandatory attribution.
