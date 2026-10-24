# Ltex Editor

> Sources: Local project repository, 2026-10-08; Local project repository, 2026-10-24
> Raw: [ltex-editor-project-context](../../raw/ltex-editor/ltex-editor-project-context.md); [2026-10-24-v1-native-release-and-word-export](../../raw/ltex-editor/2026-10-24-v1-native-release-and-word-export.md)
> Updated: 2026-10-24

## Overview

Ltex Editor is a local-first, 100% offline native desktop LaTeX editor designed to eliminate reliance on internet-dependent cloud tools like Overleaf. Built around an Apple HIG-inspired dual-pane layout (CodeMirror on the left, live PDF.js preview on the right), it provides crash-free compilation, bidirectional SyncTeX navigation, and native Microsoft Word (`.docx`) export.

## Architecture

The project has transitioned from its initial dual-target setup into a pure native Windows desktop application:

- **Frontend:** CodeMirror 5 editor (syntax highlighting, active line spotlight), offline PDF.js engine, and Split.js resizable dual panes.
- **Native Backend (Tauri v2 & Rust):** Rust child process management handles `pdflatex` compilation and `synctex` lookups. Background processes enforce strict 10-second timeouts to prevent runaway loops or CPU freezing.
- **Embedded Pandoc Converter:** Standalone Pandoc binary (version 3.12.1) bundled into application resources (`src-tauri/binaries/pandoc.exe`), converting LaTeX files directly into Microsoft Word (`.docx`) format with native editable Equation Editor (OMML) objects.
- **Native Win32 Dialogs:** Uses `tinyfiledialogs` via direct Win32/COM calls to guarantee native Windows File Explorer Save dialogs for exports.

> **Status: Outdated** (2026-10-24)
> The Express/Node.js web server (`server.js`) and web distribution package have been completely removed from the project in favor of the unified Tauri native desktop architecture.

## Core Features

- **Side-by-Side Live Workflow:** Instant local rendering via PDF.js with responsive zoom stepper controls (60% to 300%).
- **Precision Bidirectional SyncTeX:** Clicking or double-clicking anywhere on the rendered PDF preview resolves the mouse position to PDF points, probes vertical offsets (`0.0, -4.0, +4.0, -8.0, +8.0, -14.0, +14.0`) to avoid margin gaps, selects the corresponding line in CodeMirror, smoothly centers it in the viewport, and flashes it with a luminous green highlight.
- **Auto-Compilation Control:** Header toggle button (`ON` / `OFF`) defaulting to OFF. Users draft without interruptions and compile on demand (`Ctrl+S` / `Ctrl+Enter`), or toggle auto-compilation on (1200ms debounce). Setting persists across sessions in `localStorage`.
- **Unified Export Menu:** Integrated dropdown providing 1-click export to both PDF (`.pdf`) and Microsoft Word (`.docx`).

## Prerequisites & Installation

- **System Requirement:** Requires a local TeX distribution installed on the host system to supply `pdflatex` and `synctex` executables in PATH. **MiKTeX** is strongly recommended as the baseline platform, with **TeX Live** supported as an alternative.
- **Distribution:** Hosted on GitHub at [Bandetheighteen/ltex-editor](https://github.com/Bandetheighteen/ltex-editor). Official binary installer `LaTeX-Editor-Native-Setup.exe` (26.6 MB) is published under GitHub Release `v1.0.0`.

## Authorship & Legal Licensing

- **Author:** Created and designed by **Shounak Saha** (*S-H-O-U-N-A-K  S-A-H-A*), a non-coder who hardcoded and vibe-coded the entire software from scratch to resolve personal research friction. The project maintains an active call for open-source contributors and collaborators.
- **License:** Distributed under the **MIT License** &copy; 2026 Shounak Saha, granting full open-source usage rights while legally protecting the author through disclaimer of warranty and mandatory attribution.
