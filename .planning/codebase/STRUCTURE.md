---
last_mapped_commit: 08970adae4941348382245362dc7c159533843bd
last_mapped_at: 2026-10-25
---
# Codebase Structure

**Analysis Date:** 2026-10-25

## Directory Layout

```
latex-editor/
├── .github/
│   └── workflows/
│       └── release.yml          # GitHub Actions multi-platform release matrix
├── assets/
│   ├── banner.jpg               # Official high-resolution README banner
│   └── native-icon.ico          # Windows application icon asset
├── public/                      # Frontend web assets embedded into Tauri WebView
│   ├── vendor/                  # Offline vendored JavaScript/CSS libraries
│   │   ├── codemirror/          # CodeMirror 5 editor, theme, and stex mode
│   │   ├── pdfjs/               # PDF.js library and worker
│   │   └── split/               # Split.js library
│   ├── backend-adapter.js       # Desktop Tauri IPC bridge
│   ├── favicon.ico              # Web view icon
│   ├── index.html               # Main desktop application HTML shell
│   ├── main.js                  # Core frontend application logic
│   └── style.css                # Apple HIG dark theme styling
├── raw/                         # Raw planning context and release chronicles
│   └── ltex-editor/
├── release/                     # Packaged output installers
│   └── LaTeX-Editor-Native-Setup.exe  # Standalone Windows NSIS installer
├── src-tauri/                   # Rust native backend and Tauri configuration
│   ├── binaries/                # Pre-packaged native binaries (Pandoc)
│   │   └── pandoc.exe           # Standalone Pandoc binary for Word export
│   ├── icons/                   # Multi-resolution application icons
│   ├── src/
│   │   ├── lib.rs               # Tauri command handlers, PATH logic, child processes
│   │   └── main.rs              # Rust entry point
│   ├── build.rs                 # Tauri build script
│   ├── Cargo.lock               # Cargo dependency lockfile
│   ├── Cargo.toml               # Rust package manifest and crate dependencies
│   └── tauri.conf.json          # Tauri application and bundler configuration
├── wiki/                        # Knowledge base archive
│   ├── index.md
│   ├── log.md
│   └── ltex-editor/
├── .gitignore                   # Git exclusion rules
├── LICENSE                      # MIT Open Source License (c) 2026 Shounak Saha
├── package-lock.json            # Node.js lockfile
├── package.json                 # npm scripts, metadata, and dev dependencies
├── README.md                    # Main project documentation, quickstart, and downloads
├── setup-pandoc.js              # Platform-aware standalone Pandoc setup utility
└── setup-vendors.js             # Offline vendor dependency downloader
```

## Directory Purposes

**`public/`:**
- Purpose: Contains all client-side assets served by the Tauri WebView.
- Contains: HTML shell, CSS stylesheets, client JavaScript, and offline vendor bundles.
- Key files: `public/index.html`, `public/main.js`, `public/backend-adapter.js`, `public/style.css`.

**`public/vendor/`:**
- Purpose: Completely offline local distribution of third-party frontend dependencies, ensuring 100% functionality without internet access.
- Contains: `codemirror/`, `pdfjs/`, `split/`.
- Key files: `public/vendor/codemirror/codemirror.min.js`, `public/vendor/pdfjs/pdf.min.js`.

**`src-tauri/`:**
- Purpose: Rust backend powering the native desktop application.
- Contains: Rust source files, Tauri configuration, native icon assets, and bundled offline CLI binaries.
- Key files: `src-tauri/src/lib.rs`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`.

**`src-tauri/binaries/`:**
- Purpose: Houses pre-compiled offline binaries that are packaged into the application distribution.
- Contains: `pandoc.exe` (Windows) and extracted Unix binaries during CI/CD.
- Key files: `src-tauri/binaries/pandoc.exe`.

**`.github/workflows/`:**
- Purpose: Continuous Integration and multi-platform packaging automation.
- Contains: GitHub Actions workflow specifications.
- Key files: `.github/workflows/release.yml`.

## Key File Locations

**Entry Points:**
- `src-tauri/src/main.rs`: Native desktop binary entry point.
- `src-tauri/src/lib.rs`: Rust application library entry point (`app_lib::run()`).
- `public/index.html`: Desktop webview DOM structure.
- `public/main.js`: Frontend application bootstrap and controller.

**Configuration:**
- `src-tauri/tauri.conf.json`: Tauri windowing, bundle identifiers, and target settings.
- `src-tauri/Cargo.toml`: Rust compiler edition and crate dependencies.
- `package.json`: NPM package metadata and lifecycle scripts.

**Core Logic:**
- `src-tauri/src/lib.rs`: Compiles LaTeX documents, queries SyncTeX positions, runs Pandoc DOCX conversions, and opens native file save dialogs.
- `public/backend-adapter.js`: Frontend abstraction layer dispatching Tauri commands.
- `public/main.js`: Split layout management, editor change debouncing, canvas rendering, zoom scaling, and SyncTeX click coordinate mapping.

**Testing:**
- Currently driven through manual verification and automated packaging via `.github/workflows/release.yml`.

## Naming Conventions

**Files:**
- JavaScript / Scripts: `kebab-case.js` (`setup-pandoc.js`, `backend-adapter.js`, `setup-vendors.js`)
- Rust source: `snake_case.rs` (`lib.rs`, `main.rs`, `build.rs`)
- CSS stylesheets: `kebab-case.css` (`style.css`)
- Documentation / Planning: `SCREAMING_SNAKE.md` or `kebab-case.md` (`README.md`, `STACK.md`, `ARCHITECTURE.md`)

**Directories:**
- Top-level: `kebab-case` (`src-tauri`, `public`, `assets`, `release`, `wiki`)
- Subdirectories: `lowercase` (`vendor`, `binaries`, `icons`, `workflows`)

## Where to Add New Code

**New Native Command / OS Feature:**
- Add command function with `#[tauri::command]` in `src-tauri/src/lib.rs`.
- Register the command in `tauri::generate_handler![...]` inside `src-tauri/src/lib.rs:215`.
- Expose the corresponding method in `BackendAdapter` inside `public/backend-adapter.js`.
- Wire the UI trigger and response handler in `public/main.js`.

**New UI Feature / Modal / Control:**
- Add HTML markup with appropriate semantic classes in `public/index.html`.
- Style the component in `public/style.css` adhering to Apple HIG dark theme guidelines.
- Bind event listeners and state management in `public/main.js`.

**New Export Format:**
- Add conversion logic or CLI invocation in `src-tauri/src/lib.rs`.
- Add dropdown item in `public/index.html` under `#export-dropdown-menu`.
- Bind click listener in `public/main.js` invoking the new backend adapter method.

## Special Directories

**`src-tauri/target/`:**
- Purpose: Rust compilation artifacts and intermediate object files.
- Generated: Yes (during `cargo check` or `tauri build`).
- Committed: No (ignored via `.gitignore`).

**`node_modules/`:**
- Purpose: Local Node.js development dependencies (`@tauri-apps/cli`).
- Generated: Yes (via `npm install`).
- Committed: No (ignored via `.gitignore`).

**`release/`:**
- Purpose: Contains final packaged setup executables.
- Generated: Yes (via `npm run build`).
- Committed: Yes (for local tracking and immediate deployment).

---

*Structure analysis: 2026-10-25*
