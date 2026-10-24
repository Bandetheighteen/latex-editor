<div align="center">
  <img src="assets/banner.jpg" alt="LaTeX Editor Banner" width="800"/>
  
  # LaTeX Editor
</div>

LaTeX is mandatory for most PhD scholars and researchers. While tools like Overleaf are standard, they require a constant internet connection. Working offline with them is difficult and often frustrating. I built this app for fellow researchers who need a reliable, fully offline native LaTeX editor.

I have no background in software development. This entire project was vibe-coded into existence.

## What is in the repository

This repository contains the standalone native desktop app.

### The Native Desktop App
A lightweight, high-performance desktop application built with Tauri v2 and Rust. It handles memory safely, eliminates the crashes common in heavy editors, and runs completely offline on your computer.

## Installation

### Direct Download
1. Head over to the [`release/`](release/) folder in this repository.
2. Download and run **`LaTeX-Editor-Native-Setup.exe`**.
3. Follow the setup wizard to install the app on your computer.

*Note: You need a local TeX distribution (such as MiKTeX or TeX Live) installed on your system so the editor can compile your LaTeX documents.*

### Build from Source
If you prefer to build the app from source:

```bash
git clone https://github.com/Bandetheighteen/ltex-editor.git
cd ltex-editor
npm install

# Run the native app in development mode
npm run dev

# Or package the installer
npm run build
```

## How It Works

- **Offline PDF Preview:** Uses an embedded, offline PDF.js engine for instant rendering without web requests.
- **Bidirectional SyncTeX:** Double-click anywhere on the rendered PDF preview, and the editor jumps straight to that exact line in your LaTeX source.
- **Runaway Process Safeguard:** Enforces a 10-second compilation timeout. If code causes an infinite LaTeX loop, the background `pdflatex` process is cleanly terminated to protect your CPU.
- **Export to PDF & Word (.docx):** A unified export dropdown menu allows you to export to PDF or Microsoft Word (`.docx`). Equations are automatically converted into native, fully editable Word equations (OMML) powered by a bundled Pandoc engine.
- **Native File Dialogs:** Seamlessly exports files directly using native Windows file dialogs.

## How It Was Built

- **Frontend:** CodeMirror for the code editor, Split.js for adjustable split panes, and PDF.js for canvas rendering, packaged with offline assets.
- **Backend:** Tauri v2 and Rust handle local child processes (`pdflatex`, `synctex`), memory safety, IPC communication, and file system operations.
