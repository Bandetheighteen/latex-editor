---
last_mapped_commit: 5b68d289dda9aa8130b04bde7f997b5e780b19de
last_mapped_at: 2026-10-25
---
# Testing Patterns

**Analysis Date:** 2026-10-25

## Test Framework

**Runner:**
- Rust: `cargo test` (standard Rust built-in test runner).
- Frontend: Manual UAT and exploratory testing currently; zero external JS test runners (Jest/Vitest) configured to avoid bloat and complex build steps.

**Assertion Library:**
- Rust standard library assertions (`assert!`, `assert_eq!`).

**Run Commands:**

```bash
cargo check --manifest-path src-tauri/Cargo.toml   # Verify Rust backend compilation and typing
npm run dev                                        # Launch desktop app for interactive verification
npm run build                                      # Compile complete release bundle with all targets
```

## Test File Organization

**Location:**
- Co-located unit tests in Rust source files using `#[cfg(test)] mod tests` pattern where applicable.
- Integration verification occurs against live binaries via CI workflow `.github/workflows/release.yml`.

**Naming:**
- Standard Rust naming conventions (`*_test.rs` or inline `tests` modules).

## Test Structure

**Suite Organization (Rust backend):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandoc_discovery() {
        let path = get_pandoc_path();
        assert!(!path.as_os_str().is_empty());
    }
}
```

**Manual UAT Verification Protocol:**
1. **Compilation Check:** Enter valid LaTeX syntax, press `Ctrl+S` / `Cmd+S` / `Ctrl+Enter`, verify status pill displays "Ready" / "Compiled Successfully" and PDF canvas renders cleanly.
2. **Error Boundary Check:** Introduce intentional syntax error (`\undefinedcommand`), press `Compile`, verify status pill displays "Compilation Error" with log output visible in `#error-overlay`.
3. **SyncTeX Navigation Check:** Click or double-click on rendered PDF preview text, confirm editor smoothly scrolls to the exact source line and flashes a luminous highlight.
4. **Auto-Compile Toggle:** Toggle button between `OFF` and `ON`. In `OFF`, verify typing does not trigger compilation; in `ON`, verify compilation debounces after typing stops (1200ms delay).
5. **Zoom Controls:** Click zoom in (`+`), zoom out (`-`), and verify scaling between 60% and 300% renders crisply at all DPI levels.
6. **Export Verification:**
   - Click Export &rarr; "Export PDF (.pdf)", verify native save dialog opens and saves valid PDF.
   - Click Export &rarr; "Export Word (.docx)", verify native save dialog opens, saves valid `.docx`, and equations open in Word as native OMML objects.

## Mocking

**Framework:**
- None. System interactions directly test real binary outputs and OS filesystem paths.

**What to Mock (Future Unit Tests):**
- System command spawns (`pdflatex`, `synctex`, `pandoc`) via trait abstractions if unit testing Rust command dispatch in isolation.

**What NOT to Mock:**
- File writing and temporary directory operations (`tempfile`).
- IPC serialization contracts between JavaScript and Rust.

## Fixtures and Factories

**Test Data:**
- Starter document template in `public/main.js` (`DEFAULT_LATEX`) serves as standard test document containing math equations, Euler's identity, itemized lists, and section headers.

**Location:**
- Embedded directly in `public/main.js:16-53`.

## Coverage

**Requirements:**
- None currently enforced via coverage tools. Priority is placed on zero-crash runtime guarantees and build-time type verification.

**View Coverage:**
- Not applicable.

## Test Types

**Unit Tests:**
- Rust compiler and type checker gate backend safety (`cargo check`).

**Integration Tests:**
- GitHub Actions CI/CD matrix (`.github/workflows/release.yml`) compiles and packages real release targets on macOS (`aarch64-apple-darwin`) and Ubuntu 22.04 (`x86_64`).

**E2E Tests:**
- Manual interactive verification protocol on target platforms.

## Common Patterns

**Async Command Verification:**

```javascript
// Test backend adapter response
const result = await BackendAdapter.invokeCompile(testCode);
console.assert(!result.error, 'Compilation should succeed without error');
console.assert(result.pdfBuffer instanceof ArrayBuffer, 'Should return valid PDF buffer');
```

**Error Handling Verification:**

```rust
// Verify failure does not panic backend
let res = compile_latex(String::from("\\invalid{syntax"));
assert!(res.is_ok()); // Returns Ok(CompileResponse) with error field populated
```

---

*Testing analysis: 2026-10-25*
