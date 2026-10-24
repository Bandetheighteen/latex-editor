# LaTeX Editor v1.0 Native Release and Feature Expansion

> Source: Local project repository (README.md, src-tauri/src/lib.rs, and GitHub Release v1.0.0)
> Collected: 2026-10-24
> Published: 2026-10-24

LaTeX Editor has finalized its transition into a 100% native offline desktop application for Windows, eliminating the legacy Express/Node.js web backend entirely.

Key Architectural State:
- Native Desktop Application: Built using Tauri v2 and Rust. The backend directly executes `pdflatex` and `synctex` through Rust child processes with 10-second timeout enforcement to prevent runaway processes.
- Embedded Pandoc Converter: Standalone Pandoc binary (version 3.12.1) bundled into Tauri application resources (`src-tauri/binaries/pandoc.exe`), enabling direct export of LaTeX documents to Microsoft Word (`.docx`) format with native editable Equation Editor (OMML) objects.
- Zoom Stepper: Responsive zoom stepper controls ranging from 60% to 300% scale.
- Unified Export Menu: Replaced standalone export buttons with an Apple HIG-styled unified dropdown offering both PDF and Word (.docx) exports with native Win32/COM file save dialogs powered by `tinyfiledialogs`.
- Auto-Compilation Control: Added a header toggle button (`ON` / `OFF`) defaulting to OFF. When disabled, drafting occurs without background compilation interruptions; compilation triggers manually via `Compile` button or `Ctrl+S` / `Ctrl+Enter`. When enabled, auto-compilation debounces after 1200ms. Preference persists across sessions in `localStorage`.
- High-Precision SyncTeX Navigation: Normalized canvas coordinate mapping with canvas-to-client scaling ratios, resolving Windows high-DPI scaling offsets (125%/150%). Added neighborhood vertical offset probing in Rust backend (`0.0, -4.0, +4.0, -8.0, +8.0, -14.0, +14.0`) so clicking text, margins, or line spacing reliably resolves to the nearest line. Configured both single click and double click to trigger navigation, automatically selecting the target line in CodeMirror, smoothly centering it in the editor viewport, and flashing it with a luminous green highlight.
- Prerequisite: Requires a local TeX distribution (MiKTeX recommended, or TeX Live) installed on the host system to provide `pdflatex` and `synctex` binaries.
- Authorship & Licensing: Created and designed by Shounak Saha, a non-coder who hardcoded and vibe-coded the software from scratch to resolve offline academic research challenges. Licensed under the permissive MIT License (Copyright (c) 2026 Shounak Saha) with an open call for community collaboration.
- Official Distribution: Live on GitHub at `https://github.com/Bandetheighteen/ltex-editor`. Published official GitHub Release `v1.0.0` with the standalone Windows installer `LaTeX-Editor-Native-Setup.exe` (26.6 MB) available for direct download.
