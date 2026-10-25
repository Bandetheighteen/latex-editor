---
last_mapped_commit: 5b68d289dda9aa8130b04bde7f997b5e780b19de
last_mapped_at: 2026-10-25
---
# Codebase Concerns

**Analysis Date:** 2026-10-25

## Tech Debt

**Legacy Electron Detection:**
- Issue: `public/main.js:3-5` contains `if (window.api) { document.body.classList.add('is-electron'); }`, an obsolete remnant from the pre-Tauri Electron architecture.
- Files: `public/main.js`
- Impact: Harmless dead branch, but causes minor confusion regarding runtime identity.
- Fix approach: Remove the legacy check and rely solely on `BackendAdapter.isTauri`.

**Temporary Directory Cleanup & Workspace Isolation:**
- Issue: `get_workspace_dir()` creates a single static `latex-editor-workspace` in the OS temp directory. Compilation artifacts (`document.tex`, `document.pdf`, `document.log`, `document.docx`, `document.synctex.gz`) are overwritten in place and never cleaned up on application exit.
- Files: `src-tauri/src/lib.rs:6-12`
- Impact: Leaves intermediate TeX artifacts in OS temp directory; would cause race conditions if multiple windows or concurrent compilations occur.
- Fix approach: Introduce unique session-scoped workspace directories with cleanup hooks on application exit.

**Hardcoded Starter Template:**
- Issue: Default LaTeX document is embedded as a raw multi-line template string in `public/main.js:16-53`.
- Files: `public/main.js`
- Impact: Makes updating default templates or offering template libraries difficult.
- Fix approach: Externalize starter templates into modular JSON or TeX asset files.

## Known Bugs

**Non-Virtualized Multi-Page Canvas Rendering:**
- Symptoms: `renderPdf` loops through all pages and appends canvas elements for every single page into `#pdf-viewer` without virtualization or lazy rendering (`public/main.js:165-224`).
- Files: `public/main.js:165-224`
- Trigger: Compiling large LaTeX documents with many pages (e.g., 30+ pages).
- Impact: Substantial browser memory consumption and UI jank during zoom or re-compilation of long manuscripts.
- Workaround: The full PDF is generated and can be saved via "Export PDF"; future iteration should implement a virtualized canvas list or PDF.js continuous viewer component.

**Mismatched Argument in PDF Save Handler:**
- Symptoms: `public/main.js:289` calls `await BackendAdapter.invokeSavePdf(currentPdfUrl);`, but `BackendAdapter.invokeSavePdf()` in `public/backend-adapter.js:32-44` takes no parameters and delegates to Rust `save_pdf`, which reads directly from `dir.join("document.pdf")`.
- Files: `public/main.js:289`, `public/backend-adapter.js:32`
- Impact: Unused argument passed; harmless today but creates architectural confusion if `invokeSavePdf` signature changes.
- Fix approach: Align signatures or remove the redundant argument in `main.js`.

## Security Considerations

**LaTeX Shell Escape Protection:**
- Risk: Malicious or compromised LaTeX packages using `\write18` or `\immediate\write18` to execute arbitrary host commands during compilation.
- Files: `src-tauri/src/lib.rs:46-53`
- Current mitigation: Default `pdflatex` compilation runs in restricted mode.
- Recommendations: Explicitly pass `-no-shell-escape` to `create_system_command("pdflatex")` in `src-tauri/src/lib.rs` for guaranteed defense-in-depth against malicious LaTeX code.

**CSP Configuration:**
- Risk: `src-tauri/tauri.conf.json` defines `"csp": null`.
- Files: `src-tauri/tauri.conf.json:21`
- Current mitigation: Frontend relies entirely on local offline assets (`public/`).
- Recommendations: Define a strict Content Security Policy (`default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'`) to prevent unexpected inline injection risks.

**Direct Command Execution Safety:**
- Risk: Potential command injection if shell interpreters are invoked.
- Files: `src-tauri/src/lib.rs:14-30, 46, 78, 186`
- Current mitigation: Rust's `std::process::Command` executes target binaries directly without invoking a shell (`sh`, `bash`, `cmd.exe`). Arguments are passed as individual array items, preventing argument and command injection.

## Performance Bottlenecks

**Large Binary IPC Transport:**
- Problem: `compile_latex` reads the entire compiled PDF into memory as a `Vec<u8>` (`src-tauri/src/lib.rs:58`) and transmits it across Tauri IPC to JavaScript.
- Files: `src-tauri/src/lib.rs:34, 58`, `public/backend-adapter.js:13`
- Cause: Byte buffer serialization for multi-megabyte PDFs adds IPC serialization latency and memory duplication.
- Improvement path: Serve the generated PDF via Tauri's asset protocol (`asset://localhost/...`) or custom streaming protocol for zero-copy loading in PDF.js.

**Sequential Canvas Page Rendering:**
- Problem: Each page is rendered sequentially with `await page.render(renderContext).promise` in a blocking loop (`public/main.js:178`).
- Files: `public/main.js:165-224`
- Improvement path: Render visible pages first, then lazily queue off-screen pages using an IntersectionObserver.

## Fragile Areas

**External TeX Distribution Requirement:**
- Files: `src-tauri/src/lib.rs:14-30`, `README.md`
- Why fragile: If the user does not have MiKTeX, MacTeX, or TeX Live installed, `pdflatex` invocation fails with an OS process spawn error ("Failed to spawn pdflatex").
- Safe modification: Catch executable not found errors in Rust and return a clear, localized guidance message directing the user to install MiKTeX (Windows), MacTeX (macOS), or TeX Live (Linux).
- Test coverage: Needs an automated check to verify whether `pdflatex` exists on the host system before attempting compilation.

**macOS GUI PATH Invariance:**
- Files: `src-tauri/src/lib.rs:17-22`
- Why fragile: macOS GUI applications launched from Finder or Spotlight do not inherit user shell profiles (`.zshrc`, `.bash_profile`).
- Current mitigation: `create_system_command` explicitly injects `/Library/TeX/texbin`, `/opt/homebrew/bin`, and `/usr/local/bin` into the PATH environment for macOS.
- Safe modification: Ensure any new TeX distribution installation paths are added to this fallback list.

## Scaling Limits

**Non-Virtualized Canvas Scaling:**
- Current capacity: Efficient for 1–15 page documents.
- Limit: Documents exceeding 30–50 pages will consume heavy GPU/browser canvas memory.
- Scaling path: Introduce virtualized canvas scrolling using PDF.js `PDFViewer` with text layers.

**Single Document Workspace:**
- Current capacity: Single active document (`document.tex`).
- Limit: Multi-file LaTeX projects (`\include`, `\input`, custom images or bib files) require all files to reside in the same workspace directory.
- Scaling path: Support opening folders/projects and compiling project root documents with relative path preservation.

## Dependencies at Risk

**Vendored Client Libraries in `public/vendor/`:**
- Risk: CodeMirror 5, PDF.js, and Split.js are committed as static bundles in `public/vendor/`.
- Impact: CodeMirror 5 is in maintenance mode (CodeMirror 6 is the modern rewrite).
- Migration plan: Transition to CodeMirror 6 or Monaco Editor in a future major milestone for tree-shaking, richer LSP, and modern snippet expansion.

## Missing Critical Features

**Forward SyncTeX (Code to PDF Navigation):**
- Problem: SyncTeX currently operates exclusively in reverse (click PDF &rarr; jump to source code line). Forward SyncTeX (cursor in code &rarr; scroll to and highlight corresponding position in PDF preview) is not yet implemented.

**Text Selection and Copy from PDF Preview:**
- Problem: PDF is rendered purely to HTML5 `<canvas>` elements without a PDF.js text layer overlay.
- Impact: Users cannot highlight, select, or copy text directly from the PDF preview pane.

**Project/Folder Tree Support:**
- Problem: Editor currently operates on a single text buffer stored in `localStorage`. Multi-file projects with external figures (`\includegraphics`) or bibliography files (`.bib`) cannot easily be loaded.

## Test Coverage Gaps

**Automated Frontend Unit / E2E Tests:**
- What's not tested: Split pane resizing, zoom stepping math, SyncTeX click calculations, and auto-compile state transitions are currently verified manually.
- Files: `public/main.js`, `public/backend-adapter.js`
- Risk: Regressions during frontend refactoring could go unnoticed prior to manual testing.
- Priority: Medium

---

*Concerns audit: 2026-10-25*
