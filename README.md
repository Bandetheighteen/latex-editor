<div align="center">
  <img src="assets/banner.jpg" alt="LaTeX Editor Banner" width="800"/>
  
  # LaTeX Editor
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](LICENSE)
  [![Platform: Windows | macOS | Linux](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue.svg)](#)
  [![Release](https://img.shields.io/github/v/release/Bandetheighteen/ltex-editor?color=green)](https://github.com/Bandetheighteen/ltex-editor/releases)
</div>

LaTeX is mandatory for PhD scholars, academics, and researchers worldwide. While web tools like Overleaf are widely used, they require a persistent internet connection, and working offline with them can be frustrating and unreliable. 

This application was created for research co-fellows and students who need a high-performance, crash-free native LaTeX editor that operates **100% offline** on their desktop.

---

## 🚀 How to Install & Run

Running LaTeX on your computer requires a TeX engine to convert raw code into typesetting. Follow these two quick steps:

### Step 1: Install a TeX Distribution (Prerequisite)

Before launching LaTeX Editor, you must have a local TeX distribution installed on your computer so the editor can compile your `.tex` files offline.

- **Windows**: Install **[MiKTeX](https://miktex.org/download)** (recommended) or **[TeX Live](https://tug.org/texlive/)**.
  ```powershell
  winget install MiKTeX.MiKTeX
  ```
  *(After installing, ensure MiKTeX finishes its initial setup wizard so `pdflatex` is in PATH).*

- **macOS**: Install **[MacTeX](https://tug.org/mactex/)** via Homebrew:
  ```bash
  brew install --cask mactex-no-gui
  # Or lightweight BasicTeX:
  # brew install --cask basictex
  ```

- **Linux (Ubuntu / Debian / Fedora)**: Install TeX Live via your package manager:
  ```bash
  # Ubuntu / Debian / Pop!_OS:
  sudo apt install texlive-latex-base texlive-latex-extra texlive-binaries

  # Fedora / RHEL:
  sudo dnf install texlive-scheme-basic texlive-synctex
  ```

---

### Step 2: Download & Install LaTeX Editor

Once a TeX distribution is installed:

1. Go to the **[Releases](https://github.com/Bandetheighteen/ltex-editor/releases)** section of this GitHub repository.
2. Download the installer for your operating system:
   - **Windows**: `LaTeX-Editor-Native-Setup.exe`
   - **macOS**: `LaTeX-Editor.dmg` (mount DMG and drag to `/Applications`)
   - **Linux**: `.deb` package (`sudo dpkg -i ...`) or standalone `.AppImage` (`chmod +x` and run)
3. Launch **LaTeX Editor** directly from your Applications menu or Desktop!

---

### Alternative: Build from Source

If you prefer building and running the project from source:

```bash
git clone https://github.com/Bandetheighteen/ltex-editor.git
cd ltex-editor
npm install

# Download offline vendor and pandoc binaries
npm run setup

# Run the desktop app in development mode
npm run dev

# Package the release installer
npm run build
```

---

## ✨ Key Features

- **100% Offline Workflow:** Zero cloud dependence. Work seamlessly on airplanes, remote field sites, or anywhere without an internet connection.
- **Precision Bidirectional SyncTeX:** Single-click or double-click anywhere on the rendered PDF preview to jump instantly to that exact line in your LaTeX source code, complete with visual highlight pulsing.
- **Auto-Compilation Toggle:** Control your compilation flow with a header toggle switch (`ON` / `OFF`). Keep it off while drafting, and compile on demand (`Ctrl+S` or `Ctrl+Enter`) without unexpected interruptions.
- **Export to PDF & Microsoft Word (`.docx`):** Built-in unified export menu. Equations are automatically translated into native, fully editable Microsoft Word Equation Editor (OMML) objects using a bundled Pandoc engine.
- **Runaway Process Safeguard:** Strict 10-second compilation timeouts cleanly terminate stuck background loops without crashing the editor or freezing your CPU.
- **Native Windows Integration:** Uses native Windows File Explorer dialogs for saving files and exports.

---

## 🛠️ Architecture

- **Frontend:** CodeMirror 5 (with active line spotlighting and syntax highlighting), PDF.js (offline canvas rendering), and Split.js (smooth resizable dual-pane layout).
- **Backend:** Built with **Tauri v2** and **Rust**. Rust manages child process execution (`pdflatex`, `synctex`), memory safety, IPC communication, and native Windows Win32/COM file dialogs.
- **Converter:** Embedded, bundled standalone Pandoc binary for native `.docx` export without external dependencies.

---

## 👨‍💻 Author

This project was built and designed by **Shounak Saha**.

> *"I am a non-coder with no formal background in software engineering. I hardcoded and vibe-coded this entire application from the ground up to solve the real everyday frustrations my research peers and I faced while writing LaTeX papers offline.*
> 
> *I warmly welcome open-source developers, researchers, and LaTeX enthusiasts to collaborate, submit pull requests, open issues, and help improve this tool together!"*

- **GitHub:** [@Bandetheighteen](https://github.com/Bandetheighteen)
- **Portfolio / Connect:** [linktr.ee/shounaksaha2000b](https://linktr.ee/shounaksaha2000b)

---

## 📄 License

This project is licensed under the **[MIT License](LICENSE)** &copy; 2026 **Shounak Saha**.

You are free to use, modify, distribute, and collaborate on this project, provided that the original copyright and permission notice are preserved.
