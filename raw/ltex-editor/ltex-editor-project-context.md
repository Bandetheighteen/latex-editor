# LaTeX Editor Project Context

> Source: Local project repository (.planning/PROJECT.md and README.md)
> Collected: 2026-10-08
> Published: 2026-10-08

LaTeX Editor is a sleek, local-first LaTeX editor with an Apple HIG-inspired interface. LaTeX is mandatory for most PhD scholars and researchers. While tools like Overleaf are standard, they require a constant internet connection. Working offline with them is difficult and often frustrating. I built this app for fellow researchers who need a reliable, fully offline native LaTeX editor. I have no background in software development. This entire project was vibe-coded into existence.

A lightweight, crash-free IDE for LaTeX that provides a side-by-side view with LaTeX code on the left and a live PDF preview on the right. The application is distributed in two forms: a highly optimized native desktop app and a web-based version that can be opened in the browser.

Core Value: A stable, bug-free, and high-performance LaTeX editing experience that never shuts down unexpectedly and reliably renders PDFs side-by-side.

Current State (v1.0)
The application has successfully migrated to a dual-target architecture:
1. Tauri Native App: High-performance desktop application with native `pdflatex` compilation and SyncTeX support safely executing via Rust child processes.
2. Express Web Server: Robust Node.js backend serving the same frontend with HTTP endpoints for compile and SyncTeX, featuring strict timeout constraints to prevent zombie processes.
The buggy Electron architecture has been entirely replaced and obsolete files removed.

Requirements
- Basic side-by-side layout (editor left, PDF right)
- Integration with local LaTeX compiler (`pdflatex`)
- SyncTeX support (PDF to source mapping)
- Re-architect the native app using Tauri (Rust) for performance and stability
- Implement a stable Node.js backend for the web version
- Fix memory leaks and unexpected shutdowns (eliminated via timeouts & Tauri)
- Maintain a single frontend codebase (`public/`) shared across both Native and Web apps

Constraints
- Tech Stack: Native App: Tauri (Rust). Web Server: Node.js (Express). Frontend: HTML/JS/CSS.
- Dependency: Requires local LaTeX distribution (`pdflatex`, `synctex`) installed on the host system.

Key Decisions
- Use Tauri for Native App: Solves the memory and shutdown issues of Electron while remaining fast.
- Express Web Backend: Chosen for web target, adding strict 10s timeouts to prevent hanging.
- Shared Frontend: `public` folder is served by both Tauri and the Web backend via a `BackendAdapter`.
