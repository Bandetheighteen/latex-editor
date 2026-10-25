# LaTeX Editor Version 2.0: Projects Dashboard, Direct Disk Saving, and Autocomplete

> Source: Local project repository (src-tauri/src/lib.rs, src-tauri/tauri.conf.json, public/main.js, public/latex-autocomplete.js, public/index.html, README.md, and GitHub Release v2.0.0)
> Collected: 2026-10-25
> Published: 2026-10-25

LaTeX Editor Version 2.0 (v2.0.0) upgrades the application from a single-buffer offline editor into a native desktop IDE featuring project management, direct disk file editing, OS file association, project-relative asset compilation, and Overleaf-style autocomplete.

Key Version 2.0 Architecture & Features:
- Projects & Workspaces Dashboard:
  - Added a two-view architecture (`#welcome-view` and `#editor-view`) switching seamlessly without page reloads.
  - Recent Projects Management: Stores up to 30 recent projects in user app data (`recent_projects.json`). Displays relative access timestamps, file sizes, existence status badges, and deletion controls.
  - Instant Filter: Real-time search input filters projects by name or directory path.
  - Blank Slate Fix: Clicking "New Project" explicitly resets the editor to an empty string (`setValue('')`), clears undo history (`clearHistory()`), revokes previous PDF blob URLs, clears the PDF canvas DOM, and renders an empty preview placeholder guidance card.
  - Quick Starters: Built-in starter templates for Research Article, Beamer Slides, Resume / CV, and Math Cheatsheet.
- Direct Disk Saving & OS File Associations:
  - Registered OS file associations in `tauri.conf.json` for `.tex`, `.latex`, `.ltx`, `.txt`, and `.md` file types.
  - "Open With..." Integration: Native command line argument inspection (`get_launch_file` in Rust) parses `std::env::args_os()` to open files directly into the editor when launched via Windows Explorer context menu.
  - Native File Picker: Rust command `open_file_dialog` invokes the native file dialog for opening local documents.
  - Atomic File Saving: Rust command `save_file_direct` writes code to a temporary buffer in the document's parent directory before atomically renaming to the target path, preventing partial writes or file corruption on system interrupts.
  - `Ctrl+S` / `Cmd+S` Shortcut: Saves modifications directly to the opened disk file and triggers immediate compilation.
- Overleaf-Style Intelligent Autocomplete Engine:
  - Integrated local offline CodeMirror 5 hint provider (`public/latex-autocomplete.js`) utilizing offline assets `show-hint.min.js` and `show-hint.min.css`.
  - Environment Snippet Expansion: Typing `\begin` or inside `\begin{...}` provides dropdown options (`itemize`, `enumerate`, `description`, `equation`, `align`, `figure`, `table`, `abstract`, `pmatrix`, `bmatrix`, `theorem`) and expands full snippets with matching `\end{...}` tags, placing cursor inside.
  - Standard Commands & Math: Autocompletes standard LaTeX macros, Greek letters (`\alpha`, `\beta`, `\pi`, etc.), and mathematical operators (`\sum`, `\int`, `\infty`, `\partial`, etc.).
  - Label Discovery: Dynamically scans the active document buffer for `\label{...}` entries to populate autocomplete options inside `\ref{` and `\cite{`.
  - Triggering: Triggers automatically on typing `\` or `{`, as well as on manual shortcut `Ctrl+Space`.
- Project-Relative Asset Compilation:
  - Updated `compile_latex` in Rust to inspect the optional `project_path` and set `cmd.current_dir(parent_dir)`. Allows relative images (`\includegraphics{images/...}`) and included sub-documents (`\input{sections/...}`) to resolve naturally during compilation.
- Repository Rename & Release Packaging:
  - Repository renamed from `Bandetheighteen/ltex-editor` to `Bandetheighteen/latex-editor`.
  - Published official GitHub Release `v2.0.0` with the standalone Windows installer `LaTeX-Editor-Setup-v2.0.0.exe` (26.64 MiB / 27,932,441 bytes) and universal installer `LaTeX-Editor-Native-Setup.exe` (27,932,441 bytes) available for direct download and mirrored in the repository `release/` folder.
