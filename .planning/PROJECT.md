# LaTeX Editor

## What This Is

A lightweight, crash-free, 100% offline native desktop LaTeX editor featuring an Apple HIG-inspired dual-pane interface: CodeMirror 5 on the left and a live PDF.js preview on the right. Built with Tauri v2 and Rust, it provides crash-free compilation, precision bidirectional SyncTeX navigation, and native Microsoft Word (`.docx`) export across Windows, macOS, and Linux.

## Core Value

A stable, bug-free, and high-performance offline LaTeX editing experience that never shuts down unexpectedly and reliably renders PDFs side-by-side with zero reliance on cloud services like Overleaf.

## Current State (v1.1)

The application has finalized its pure native desktop architecture across three major operating systems:
1. **Windows (10/11):** Standalone native installer (`.exe`) with `pdflatex`, precision SyncTeX, Win32 save dialogs, and bundled Pandoc Word export.
2. **macOS (Apple Silicon M-Series):** Native disk image (`.dmg`) and app bundle with automated `/Library/TeX/texbin` and Homebrew PATH resolution.
3. **Linux (Ubuntu / Debian / Fedora):** Native Debian packages (`.deb`), universal `.AppImage`, and `.rpm` packages with WebKitGTK and TeX Live integration.

*Note: The legacy Express web server (`server.js`) was decommissioned in favor of the unified Tauri v2 native desktop architecture.*

## Requirements

### Validated

- ✓ Basic side-by-side layout (editor left, PDF right)
- ✓ Integration with local LaTeX compiler (`pdflatex`)
- ✓ Precision SyncTeX support (PDF to source mapping with neighborhood vertical probing)
- ✓ Auto-compilation toggle button (ON/OFF with 1200ms debounce)
- ✓ Export to Microsoft Word (`.docx`) with native OMML equations via bundled Pandoc
- ✓ Re-architect native app using Tauri v2 (Rust) for performance and stability
- ✓ Process timeout safety (strict 10-second child process timeouts)
- ✓ macOS Apple Silicon support with automated PATH resolution
- ✓ Linux packaging support (`.deb`, `.AppImage`, `.rpm`)
- ✓ Automated multi-runner CI/CD GitHub Actions release pipeline

### Out of Scope

- [Cloud LaTeX compilation] — Compilation will remain local to maintain 100% offline security and zero server costs.
- [Heavy IDE extensions / bloated plugin ecosystems] — Keeping it focused, lightweight, and fast.

## Tech Stack & Architecture

- **Frontend:** CodeMirror 5, PDF.js, Split.js, Vanilla HTML/CSS/JS.
- **Backend:** Tauri v2, Rust (`std::process::Command`, `tinyfiledialogs`, `serde`).
- **Export Engine:** Bundled standalone Pandoc binary (version 3.12.1).
- **Packaging:** GitHub Actions CI/CD matrix (`macos-latest`, `ubuntu-22.04`).
- **Dependencies:** Requires local TeX distribution (`pdflatex`, `synctex`) installed on the host machine (MiKTeX on Windows, MacTeX on macOS, TeX Live on Linux).

## Key Decisions

| Decision | Rationale | Outcome |
|---|---|---|
| Tauri v2 (Rust) | Solves memory leaks and unexpected crashes while delivering high speed | Validated (v1.0) |
| Pure Native Desktop | Decommissioned Express web server to unify around single desktop binary | Validated (v1.0) |
| Embedded Pandoc | Bundled standalone Pandoc binary for offline Word (.docx) export with OMML equations | Validated (v1.0) |
| Automated TeX PATH Discovery | Augmented execution PATH on macOS (`/Library/TeX/texbin`, Homebrew) so GUI apps locate `pdflatex` | Validated (v1.1) |
| GitHub Actions Release Matrix | Hosted Apple Silicon and Ubuntu cloud runners automatically compile and publish `.dmg`, `.deb`, and `.AppImage` | Validated (v1.1) |

---
*Last updated: 2026-10-25 after v1.1.0 multi-platform release*
