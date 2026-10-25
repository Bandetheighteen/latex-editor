---
last_mapped_commit: 5b68d289dda9aa8130b04bde7f997b5e780b19de
last_mapped_at: 2026-10-25
---
# Coding Conventions

**Analysis Date:** 2026-10-25

## Naming Patterns

**Files:**
- JavaScript modules: `kebab-case.js` (e.g., `backend-adapter.js`, `setup-pandoc.js`, `setup-vendors.js`).
- Rust source files: `snake_case.rs` (e.g., `lib.rs`, `main.rs`, `build.rs`).
- CSS stylesheets: `kebab-case.css` (e.g., `style.css`).
- Markdown documents: `UPPERCASE.md` for planning and architecture docs (`STACK.md`, `ARCHITECTURE.md`, `CONVENTIONS.md`).

**Functions:**
- JavaScript: `camelCase` for functions and methods (e.g., `compileLatex()`, `updateAutoCompileUI()`, `renderPdf()`, `triggerSyncTexJump()`).
- Rust: `snake_case` for functions, commands, and local variables (e.g., `compile_latex()`, `get_workspace_dir()`, `create_system_command()`, `get_pandoc_path()`, `query_synctex_point()`).

**Variables:**
- JavaScript: `camelCase` for variables and constants (e.g., `savedDraft`, `isAutoCompile`, `pdfDoc`, `compileTimeout`); `UPPER_SNAKE_CASE` for global constant defaults (e.g., `DEFAULT_LATEX`).
- Rust: `snake_case` for local variables (e.g., `tex_path`, `pdf_buffer`, `pandoc_cmd`); `UPPER_SNAKE_CASE` for statics and constants.

**Types & Structs:**
- JavaScript: `PascalCase` for classes (e.g., `class BackendAdapter`).
- Rust: `PascalCase` for structs and enums (e.g., `struct CompileResponse`, `struct SyncTexResponse`).

## Code Style

**Formatting:**
- JavaScript: 4-space indentation, semicolons enforced, single quotes for strings in JS scripts, double quotes for JSON.
- Rust: Standard `rustfmt` formatting (4 spaces, trailing commas in multi-line lists, standard `Result<T, E>` idiomatic matching).
- CSS: Modern CSS custom properties (`--bg-primary`, `--accent-color`, `--text-muted`, `--border-color`) for consistent Apple HIG dark mode theming.

**Linting:**
- Rust compiler warnings enforced via `cargo check` and `clippy`.
- Minimalist dependency footprint in frontend (zero bundler overhead, pure vanilla ES6+ scripts directly executable in modern WebViews).

## Import Organization

**Order (Rust):**
1. Standard library imports (`std::fs`, `std::process::Command`, `std::path::PathBuf`)
2. External crates (`serde::Serialize`, `tinyfiledialogs`)
3. Internal module / crate declarations (`app_lib::run`)

**Order (JavaScript):**
1. Offline vendor scripts loaded in `<head>` (`codemirror.min.js`, `pdf.min.js`, `split.min.js`)
2. Custom abstraction adapters (`backend-adapter.js`)
3. Primary application controller (`main.js`)

**Path Aliases:**
- None. Relative paths and standard `/vendor/` routing used across `public/`.

## Error Handling

**Patterns (Rust):**
- System errors converted to clean strings using `.map_err(|e| e.to_string())` or `format!("Failed to spawn ...: {}", e)`.
- Non-fatal process failures gracefully captured in response structs (`CompileResponse { pdf_buffer: None, log: log_content, error: Some(...) }`) rather than panicking or crashing.
- File dialog cancellations cleanly return `Ok(false)` without error propagation.

**Patterns (JavaScript):**
- Async operations wrapped in `try { ... } catch (err) { ... }` blocks.
- User feedback delivered via status pills (`#status-pill`), alert notifications for critical OS errors, or dedicated in-app compile error dialogs (`#error-overlay`).

## Logging

**Framework:**
- Rust: `tauri-plugin-log` and `log` crate initialized in debug mode with `LevelFilter::Info`.
- JavaScript: Native `console.error` and `console.log` for debugging and IPC tracing.

**Patterns:**
- Errors during IPC commands log context before returning user-friendly messages.
- Silent fallbacks implemented where appropriate (e.g., SyncTeX offset scanning logs warnings instead of crashing).

## Comments

**When to Comment:**
- Crucial architectural constraints (e.g., macOS/Linux PATH augmentation reasons).
- High-DPI coordinate transformation formulas in SyncTeX event handlers.
- Auto-compilation debouncing rationale and state machine flows.

**JSDoc/TSDoc:**
- Concise doc comments on public functions and adapter methods (`BackendAdapter.invokeCompile`).

## Function Design

**Size:**
- Single responsibility functions kept under 50 lines where possible.
- Complex coordinate resolution factored into specialized helpers (`query_synctex_point`, `get_pandoc_path`).

**Parameters:**
- Explicit type declarations in Rust.
- Normalized parameter structures across IPC boundaries.

**Return Values:**
- Structured responses (`Result<CompileResponse, String>`) ensuring complete auditability of stdout and stderr.

## Module Design

**Exports:**
- JavaScript modules attach directly to global scope in browser environment (`window.BackendAdapter = BackendAdapter`) to remain compatible without a bundler.
- Rust functions marked with `#[tauri::command]` and exposed via `generate_handler!`.

**Barrel Files:**
- `src-tauri/src/lib.rs` acts as the primary library barrel for Tauri runtime integration.

---

*Convention analysis: 2026-10-25*
