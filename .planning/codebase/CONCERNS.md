---
last_mapped_commit: 08970adae4941348382245362dc7c159533843bd
last_mapped_at: 2026-10-25
---
# Codebase Concerns

**Analysis Date:** 2026-10-25

## Tech Debt

**Legacy Electron Detection:**
- Issue: `public/main.js:3` contains `if (window.api) { document.body.classList.add('is-electron'); }`, an obsolete remnant from the pre-Tauri Electron architecture.
- Files: `public/main.js`
- Impact: Harmless dead branch, but causes minor confusion regarding runtime identity.
- Fix approach: Remove the legacy check and rely solely on `BackendAdapter.isTauri`.

**Temporary Directory Cleanup:**
- Issue: `get_workspace_dir()` creates `latex-editor-workspace` in the OS temp directory, but compilation files (`document.tex`, `document.pdf`, `document.log`, `document.docx`) are overwritten in place and never explicitly cleaned up on application exit.
- Files: `src-tauri/src/lib.rs:6-12`
- Impact: Leaves small text and PDF files in OS temp folder (typically cleared automatically on OS reboot).
- Fix approach: Implement a clean exit hook or scoped temporary subdirectories with cleanup on drop.

## Known Bugs

**Single Page Rendering Limitation in UI:**
- Symptoms: Only the first page of multi-page PDF documents is currently rendered onto the preview canvas.
- Files: `public/main.js:180-205`
- Trigger: Compiling documents with more than one page of content.
- Workaround: The full PDF is always generated and can be saved via "Export as PDF"; future UI should implement a scrollable multi-page canvas or continuous scroll viewer.

## Security Considerations

**Command Spawning & Shell Safety:**
- Risk: Potential command injection if arbitrary text were passed to shell interpreters.
- Files: `src-tauri/src/lib.rs:46, 78, 186`
- Current mitigation: Rust's `std::process::Command` executes binaries directly (`pdflatex`, `synctex`, `pandoc`) without invoking a system shell (`sh`, `bash`, `cmd.exe`). Arguments are passed as individual array items, preventing argument injection.
- Recommendations: Maintain strict direct process invocation; never wrap commands in `sh -c` or `cmd /C`.

**LaTeX Shell Escape Disabled:**
- Risk: Malicious LaTeX documents using `\write18` or `\immediate\write18` to execute arbitrary host commands.
- Files: `src-tauri/src/lib.rs:46-53`
- Current mitigation: Default `pdflatex` compilation runs with restricted/disabled shell escape (no `-shell-escape` flag is supplied).
- Recommendations: Explicitly pass `-no-shell-escape` in `src-tauri/src/lib.rs` for defense-in-depth against malicious packages.

## Performance Bottlenecks

**Frequent LaTeX Compilations:**
- Problem: TeX engines (`pdflatex`) are file-IO and CPU intensive compared to modern compilers.
- Files: `src-tauri/src/lib.rs:40-70`, `public/main.js:98-102`
- Cause: Compiling on every keystroke causes stutter and high CPU load.
- Current mitigation: Auto-compilation is disabled by default (toggled to `OFF`). When enabled, it is debounced with a 1200ms delay.
- Improvement path: Persist intermediate precompiled formats or explore WebAssembly TeX engines for instant background previews if needed.

## Fragile Areas

**External TeX Distribution Requirement:**
- Files: `src-tauri/src/lib.rs:14-30`, `README.md`
- Why fragile: If the user does not have MiKTeX, MacTeX, or TeX Live installed, `pdflatex` invocation fails with an OS process spawn error ("executable not found").
- Safe modification: Catch executable not found errors in Rust and return a clear, localized guidance message directing the user to install MiKTeX or TeX Live.
- Test coverage: Needs an automated check to verify whether `pdflatex` exists on the host system before attempting compilation.

**macOS GUI PATH Invariance:**
- Files: `src-tauri/src/lib.rs:17-22`
- Why fragile: macOS GUI applications launched from Finder or Spotlight do not inherit user shell profiles (`.zshrc`, `.bash_profile`).
- Current mitigation: `create_system_command` explicitly injects `/Library/TeX/texbin`, `/opt/homebrew/bin`, and `/usr/local/bin` into the PATH environment for macOS.
- Safe modification: Ensure any new TeX distribution installation paths are added to this fallback list.

## Scaling Limits

**Document Size & Canvas Viewport:**
- Current capacity: Single page canvas rendering at 60%–300% zoom.
- Limit: Documents exceeding 10–20 pages are best reviewed by exporting to dedicated external PDF readers until multi-page canvas virtualization is added.
- Scaling path: Introduce virtualized canvas scrolling using PDF.js `PDFViewer` with text layers.

## Dependencies at Risk

**Vendored Client Libraries in `public/vendor/`:**
- Risk: CodeMirror 5, PDF.js, and Split.js are committed as static bundles in `public/vendor/`.
- Impact: CodeMirror 5 is in maintenance mode (CodeMirror 6 is the modern rewrite).
- Migration plan: Transition to CodeMirror 6 or Monaco Editor in a future major milestone for tree-shaking and richer LSP integration.

## Missing Critical Features

**Multi-Page PDF Viewer:**
- Problem: Canvas renders page 1 only.
- Blocks: Viewing multi-page academic papers directly within the split preview without external viewer export.

**Continuous Reverse SyncTeX (Code to PDF):**
- Problem: SyncTeX currently operates from PDF preview to source code (click PDF &rarr; jump to code line). Forward SyncTeX (cursor in code &rarr; highlight in PDF) is not yet wired.

## Test Coverage Gaps

**Automated Frontend Unit / E2E Tests:**
- What's not tested: Split pane resizing, zoom stepping math, and auto-compile state transitions are currently verified manually.
- Files: `public/main.js`, `public/backend-adapter.js`
- Risk: Regressions during frontend refactoring could go unnoticed prior to manual testing.
- Priority: Medium

---

*Concerns audit: 2026-10-25*
