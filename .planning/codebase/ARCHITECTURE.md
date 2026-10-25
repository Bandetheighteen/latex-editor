---
last_mapped_commit: 5b68d289dda9aa8130b04bde7f997b5e780b19de
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Architecture

**Analysis Date:** 2026-10-25

## System Overview

```text
┌────────────────────────────────────────────────────────────────────────┐
│                      Frontend Presentation Layer                        │
├──────────────────────────┬──────────────────────────┬──────────────────┤
│   CodeMirror 5 Editor    │   PDF.js Canvas Engine   │  Split.js Panes  │
│    `public/main.js`      │    `public/main.js`      │ `public/main.js` │
└────────────┬─────────────┴────────────┬─────────────┴──────────┬───────┘
             │                          │                        │
             ▼                          ▼                        ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        IPC Adapter Boundary                            │
│                     `public/backend-adapter.js`                        │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ window.__TAURI__.core.invoke
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    Native Rust Backend (Tauri v2)                      │
│                       `src-tauri/src/lib.rs`                           │
│  - compile_latex        - synctex        - save_pdf       - save_docx  │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Child Process Execution
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    Local System Tooling & Bundled Binaries             │
│   - pdflatex (MiKTeX/TeX Live)       - synctex CLI                     │
│   - pandoc (`src-tauri/binaries/`)   - tinyfiledialogs (Native Win/Mac)│
│   - Workspace: `std::env::temp_dir()/latex-editor-workspace/`          │
└────────────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| `public/index.html` | Application shell, controls header, dual split panes, zoom stepper, auto-compile toggle, export dropdown | `public/index.html` |
| `public/main.js` | UI events, CodeMirror initialization, multi-page PDF.js rendering, SyncTeX click coordination, zoom calculations, local draft persistence | `public/main.js` |
| `public/backend-adapter.js` | Strict IPC abstraction bridging JavaScript frontend with Tauri v2 commands | `public/backend-adapter.js` |
| `public/style.css` | Apple HIG dark aesthetic, glassmorphism, responsive split gutters, active line spotlight, luminous SyncTeX pulse | `public/style.css` |
| `src-tauri/src/lib.rs` | Tauri commands (`compile_latex`, `synctex`, `save_pdf`, `save_docx`), system PATH resolution, process supervision, workspace management | `src-tauri/src/lib.rs` |
| `src-tauri/src/main.rs` | Rust desktop entry point delegating to `app_lib::run()` | `src-tauri/src/main.rs` |
| `setup-pandoc.js` | Architecture and OS detection for automated download/extraction of standalone Pandoc binary into `src-tauri/binaries/` | `setup-pandoc.js` |
| `setup-vendors.js` | Offline dependency downloader ensuring CodeMirror, PDF.js, and Split.js are available without internet | `setup-vendors.js` |

## Pattern Overview

**Overall:** Local-First Desktop Architecture with IPC Command Dispatch

**Key Characteristics:**
- **Zero-Network Decoupling:** Complete separation from external servers; all operations execute locally through system child processes.
- **Asynchronous IPC:** Frontend dispatches async commands to Tauri Rust backend via `window.__TAURI__.core.invoke()`, preventing UI thread blocking during compilation.
- **Resilient File Synchronization:** Transient files (`document.tex`, `document.pdf`, `document.synctex.gz`, `document.docx`) are isolated in the OS temporary directory (`latex-editor-workspace`), while source drafts persist in browser `localStorage`.

## Layers

**Presentation Layer (`public/`):**
- Purpose: Render user interface, manage editor state, display compiled PDF canvas, handle zoom and split layout.
- Location: `public/index.html`, `public/main.js`, `public/style.css`
- Contains: CodeMirror 5 configuration, PDF.js integration, DOM event listeners.
- Depends on: Local vendor scripts (`public/vendor/`).
- Used by: Tauri WebView rendering engine.

**Adapter Boundary (`public/backend-adapter.js`):**
- Purpose: Normalize backend communication into a unified `BackendAdapter` static class.
- Location: `public/backend-adapter.js`
- Contains: `invokeCompile`, `invokeSyncTeX`, `invokeSavePdf`, `invokeSaveDocx`.
- Depends on: `window.__TAURI__.core`.
- Used by: `public/main.js`.

**Backend System Layer (`src-tauri/`):**
- Purpose: Execute TeX compilation, coordinate SyncTeX lookups, run Pandoc conversion, spawn native OS save dialogs.
- Location: `src-tauri/src/lib.rs`, `src-tauri/src/main.rs`
- Contains: Tauri command handlers, PATH configuration, child process spawning.
- Depends on: `tauri`, `serde`, `tinyfiledialogs`, `tempfile`.
- Used by: Native desktop executable.

## Data Flow

### Primary Request Path (LaTeX Compilation)

1. User triggers compilation via `Ctrl+S`, `Cmd+S`, `Ctrl+Enter`, `Compile` button, or auto-compile debounce (`public/main.js:98-107, 309-328`).
2. `BackendAdapter.invokeCompile(latexCode)` sends IPC command `compile_latex` with current text payload (`public/backend-adapter.js:6-20`).
3. Rust command writes `document.tex` to `latex-editor-workspace` and executes `pdflatex -synctex=1 -interaction=nonstopmode -halt-on-error` (`src-tauri/src/lib.rs:40-54`).
4. On success, `document.pdf` is read as a byte buffer (`Vec<u8>`) and returned in `CompileResponse` (`src-tauri/src/lib.rs:58-69`).
5. Frontend converts byte buffer into an `ArrayBuffer`, creates a Blob URL, feeds it into `pdfjsLib.getDocument()`, and loops through all pages rendering each onto `<canvas class="pdf-canvas">` (`public/main.js:159-224, 238-250`).

### Secondary Flow Name (Bidirectional SyncTeX Navigation)

1. User clicks or double-clicks on the rendered PDF preview canvas (`public/main.js:180-222`).
2. Frontend converts mouse client coordinates into PDF page coordinates using viewport scale and element dimensions (`public/main.js:182-192`).
3. `BackendAdapter.invokeSyncTeX(page, pdfX, pdfY)` calls Rust `synctex` command (`public/backend-adapter.js:22-30`).
4. Rust backend probes vertical offsets (`0.0, -4.0, 4.0, -8.0, 8.0, -14.0, 14.0`) against `synctex edit` to accommodate line spacing and margins (`src-tauri/src/lib.rs:112-122`).
5. Line number is returned, CodeMirror scrolls to target line, sets cursor/selection, and flashes line with a luminous highlight (`public/main.js:196-214`).

### Tertiary Flow Name (Word .docx Export via Pandoc)

1. User selects "Export Word (.docx)" from the Export dropdown (`public/main.js:293-306`).
2. `BackendAdapter.invokeSaveDocx(latexCode)` calls `save_docx` command (`public/backend-adapter.js:46-58`).
3. Rust backend writes `document.tex`, discovers Pandoc path via `get_pandoc_path()`, executes `pandoc -s document.tex -o document.docx`, and launches native file dialog via `tinyfiledialogs::save_file_dialog_with_filter` (`src-tauri/src/lib.rs:177-210`).
4. Output file is copied to user's selected file path.

**State Management:**
- Source Code: Kept in CodeMirror instance and synchronized into `localStorage['latex_source_code']` on change.
- Auto-Compile Toggle: Boolean state tracked in `isAutoCompile` and synchronized into `localStorage['auto_compile_enabled']`.
- Zoom Factor: Tracked in `currentScale` (default 1.35, range 0.6–3.0) and displayed via `updateZoomDisplay()`.
- Compilation Artifacts: Persisted in OS temp directory (`latex-editor-workspace`).

## Key Abstractions

**`BackendAdapter` (`public/backend-adapter.js`):**
- Purpose: Encapsulates desktop IPC communication, ensuring the frontend code remains decoupled from Tauri specifics.
- Pattern: Static Gateway / Facade Pattern.

**`create_system_command` (`src-tauri/src/lib.rs:14-30`):**
- Purpose: Cross-platform command factory augmenting execution PATH with TeX distributions on macOS and Linux.
- Pattern: Factory Method Pattern.

**`get_pandoc_path` (`src-tauri/src/lib.rs:148-174`):**
- Purpose: Robust multi-location binary discovery searching resources, app bundles, development paths, and system PATH.
- Pattern: Strategy / Fallback Chain Pattern.

## Entry Points

**Desktop Native Executable:**
- Location: `src-tauri/src/main.rs`
- Triggers: Application launch by user.
- Responsibilities: Initializes Tauri runtime, sets up window configuration, registers IPC command handlers, and boots the WebView.

**Frontend Application:**
- Location: `public/main.js`
- Triggers: `DOMContentLoaded` event inside the WebView.
- Responsibilities: Initializes Split.js, CodeMirror, and event listeners; restores saved state; triggers initial compilation.

## Architectural Constraints

- **Threading:** Rust backend executes async commands in Tauri's threadpool. Child processes (`pdflatex`, `synctex`, `pandoc`) are spawned synchronously within async tasks, preserving UI responsiveness.
- **Global State:** Zero global mutable static variables in Rust. All state is either derived from the filesystem workspace or managed per command invocation.
- **No Remote Dependencies:** Zero web runtime servers, sockets, or HTTP services.
- **Single Window Context:** Default capabilities configure a single window `"main"`.

## Anti-Patterns

### Running Standalone Web Servers for Local Desktop Apps

**What happens:** Spawning a Node.js Express server to serve HTML files and receive HTTP compile requests locally.
**Why it's wrong:** High memory footprint, zombie processes left behind when window closes, port conflicts (`EADDRINUSE`), security exposure on `localhost`.
**Do this instead:** Use Tauri v2 IPC (`#[tauri::command]` and `window.__TAURI__.core.invoke`), keeping everything in-process and memory-safe (`src-tauri/src/lib.rs`).

### Hardcoding TeX Engine Executable Paths

**What happens:** Assuming `pdflatex` or `pandoc` is strictly located at a hardcoded Windows or Unix path.
**Why it's wrong:** Different distributions (MiKTeX, TeX Live, MacTeX) and install methods (winget, brew, apt) use different paths.
**Do this instead:** Augment standard platform PATH search directories and use dynamic multi-directory discovery chains (`src-tauri/src/lib.rs:14-30, 148-174`).

## Error Handling

**Strategy:** Comprehensive boundary checking with graceful visual feedback.

**Patterns:**
- Rust errors are transformed into human-readable strings via `.map_err(|e| e.to_string())` and returned as `Result<T, String>` to JavaScript.
- LaTeX compilation errors capture standard output / stderr and display an expandable error overlay with the `document.log` excerpt.
- Process failures return descriptive messages without terminating the application.

## Cross-Cutting Concerns

**Logging:** Native logging enabled via `tauri-plugin-log` in debug mode (`src-tauri/src/lib.rs:218-222`).
**Validation:** `code` input passed directly to file write; child process arguments strictly tokenized to avoid shell injection.
**Security:** Tauri window CSP enabled; no arbitrary shell execution (direct executable execution without shell wrapping).

---

*Architecture analysis: 2026-10-25*
