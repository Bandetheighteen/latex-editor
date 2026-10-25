---
last_mapped_commit: 08970adae4941348382245362dc7c159533843bd
last_mapped_at: 2026-10-25
---
# Technology Stack

**Analysis Date:** 2026-10-25

## Languages

**Primary:**
- Rust 2024 edition (rust-version 1.90) - Native desktop backend, process orchestration, filesystem I/O, IPC commands (`src-tauri/src/lib.rs`, `src-tauri/src/main.rs`)
- JavaScript (ES6+ / Vanilla) - Frontend UI orchestration, CodeMirror editor binding, PDF.js canvas rendering, IPC adapter (`public/main.js`, `public/backend-adapter.js`, `setup-pandoc.js`, `setup-vendors.js`)

**Secondary:**
- HTML5 & CSS3 - Modern Apple HIG-inspired dual-pane interface, dark theme styling, glassmorphism (`public/index.html`, `public/style.css`)
- LaTeX (TeX) - Document typesetting input and templates (`public/main.js`)

## Runtime

**Environment:**
- Tauri v2 (v2.12.1) - Native desktop application runtime bridging web frontend with Rust system backend (`src-tauri/tauri.conf.json`)
- Node.js (v18+ / v22 in CI) - Build tooling and offline dependency provisioning (`package.json`, `.github/workflows/release.yml`)

**Package Manager:**
- Cargo (Rust toolchain) - Backend dependency manager (`src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`)
  - Lockfile: present (`src-tauri/Cargo.lock`)
- npm (Node.js) - Frontend build dependencies and scripts (`package.json`, `package-lock.json`)
  - Lockfile: present (`package-lock.json`)

## Frameworks

**Core:**
- Tauri v2 (`@tauri-apps/api` ^2.12.1, `tauri` 2.12.1) - Native desktop windowing, OS integration, and IPC (`package.json`, `src-tauri/Cargo.toml`)
- CodeMirror 5 (v5.65.16) - In-browser code editor with `stex` syntax mode, active line spotlight, and line wrapping (`public/vendor/codemirror/`)
- PDF.js (v3.11.174) - Offline PDF document rendering onto HTML5 canvas (`public/vendor/pdfjs/`)
- Split.js (v1.6.5) - Resizable split panes for dual-pane editing layout (`public/vendor/split/`)

**Testing:**
- None formal currently configured (manual verification and CI release builds in `.github/workflows/release.yml`)

**Build/Dev:**
- `@tauri-apps/cli` (^2.12.1) - Desktop bundling and cross-platform compilation CLI (`package.json`)
- `tauri-build` (2.7.1) - Rust build script dependency (`src-tauri/Cargo.toml`)

## Key Dependencies

**Critical:**
- `tinyfiledialogs` (3.9.1) - Native cross-platform Win32/macOS/GTK save file dialogs for PDF and Word DOCX exports (`src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`)
- `tempfile` (3.x) - Workspace management and secure temporary file generation (`src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`)
- `serde` / `serde_json` (1.0) - Rust serialization for Tauri IPC structs (`src-tauri/Cargo.toml`)
- `tauri-plugin-log` (2.x) - Native logging and diagnostic output (`src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`)
- Pandoc (v3.12.1 standalone) - Offline binary engine for converting LaTeX into Microsoft Word (`.docx`) with OMML formulas (`src-tauri/binaries/pandoc.exe`, `setup-pandoc.js`)

**Infrastructure:**
- `pdflatex` (MiKTeX / TeX Live) - System-level compiler invoked via child processes (`src-tauri/src/lib.rs`)
- `synctex` - System-level CLI for bidirectional position mapping between PDF coordinates and source lines (`src-tauri/src/lib.rs`)

## Configuration

**Environment:**
- Desktop native application; zero runtime environment variables or external API keys required.
- Dynamic PATH augmentation in Rust backend (`src-tauri/src/lib.rs`) to detect TeX binaries on macOS (`/Library/TeX/texbin`, `/opt/homebrew/bin`, `/usr/local/bin`) and Linux (`/usr/local/bin`, `/usr/bin`).

**Build:**
- `src-tauri/tauri.conf.json` - Bundle metadata, window sizes (1200x800), CSP settings, bundled resources (`binaries/*`)
- `package.json` - Setup commands (`npm run setup`, `npm run dev`, `npm run build`)
- `setup-pandoc.js` - Automated architecture-aware download of standalone Pandoc binaries

## Platform Requirements

**Development:**
- Windows 10/11, macOS 12+, or Ubuntu 20.04+
- Rust toolchain (`rustup`, `cargo`)
- Node.js (v18+)
- Local TeX distribution (MiKTeX on Windows, MacTeX/BasicTeX on macOS, TeX Live on Linux)

**Production:**
- Windows: Standalone NSIS installer (`LaTeX-Editor-Native-Setup.exe`)
- macOS: Disk Image (`.dmg`) and App archive (`.app.tar.gz`) for Apple Silicon (`aarch64`)
- Linux: Debian package (`.deb`), Universal AppImage (`.AppImage`), and Red Hat package (`.rpm`) for `x86_64` / `amd64`

---

*Stack analysis: 2026-10-25*
