# Ltex Editor

> Sources: Local project repository, 2026-10-24
> Raw: [ltex-editor-project-context](../../raw/ltex-editor/ltex-editor-project-context.md)
> Updated: 2026-10-24

## Overview

Ltex Editor is a local-first, offline LaTeX editor offering a clean, side-by-side interface with LaTeX source code on the left and live PDF preview on the right. It was created to provide a reliable, crash-free offline alternative to cloud-dependent services like Overleaf.

## Architecture

The application is built as a pure native desktop application:

- **Frontend:** CodeMirror for editing, PDF.js for offline document rendering, and Split.js for resizable panes.
- **Native Backend:** Built with Tauri v2 and Rust, managing `pdflatex` compilation, SyncTeX bidirectional jumps, and native Windows Save File dialogs via direct IPC.

The backend enforces strict 10-second timeouts on compilation child processes to prevent runaway processes and CPU drain.

## Core Features and Dependencies

- Side-by-side layout (editor on the left, PDF preview on the right).
- Bidirectional SyncTeX support mapping PDF locations directly back to the editor source line.
- Native Windows File Explorer integration for PDF export.
- Relies on a local TeX distribution (like MiKTeX or TeX Live) installed on the host system to provide `pdflatex` and `synctex` executables in the system PATH.
- Self-contained, offline-first design with zero cloud or network dependencies.

## Project Background

The creator of the project built the editor entirely through "vibe-coding" with no prior software development experience. It was built for PhD scholars and researchers who require LaTeX for publications but need a fast, crash-free, offline desktop tool.
