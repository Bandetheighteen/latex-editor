document.addEventListener('DOMContentLoaded', () => {
    // Detect Electron and add body class for platform-specific CSS
    if (window.api) {
        document.body.classList.add('is-electron');
    }

    // 1. Initialize Split.js Panes
    Split(['#editor-pane', '#preview-pane'], {
        sizes: [48, 52],
        minSize: [300, 350],
        gutterSize: 6,
        cursor: 'col-resize'
    });

    // 2. Default LaTeX Starter Code (Loaded if no saved draft exists)
    const DEFAULT_LATEX = `\\documentclass[11pt,a4paper]{article}
\\usepackage[left=0.8in,right=0.8in,top=0.8in,bottom=0.8in]{geometry}
\\usepackage{amsmath,amsfonts,amssymb}
\\usepackage{hyperref}
\\usepackage{xcolor}

\\definecolor{accentblue}{RGB}{37, 99, 235}

\\title{\\textbf{\\Huge Sample Research Document}}
\\author{\\Large Author Name}
\\date{\\today}

\\begin{document}
\\maketitle

\\section{Introduction}
Welcome to your new \\textbf{Native LaTeX Editor}! This application runs completely offline on your computer with instantaneous compilation and zero crashes.

\\section{Mathematical Formulation}
LaTeX renders beautiful mathematical formulas effortlessly:
\\begin{equation}
    i\\hbar \\frac{\\partial}{\\partial t} \\Psi(\\mathbf{r}, t) = \\hat{\\mathcal{H}} \\Psi(\\mathbf{r}, t)
\\end{equation}

And Euler's identity:
\\begin{equation}
    e^{i\\pi} + 1 = 0
\\end{equation}

\\section{Features}
\\begin{itemize}
    \\item \\textbf{SyncTeX Jump:} Double-click on any text in the PDF preview to jump directly to that line in your code.
    \\item \\textbf{White Spotlight:} Notice how the active line in your editor is highlighted with a focused horizontal beam.
    \\item \\textbf{Zoom Stepper:} Use the zoom buttons at the top to scale the preview crisply.
\\end{itemize}

\\end{document}
`;

    // 3. Initialize CodeMirror Editor
    const savedDraft = localStorage.getItem('latex_source_code') || DEFAULT_LATEX;
    const editor = CodeMirror.fromTextArea(document.getElementById('latex-code'), {
        lineNumbers: true,
        mode: 'stex',
        theme: 'monokai',
        lineWrapping: true,
        matchBrackets: true,
        styleActiveLine: true,
        value: savedDraft
    });
    editor.setValue(savedDraft);

    // Auto-Compile State (Default to OFF per user preference)
    let isAutoCompile = localStorage.getItem('auto_compile_enabled') === 'true';
    const autoCompileToggle = document.getElementById('auto-compile-toggle');
    const autoCompileLabel = document.getElementById('auto-compile-label');

    function updateAutoCompileUI() {
        if (!autoCompileToggle || !autoCompileLabel) return;
        if (isAutoCompile) {
            autoCompileToggle.classList.add('is-active');
            autoCompileLabel.textContent = 'ON';
        } else {
            autoCompileToggle.classList.remove('is-active');
            autoCompileLabel.textContent = 'OFF';
        }
    }
    updateAutoCompileUI();

    if (autoCompileToggle) {
        autoCompileToggle.addEventListener('click', () => {
            isAutoCompile = !isAutoCompile;
            localStorage.setItem('auto_compile_enabled', isAutoCompile.toString());
            updateAutoCompileUI();
            if (isAutoCompile) {
                compileLatex();
            }
        });
    }

    // Save draft automatically on typing; compile ONLY if auto-compile is toggled ON
    let compileTimeout;
    editor.on('change', () => {
        localStorage.setItem('latex_source_code', editor.getValue());
        setStatus('Uncompiled Changes', 'status-ready');
        
        clearTimeout(compileTimeout);
        if (isAutoCompile) {
            compileTimeout = setTimeout(() => {
                compileLatex();
            }, 1200);
        }
    });

    // 4. Configure PDF.js (Offline Local Worker)
    pdfjsLib.GlobalWorkerOptions.workerSrc = '/vendor/pdfjs/pdf.worker.min.js';

    // State Variables
    let currentPdfUrl = null;
    let pdfDoc = null;
    let currentScale = 1.35; // Default crisp reading scale

    // DOM Elements
    const statusPill = document.getElementById('status-pill');
    const zoomLevelEl = document.getElementById('zoom-level');
    const pdfViewer = document.getElementById('pdf-viewer');
    const errorOverlay = document.getElementById('error-overlay');
    const errorLog = document.getElementById('error-log');
    const errorDismissBtn = document.getElementById('error-dismiss-btn');

    function setStatus(text, className) {
        statusPill.textContent = text;
        statusPill.className = `status-pill ${className}`;
    }

    function updateZoomDisplay() {
        zoomLevelEl.textContent = `${Math.round(currentScale * 100)}%`;
    }
    updateZoomDisplay();

    // 5. Zoom Stepper Controls
    document.getElementById('zoom-in-btn').addEventListener('click', async () => {
        if (currentScale < 3.0) {
            currentScale = parseFloat((currentScale + 0.15).toFixed(2));
            updateZoomDisplay();
            if (currentPdfUrl) await renderPdf(currentPdfUrl);
        }
    });

    document.getElementById('zoom-out-btn').addEventListener('click', async () => {
        if (currentScale > 0.6) {
            currentScale = parseFloat((currentScale - 0.15).toFixed(2));
            updateZoomDisplay();
            if (currentPdfUrl) await renderPdf(currentPdfUrl);
        }
    });

    // 6. Dismiss Error Overlay
    errorDismissBtn.addEventListener('click', () => {
        errorOverlay.classList.add('hidden');
    });

    // 7. Render PDF Document
    async function renderPdf(pdfUrl) {
        if (!pdfUrl) return;
        try {
            pdfDoc = await pdfjsLib.getDocument(pdfUrl).promise;
            pdfViewer.innerHTML = '';

            for (let pageNum = 1; pageNum <= pdfDoc.numPages; pageNum++) {
                const page = await pdfDoc.getPage(pageNum);
                const viewport = page.getViewport({ scale: currentScale });

                const canvas = document.createElement('canvas');
                canvas.className = 'pdf-canvas';
                canvas.height = viewport.height;
                canvas.width = viewport.width;

                const renderContext = {
                    canvasContext: canvas.getContext('2d'),
                    viewport: viewport
                };
                await page.render(renderContext).promise;

                // SyncTeX: Click or Double-click on PDF to jump accurately to code
                const triggerSyncTexJump = async (e) => {
                    const rect = canvas.getBoundingClientRect();
                    // Account for any DPI scaling or CSS sizing differences
                    const scaleX = canvas.width / rect.width;
                    const scaleY = canvas.height / rect.height;
                    const canvasX = (e.clientX - rect.left) * scaleX;
                    const canvasY = (e.clientY - rect.top) * scaleY;

                    // Convert to 72 dpi PDF points from top-left corner
                    const pdfX = canvasX / currentScale;
                    const pdfY = canvasY / currentScale;

                    try {
                        const data = await window.BackendAdapter.invokeSyncTeX(pageNum, pdfX, pdfY);

                        if (data && data.line) {
                            const targetLine = data.line - 1;
                            
                            // Highlight the line selection
                            const lineLen = editor.getLine(targetLine) ? editor.getLine(targetLine).length : 0;
                            editor.setSelection({ line: targetLine, ch: 0 }, { line: targetLine, ch: lineLen });
                            
                            // Center the line smoothly in the editor viewport
                            const h = editor.getScrollInfo().clientHeight;
                            const coords = editor.charCoords({ line: targetLine, ch: 0 }, "local");
                            editor.scrollTo(null, Math.max(0, coords.top - h / 2));
                            editor.focus();

                            // Pulse visual highlight flash
                            const lineHandle = editor.addLineClass(targetLine, 'background', 'synctex-jump-flash');
                            setTimeout(() => {
                                editor.removeLineClass(lineHandle, 'background', 'synctex-jump-flash');
                            }, 1800);
                        }
                    } catch (err) {
                        console.error('SyncTeX jump request failed:', err);
                    }
                };

                canvas.addEventListener('click', triggerSyncTexJump);
                canvas.addEventListener('dblclick', triggerSyncTexJump);

                pdfViewer.appendChild(canvas);
            }
        } catch (err) {
            console.error('PDF render error:', err);
        }
    }

    // 8. Compile LaTeX Handler
    async function compileLatex() {
        const code = editor.getValue();
        if (!code.trim()) return;

        setStatus('Compiling...', 'status-compiling');

        try {
            const response = await window.BackendAdapter.invokeCompile(code);
            
            if (response.pdfBuffer) {
                if (currentPdfUrl) {
                    URL.revokeObjectURL(currentPdfUrl);
                }
                const blob = new Blob([response.pdfBuffer], { type: 'application/pdf' });
                currentPdfUrl = URL.createObjectURL(blob);
                
                await renderPdf(currentPdfUrl);

                errorOverlay.classList.add('hidden');
                setStatus('Compiled Successfully', 'status-success');
            } else {
                errorLog.textContent = response.error || response.log || 'Unknown compilation failure';
                errorOverlay.classList.remove('hidden');
                setStatus('Compilation Error', 'status-error');
            }
        } catch (err) {
            errorLog.textContent = err.toString();
            errorOverlay.classList.remove('hidden');
            setStatus('Server Error', 'status-error');
        }
    }

    // 9. Export Dropdown Handlers
    const exportDropdown = document.getElementById('export-dropdown');
    const exportMenuBtn = document.getElementById('export-menu-btn');
    const exportPdfItem = document.getElementById('export-pdf-item');
    const exportDocxItem = document.getElementById('export-docx-item');

    if (exportMenuBtn) {
        exportMenuBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            exportDropdown.classList.toggle('open');
        });
    }

    document.addEventListener('click', () => {
        if (exportDropdown) {
            exportDropdown.classList.remove('open');
        }
    });

    if (exportPdfItem) {
        exportPdfItem.addEventListener('click', async () => {
            if (exportDropdown) exportDropdown.classList.remove('open');
            if (!currentPdfUrl && !BackendAdapter.isTauri) {
                alert('Please compile the document first to export.');
                return;
            }
            await BackendAdapter.invokeSavePdf(currentPdfUrl);
        });
    }

    if (exportDocxItem) {
        exportDocxItem.addEventListener('click', async () => {
            if (exportDropdown) exportDropdown.classList.remove('open');
            const code = editor.getValue();
            setStatus('Exporting Word...', 'status-compiling');
            const success = await BackendAdapter.invokeSaveDocx(code);
            if (success) {
                setStatus('Exported Word', 'status-success');
                setTimeout(() => setStatus('Ready', 'status-ready'), 3000);
            } else {
                setStatus('Ready', 'status-ready');
            }
        });
    }

    // 10. Action Buttons
    document.getElementById('compile-btn').addEventListener('click', compileLatex);

    // 11. Global Keyboard Shortcuts
    document.addEventListener('keydown', (e) => {
        // Ctrl+S or Ctrl+Enter to compile
        if ((e.ctrlKey || e.metaKey) && (e.key === 's' || e.key === 'Enter')) {
            e.preventDefault();
            compileLatex();
        }
        // Ctrl + + / = to zoom in
        if ((e.ctrlKey || e.metaKey) && (e.key === '=' || e.key === '+')) {
            e.preventDefault();
            document.getElementById('zoom-in-btn').click();
        }
        // Ctrl + - to zoom out
        if ((e.ctrlKey || e.metaKey) && e.key === '-') {
            e.preventDefault();
            document.getElementById('zoom-out-btn').click();
        }
    });

    // 12. Initial Compilation on Launch
    compileLatex();
});
