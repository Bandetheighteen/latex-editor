---
last_mapped_commit: 5b68d289dda9aa8130b04bde7f997b5e780b19de
last_mapped_at: 2026-10-25
---
# External Integrations

**Analysis Date:** 2026-10-25

## APIs & External Services

**Cloud APIs:**
- None. The application is strictly local-first and 100% offline. Zero remote network calls, tracking, analytics, or third-party web services are contacted at runtime (`public/backend-adapter.js`, `src-tauri/src/lib.rs`).

**Offline System Tooling:**
- **TeX Engine (`pdflatex`):**
  - Used for: Compiling `.tex` documents into `.pdf` typesetting and `.synctex.gz` coordinate tables.
  - Invocation: Native child process via `create_system_command("pdflatex")` (`src-tauri/src/lib.rs:46-53`).
  - Auth/Config: Discovered via system PATH and augmented search paths (`/Library/TeX/texbin`, Homebrew, `/usr/local/bin`).
- **SyncTeX Engine (`synctex`):**
  - Used for: Resolving mouse click coordinates on PDF pages to exact source line numbers in the `.tex` file.
  - Invocation: Native child process via `create_system_command("synctex")` (`src-tauri/src/lib.rs:78-83`).
- **Pandoc Binary (`pandoc` / `pandoc.exe`):**
  - Used for: Direct conversion of LaTeX syntax into Microsoft Word `.docx` documents containing native OMML equation objects.
  - Invocation: Bundled offline binary located via `get_pandoc_path()` (`src-tauri/src/lib.rs:148-174, 186-192`).
  - Distribution: Pre-bundled in `src-tauri/binaries/` via `setup-pandoc.js`.

## Data Storage

**Databases:**
- None. No relational or document database required.

**File Storage:**
- Local filesystem only:
  - Working directory: Temporary workspace directory managed via `std::env::temp_dir().join("latex-editor-workspace")` (`src-tauri/src/lib.rs:6-12`).
  - Browser Storage: HTML5 `localStorage` used for persisting active draft code (`latex_source_code`), and auto-compilation preference (`auto_compile_enabled`) (`public/main.js:56, 69, 99`).
  - User Export Destination: Selected interactively via native OS file dialogs powered by `tinyfiledialogs` (`src-tauri/src/lib.rs:135, 199`).

**Caching:**
- Local filesystem caching in `temp_dir/latex-editor-workspace/` stores intermediate LaTeX artifacts (`document.aux`, `document.log`, `document.synctex.gz`, `document.pdf`, `document.docx`) to accelerate subsequent compilations.

## Authentication & Identity

**Auth Provider:**
- None. Offline desktop application with zero authentication barriers or account management.

## Monitoring & Observability

**Error Tracking:**
- None (zero remote telemetry or telemetry SDKs).

**Logs:**
- Local logging via `tauri-plugin-log` configured for debug builds (`src-tauri/src/lib.rs:218-222`).
- Compilation stdout/stderr captured in Rust and delivered to frontend via `CompileResponse` (`src-tauri/src/lib.rs:32-37, 61-68`) and rendered in UI error overlays (`public/main.js:251-260`).

## CI/CD & Deployment

**Hosting:**
- GitHub Releases at `https://github.com/Bandetheighteen/ltex-editor/releases`.

**CI Pipeline:**
- GitHub Actions (`.github/workflows/release.yml`):
  - Triggers: Tag push (`v*`) or manual workflow dispatch.
  - Matrix: `macos-latest` (Apple Silicon `aarch64-apple-darwin`), `ubuntu-22.04` (Linux `x86_64`).
  - Outputs: `.dmg`, `.app.tar.gz`, `.deb`, `.AppImage`, `.rpm`.
  - Automated release publication with GitHub release token (`tauri-apps/tauri-action@v0`).

## Environment Configuration

**Required env vars:**
- None for end users.
- `GITHUB_TOKEN` used in GitHub Actions CI/CD to upload build artifacts to GitHub Releases (`.github/workflows/release.yml:71`).

**Secrets location:**
- No secrets stored in codebase. All binary assets and vendors are open source or downloaded via public setup scripts (`setup-pandoc.js`, `setup-vendors.js`).

## Webhooks & Callbacks

**Incoming:**
- None.

**Outgoing:**
- None.

---

*Integration audit: 2026-10-25*
