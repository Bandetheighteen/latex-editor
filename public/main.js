document.addEventListener('DOMContentLoaded', async () => {
    // 1. Template Presets
    const TEMPLATES = {
        article: `\\documentclass[11pt,a4paper]{article}
\\usepackage[left=0.8in,right=0.8in,top=0.8in,bottom=0.8in]{geometry}
\\usepackage{amsmath,amsfonts,amssymb}
\\usepackage{hyperref}
\\usepackage{xcolor}

\\title{\\textbf{\\Huge Research Manuscript}}
\\author{\\Large Author Name}
\\date{\\today}

\\begin{document}
\\maketitle

\\begin{abstract}
This research explores advanced typesetting and computation with real-time feedback and offline reproducibility.
\\end{abstract}

\\section{Introduction}
Welcome to your \\textbf{Native LaTeX Editor}! This workspace compiles directly from your laptop files with zero cloud latency.

\\section{Methodology & Equations}
LaTeX renders complex mathematical structures effortlessly:
\\begin{equation}
    i\\hbar \\frac{\\partial}{\\partial t} \\Psi(\\mathbf{r}, t) = \\hat{\\mathcal{H}} \\Psi(\\mathbf{r}, t)
\\end{equation}

And Euler's celebrated identity:
\\begin{equation}
    e^{i\\pi} + 1 = 0
\\end{equation}

\\section{Summary}
\\begin{itemize}
    \\item Instant offline compilation.
    \\item Intelligent Overleaf-style \\texttt{\\\\begin} autocomplete.
    \\item Direct disk save with \\texttt{Ctrl+S}.
\\end{itemize}

\\end{document}
`,
        beamer: `\\documentclass{beamer}
\\usetheme{Madrid}
\\usecolortheme{default}

\\title{Presentation Title}
\\subtitle{Subtitle or Conference Name}
\\author{Author Name}
\\date{\\today}

\\begin{document}

\\begin{frame}
    \\titlepage
\\end{frame}

\\begin{frame}{Overview}
    \\tableofcontents
\\end{frame}

\\section{Key Objectives}
\\begin{frame}{Key Objectives}
    \\begin{itemize}
        \\item First major objective or finding.
        \\item High-speed native presentation compiling.
        \\item Complete offline reliability.
    \\end{itemize}
\\end{frame}

\\section{Results}
\\begin{frame}{Mathematical Insight}
    \\begin{block}{Fundamental Theorem}
        Let $f(x)$ be continuous on $[a,b]$. Then:
        $$\\int_a^b f(x)\\,dx = F(b) - F(a)$$
    \\end{block}
\\end{frame}

\\end{document}
`,
        resume: `\\documentclass[11pt,a4paper]{article}
\\usepackage[left=0.7in,right=0.7in,top=0.6in,bottom=0.6in]{geometry}
\\usepackage{hyperref}
\\usepackage{titlesec}
\\usepackage{enumitem}

\\pagestyle{empty}
\\titleformat{\\section}{\\large\\bfseries\\uppercase}{}{0em}{}[\\titlerule]

\\begin{document}

\\begin{center}
    {\\Huge \\textbf{YOUR NAME}}\\\\[4pt]
    \\small +1 (555) 019-2834 \\,\\textbar\\, email@domain.com \\,\\textbar\\, \\href{https://linkedin.com}{linkedin.com/in/profile} \\,\\textbar\\, \\href{https://github.com}{github.com/profile}
\\end{center}

\\section{Experience}
\\textbf{Senior Systems Engineer} \\hfill 2023 -- Present\\\\
\\textit{Core Engineering Group} \\hfill \\textit{City, State}
\\begin{itemize}[noitemsep,topsep=2pt]
    \\item Architected native desktop toolchain improving developer workflow speed by 40\\%.
    \\item Built crash-free offline document generation pipeline handling mission-critical output.
\\end{itemize}

\\vspace{6pt}
\\section{Education}
\\textbf{B.S. in Computer Science} \\hfill 2019 -- 2023\\\\
\\textit{University of Engineering} \\hfill GPA: 3.9 / 4.0

\\section{Skills}
\\textbf{Languages:} Rust, JavaScript, TypeScript, C++, Python\\\\
\\textbf{Frameworks & Tools:} Tauri v2, CodeMirror, PDF.js, Git, Linux, Docker

\\end{document}
`,
        math: `\\documentclass[11pt,a4paper]{article}
\\usepackage[margin=0.8in]{geometry}
\\usepackage{amsmath,amssymb,amsthm}
\\usepackage{bm}

\\title{\\textbf{Mathematics & Physics Reference}}
\\author{Reference Notes}
\\date{\\today}

\\begin{document}
\\maketitle

\\section{Linear Algebra}
Matrix transformations and eigenvalues:
\\begin{equation}
    \\mathbf{A}\\mathbf{v} = \\lambda\\mathbf{v}, \\quad \\det(\\mathbf{A} - \\lambda\\mathbf{I}) = 0
\\end{equation}

A $2 \\times 2$ rotation matrix:
\\begin{equation}
    \\mathbf{R}(\\theta) = \\begin{pmatrix}
        \\cos\\theta & -\\sin\\theta \\\\
        \\sin\\theta & \\cos\\theta
    \\end{pmatrix}
\\end{equation}

\\section{Calculus & Vector Calculus}
Maxwell's equations in differential form:
\\begin{align}
    \\nabla \\cdot \\mathbf{E} &= \\frac{\\rho}{\\varepsilon_0} & \\nabla \\times \\mathbf{E} &= -\\frac{\\partial \\mathbf{B}}{\\partial t} \\\\[6pt]
    \\nabla \\cdot \\mathbf{B} &= 0 & \\nabla \\times \\mathbf{B} &= \\mu_0 \\mathbf{J} + \\mu_0\\varepsilon_0 \\frac{\\partial \\mathbf{E}}{\\partial t}
\\end{align}

\\end{document}
`,
        blank: `\\documentclass[11pt,a4paper]{article}
\\usepackage[margin=1in]{geometry}
\\usepackage{amsmath,amsfonts,amssymb}

\\begin{document}

\\section{New Document}
Start writing your LaTeX content here...

\\end{document}
`
    };

    // 2. Initialize Split.js Panes (Optimized for side-by-side landscape on tablets and phones)
    Split(['#editor-pane', '#preview-pane'], {
        sizes: [48, 52],
        minSize: [200, 200],
        gutterSize: 6,
        cursor: 'col-resize'
    });

    // 3. Application State
    let currentFilePath = null;
    let currentFileName = 'document.tex';
    let isDirty = false;
    let isAutoCompile = localStorage.getItem('auto_compile_enabled') === 'true';
    let recentProjects = [];
    let currentPdfUrl = null;
    let pdfDoc = null;
    let currentScale = 1.35;
    let compileTimeout = null;

    // 4. DOM Elements
    const welcomeView = document.getElementById('welcome-view');
    const editorView = document.getElementById('editor-view');
    const navProjectsBtn = document.getElementById('nav-projects-btn');
    const docTitleEl = document.getElementById('doc-title');
    const dirtyIndicatorEl = document.getElementById('dirty-indicator');
    const statusPill = document.getElementById('status-pill');
    const saveBtn = document.getElementById('save-btn');
    const zoomLevelEl = document.getElementById('zoom-level');
    const pdfViewer = document.getElementById('pdf-viewer');
    const errorOverlay = document.getElementById('error-overlay');
    const errorLog = document.getElementById('error-log');
    const errorDismissBtn = document.getElementById('error-dismiss-btn');
    const autoCompileToggle = document.getElementById('auto-compile-toggle');
    const autoCompileLabel = document.getElementById('auto-compile-label');
    const recentProjectsListEl = document.getElementById('recent-projects-list');
    const emptyProjectsStateEl = document.getElementById('empty-projects-state');
    const projectsCountEl = document.getElementById('projects-count');
    const projectSearchInput = document.getElementById('project-search');
    const btnNewProject = document.getElementById('btn-new-project');
    const btnOpenProject = document.getElementById('btn-open-project');

    // 5. Initialize CodeMirror Editor
    const editor = CodeMirror.fromTextArea(document.getElementById('latex-code'), {
        lineNumbers: true,
        mode: 'stex',
        theme: 'monokai',
        lineWrapping: true,
        matchBrackets: true,
        styleActiveLine: true,
        extraKeys: {
            'Ctrl-Space': function(cm) {
                if (CodeMirror.hint && CodeMirror.hint.latex) {
                    cm.showHint({ hint: CodeMirror.hint.latex, completeSingle: false });
                }
            }
        }
    });

    // 5b. LaTeX Symbol Access Bar & Mobile Virtual Keyboard Handling
    const LATEX_SYMBOLS = [
        // Fractions & Roots
        { label: 'a/b', snippet: '\\frac{1}{2}', title: 'Fraction: \\frac{}{}' },
        { label: '√x', snippet: '\\sqrt{x}', title: 'Square Root: \\sqrt{}' },
        { label: 'x²', snippet: '^{2}', title: 'Superscript: ^{}' },
        { label: 'xᵢ', snippet: '_{i}', title: 'Subscript: _{}' },
        { type: 'sep' },
        // Large Operators
        { label: '∑', snippet: '\\sum_{i=1}^{n} ', title: 'Summation: \\sum' },
        { label: '∫', snippet: '\\int_{a}^{b} ', title: 'Integral: \\int' },
        { label: '∏', snippet: '\\prod_{i=1}^{n} ', title: 'Product: \\prod' },
        { label: 'lim', snippet: '\\lim_{x \\to 0} ', title: 'Limit: \\lim' },
        { type: 'sep' },
        // Greek Letters
        { label: 'α', snippet: '\\alpha ', title: 'Alpha' },
        { label: 'β', snippet: '\\beta ', title: 'Beta' },
        { label: 'γ', snippet: '\\gamma ', title: 'Gamma' },
        { label: 'θ', snippet: '\\theta ', title: 'Theta' },
        { label: 'λ', snippet: '\\lambda ', title: 'Lambda' },
        { label: 'π', snippet: '\\pi ', title: 'Pi' },
        { label: 'σ', snippet: '\\sigma ', title: 'Sigma' },
        { label: 'ω', snippet: '\\omega ', title: 'Omega' },
        { label: 'Δ', snippet: '\\Delta ', title: 'Delta' },
        { type: 'sep' },
        // Relations & Operators
        { label: '≠', snippet: '\\neq ', title: 'Not Equal' },
        { label: '≤', snippet: '\\leq ', title: 'Less Than or Equal' },
        { label: '≥', snippet: '\\geq ', title: 'Greater Than or Equal' },
        { label: '≈', snippet: '\\approx ', title: 'Approximately' },
        { label: '±', snippet: '\\pm ', title: 'Plus-Minus' },
        { label: '×', snippet: '\\times ', title: 'Times' },
        { label: '÷', snippet: '\\div ', title: 'Division' },
        { label: '∞', snippet: '\\infty ', title: 'Infinity' },
        { label: '∂', snippet: '\\partial ', title: 'Partial Derivative' },
        { type: 'sep' },
        // Arrows & Sets
        { label: '→', snippet: '\\to ', title: 'Right Arrow' },
        { label: '⇒', snippet: '\\implies ', title: 'Implies' },
        { label: '∈', snippet: '\\in ', title: 'Element Of' },
        { label: '∉', snippet: '\\notin ', title: 'Not In' },
        { label: '⊂', snippet: '\\subset ', title: 'Subset' },
        { label: '∪', snippet: '\\cup ', title: 'Union' },
        { label: '∩', snippet: '\\cap ', title: 'Intersection' },
        { label: '∀', snippet: '\\forall ', title: 'For All' },
        { label: '∃', snippet: '\\exists ', title: 'Exists' },
        { type: 'sep' },
        // Enclosures & Text
        { label: '{ }', snippet: '\\{  \\}', title: 'Curly Braces' },
        { label: '( )', snippet: '\\left(  \\right)', title: 'Auto-sized Parentheses' },
        { label: '$$', snippet: '$$\n\n$$', title: 'Display Math Block' },
        { label: 'B', snippet: '\\textbf{}', title: 'Bold Text' },
        { label: 'I', snippet: '\\textit{}', title: 'Italic Text' },
        { label: 'text', snippet: '\\text{}', title: 'Normal Text' }
    ];

    function insertLatexSymbol(item) {
        const doc = editor.getDoc();
        const cursor = doc.getCursor();
        const selection = doc.getSelection();

        let snippet = item.snippet;
        if (selection && snippet.includes('{}')) {
            snippet = snippet.replace('{}', `{${selection}}`);
            doc.replaceSelection(snippet);
            return;
        }

        doc.replaceRange(snippet, cursor);

        // Position cursor at placeholder
        if (snippet === '\\frac{1}{2}') {
            doc.setSelection({ line: cursor.line, ch: cursor.ch + 6 }, { line: cursor.line, ch: cursor.ch + 7 });
        } else if (snippet.includes('{}')) {
            const bracePos = snippet.indexOf('{');
            doc.setCursor({ line: cursor.line, ch: cursor.ch + bracePos + 1 });
        } else if (snippet.includes('{  }')) {
            doc.setCursor({ line: cursor.line, ch: cursor.ch + snippet.indexOf('{  }') + 2 });
        } else if (snippet.includes('(  )')) {
            doc.setCursor({ line: cursor.line, ch: cursor.ch + snippet.indexOf('(  )') + 2 });
        } else if (snippet === '$$\n\n$$') {
            doc.setCursor({ line: cursor.line + 1, ch: 0 });
        } else {
            doc.setCursor({ line: cursor.line, ch: cursor.ch + snippet.length });
        }
    }

    function initLatexSymbolBar() {
        const symbolBar = document.getElementById('latex-symbol-bar');
        if (!symbolBar) return;
        symbolBar.innerHTML = '';

        LATEX_SYMBOLS.forEach(item => {
            if (item.type === 'sep') {
                const sep = document.createElement('div');
                sep.className = 'symbol-separator';
                symbolBar.appendChild(sep);
                return;
            }

            const btn = document.createElement('button');
            btn.className = 'symbol-btn';
            btn.textContent = item.label;
            btn.title = item.title || item.label;
            btn.type = 'button';

            // Prevent focus loss on desktop; support smooth horizontal touch scrolling on mobile
            let touchMoved = false;
            let touchStartX = 0;
            let touchStartY = 0;

            btn.addEventListener('mousedown', (e) => {
                e.preventDefault();
                insertLatexSymbol(item);
            });

            btn.addEventListener('touchstart', (e) => {
                touchMoved = false;
                touchStartX = e.touches[0].clientX;
                touchStartY = e.touches[0].clientY;
            }, { passive: true });

            btn.addEventListener('touchmove', (e) => {
                if (Math.abs(e.touches[0].clientX - touchStartX) > 10 || Math.abs(e.touches[0].clientY - touchStartY) > 10) {
                    touchMoved = true;
                }
            }, { passive: true });

            btn.addEventListener('touchend', (e) => {
                if (!touchMoved) {
                    e.preventDefault();
                    insertLatexSymbol(item);
                }
            });

            symbolBar.appendChild(btn);
        });

        // Virtual Keyboard hide/show detection:
        // When on-screen keyboard appears, hide symbol bar so it doesn't move up.
        // When keyboard is dismissed, restore symbol bar for comfortable tapping.
        let maxObservedHeight = Math.max(window.innerHeight, (window.screen && window.screen.availHeight) || 0);

        const checkKeyboard = () => {
            const vv = window.visualViewport;
            const currentHeight = vv ? vv.height : window.innerHeight;
            if (currentHeight > maxObservedHeight) {
                maxObservedHeight = currentHeight;
            }

            const refHeight = Math.max(maxObservedHeight, (window.screen && window.screen.availHeight) || 0);
            const heightDiff = refHeight - currentHeight;

            // In landscape, virtual keyboard consumes > 120px and reduces viewport to < 82% of reference height
            const isKeyboardUp = heightDiff > 120 && (currentHeight < refHeight * 0.82);

            if (isKeyboardUp) {
                document.body.classList.add('keyboard-open');
                symbolBar.classList.add('is-hidden');
            } else {
                document.body.classList.remove('keyboard-open');
                symbolBar.classList.remove('is-hidden');
            }

            // Sync CodeMirror dimensions and avoid line clipping after viewport/symbol bar height changes
            if (editor) {
                editor.refresh();
            }
        };

        if (window.visualViewport) {
            window.visualViewport.addEventListener('resize', checkKeyboard);
            window.visualViewport.addEventListener('scroll', checkKeyboard);
        }
        window.addEventListener('resize', checkKeyboard);

        // CodeMirror focus and blur listeners for immediate keyboard reactivity
        editor.on('focus', () => {
            setTimeout(checkKeyboard, 250);
        });
        editor.on('blur', () => {
            setTimeout(checkKeyboard, 100);
        });
    }
    initLatexSymbolBar();

    // Tablet Recommendation Banner dismissal handler
    const disclaimerBanner = document.getElementById('tablet-disclaimer-banner');
    const dismissDisclaimerBtn = document.getElementById('disclaimer-dismiss-btn');
    if (localStorage.getItem('tablet_banner_dismissed') === 'true') {
        if (disclaimerBanner) disclaimerBanner.classList.add('hidden');
    }
    if (dismissDisclaimerBtn) {
        dismissDisclaimerBtn.addEventListener('click', () => {
            if (disclaimerBanner) disclaimerBanner.classList.add('hidden');
            localStorage.setItem('tablet_banner_dismissed', 'true');
        });
    }

    // 6. Intelligent Autocomplete Trigger
    editor.on('inputRead', (cm, change) => {
        if (!CodeMirror.hint || !CodeMirror.hint.latex) return;
        const text = change.text[0];
        // Trigger autocomplete on typing backslash or opening curly brace, or letters after backslash
        if (text === '\\' || text === '{') {
            cm.showHint({ hint: CodeMirror.hint.latex, completeSingle: false });
        } else if (/^[a-zA-Z*]$/.test(text)) {
            const cur = cm.getCursor();
            const line = cm.getLine(cur.line).slice(0, cur.ch);
            if (/\\([a-zA-Z*]*)$/.test(line) || /\\begin\{([a-zA-Z*]*)$/.test(line)) {
                cm.showHint({ hint: CodeMirror.hint.latex, completeSingle: false });
            }
        }
    });

    // 7. Track Document Changes & Auto-Compile
    editor.on('change', () => {
        setDirty(true);
        clearTimeout(compileTimeout);
        if (isAutoCompile) {
            compileTimeout = setTimeout(() => {
                compileLatex();
            }, 1200);
        }
    });

    function setDirty(dirty) {
        isDirty = dirty;
        if (isDirty) {
            dirtyIndicatorEl.classList.remove('hidden');
        } else {
            dirtyIndicatorEl.classList.add('hidden');
        }
    }

    function setStatus(text, className) {
        statusPill.textContent = text;
        statusPill.className = `status-pill ${className}`;
    }

    function updateDocTitleUI() {
        docTitleEl.textContent = currentFileName;
        document.title = `${currentFileName} • LaTeX Editor`;
    }

    // 8. Configure PDF.js Worker
    pdfjsLib.GlobalWorkerOptions.workerSrc = '/vendor/pdfjs/pdf.worker.min.js';

    function updateZoomDisplay() {
        zoomLevelEl.textContent = `${Math.round(currentScale * 100)}%`;
    }
    updateZoomDisplay();

    // 9. Zoom Stepper Controls
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

    // 10. Dismiss Error Overlay
    errorDismissBtn.addEventListener('click', () => {
        errorOverlay.classList.add('hidden');
    });

    // 11. Render PDF Canvas
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
                    const scaleX = canvas.width / rect.width;
                    const scaleY = canvas.height / rect.height;
                    const canvasX = (e.clientX - rect.left) * scaleX;
                    const canvasY = (e.clientY - rect.top) * scaleY;

                    const pdfX = canvasX / currentScale;
                    const pdfY = canvasY / currentScale;

                    try {
                        const data = await window.BackendAdapter.invokeSyncTeX(pageNum, pdfX, pdfY);

                        if (data && data.line) {
                            const targetLine = data.line - 1;
                            const lineLen = editor.getLine(targetLine) ? editor.getLine(targetLine).length : 0;
                            editor.setSelection({ line: targetLine, ch: 0 }, { line: targetLine, ch: lineLen });

                            const h = editor.getScrollInfo().clientHeight;
                            const coords = editor.charCoords({ line: targetLine, ch: 0 }, "local");
                            editor.scrollTo(null, Math.max(0, coords.top - h / 2));
                            editor.focus();

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

    // 12. Compile LaTeX Handler
    async function compileLatex() {
        const code = editor.getValue();
        if (!code.trim()) return;

        setStatus('Compiling...', 'status-compiling');

        try {
            const response = await window.BackendAdapter.invokeCompile(code, currentFilePath);

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

    // 13. Direct Disk Save Handler
    async function saveDocument() {
        const code = editor.getValue();
        setStatus('Saving...', 'status-compiling');

        try {
            const res = await BackendAdapter.saveFileDirect(currentFilePath, code);
            if (res && res.success) {
                currentFilePath = res.path;
                currentFileName = res.name;
                setDirty(false);
                updateDocTitleUI();
                setStatus('Saved to disk', 'status-ready');
                setTimeout(() => setStatus('Ready', 'status-ready'), 2000);
                return true;
            } else {
                setStatus('Ready', 'status-ready');
                return false;
            }
        } catch (err) {
            console.error('Save failed:', err);
            alert('Failed to save document: ' + err);
            setStatus('Save Error', 'status-error');
            return false;
        }
    }

    saveBtn.addEventListener('click', saveDocument);

    // 14. Auto-Compile UI Toggle
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

    autoCompileToggle.addEventListener('click', () => {
        isAutoCompile = !isAutoCompile;
        localStorage.setItem('auto_compile_enabled', isAutoCompile.toString());
        updateAutoCompileUI();
        if (isAutoCompile) {
            compileLatex();
        }
    });

    // 15. View Navigation Handlers
    function showWelcomeView() {
        welcomeView.classList.remove('hidden');
        editorView.classList.add('hidden');
        document.title = 'LaTeX Editor';
        loadAndRenderRecentProjects();
    }

    function showEditorView(filePath, fileName, content) {
        currentFilePath = filePath || null;
        currentFileName = fileName || 'Untitled.tex';

        // Clear previous PDF preview so old project output never lingers
        if (currentPdfUrl) {
            URL.revokeObjectURL(currentPdfUrl);
            currentPdfUrl = null;
        }
        pdfDoc = null;
        pdfViewer.innerHTML = '';
        errorOverlay.classList.add('hidden');

        // Set code: empty string if blank new project
        const codeToSet = (content !== undefined && content !== null) ? content : '';
        editor.setValue(codeToSet);
        editor.clearHistory();
        setDirty(false);
        updateDocTitleUI();

        welcomeView.classList.add('hidden');
        editorView.classList.remove('hidden');

        // Refresh CodeMirror layout measurements
        setTimeout(() => {
            editor.refresh();
            editor.focus();
        }, 50);

        if (codeToSet.trim()) {
            compileLatex();
        } else {
            setStatus('Ready', 'status-ready');
            pdfViewer.innerHTML = `
                <div class="empty-preview-placeholder">
                    <svg width="44" height="44" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                        <polyline points="14 2 14 8 20 8"></polyline>
                        <line x1="12" y1="18" x2="12" y2="12"></line>
                        <line x1="9" y1="15" x2="15" y2="15"></line>
                    </svg>
                    <p>New LaTeX Document</p>
                    <span>Start writing code or type <code>\\begin</code> for suggestions</span>
                </div>
            `;
        }
    }

    navProjectsBtn.addEventListener('click', () => {
        if (isDirty) {
            if (confirm(`You have unsaved changes in "${currentFileName}". Return to Projects without saving?`)) {
                showWelcomeView();
            }
        } else {
            showWelcomeView();
        }
    });

    // 16. Action: New Project & Templates
    btnNewProject.addEventListener('click', () => {
        showEditorView(null, 'Untitled.tex', '');
    });

    document.querySelectorAll('.template-chip').forEach(chip => {
        chip.addEventListener('click', () => {
            const tmplKey = chip.getAttribute('data-template');
            const tmplCode = TEMPLATES[tmplKey] || TEMPLATES.article;
            const docName = tmplKey === 'beamer' ? 'Presentation.tex' : (tmplKey === 'resume' ? 'Resume.tex' : 'document.tex');
            showEditorView(null, docName, tmplCode);
        });
    });

    // 17. Action: Open File / Project Dialog
    async function handleOpenProject() {
        try {
            const payload = await BackendAdapter.openFileDialog();
            if (payload && payload.path) {
                showEditorView(payload.path, payload.name, payload.content);
            }
        } catch (err) {
            alert('Failed to open file: ' + err);
        }
    }

    btnOpenProject.addEventListener('click', handleOpenProject);

    // 18. Recent Projects Formatting & Rendering
    function formatRelativeTime(timestampSec) {
        if (!timestampSec) return '';
        const now = Math.floor(Date.now() / 1000);
        const diff = Math.max(0, now - timestampSec);

        if (diff < 60) return 'Just now';
        if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
        if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
        if (diff < 604800) return `${Math.floor(diff / 86400)}d ago`;
        const date = new Date(timestampSec * 1000);
        return date.toLocaleDateString();
    }

    function formatFileSize(bytes) {
        if (!bytes || bytes <= 0) return '';
        if (bytes < 1024) return `${bytes} B`;
        if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
        return `${(bytes / 1048576).toFixed(1)} MB`;
    }

    async function loadAndRenderRecentProjects() {
        recentProjects = await BackendAdapter.listRecentProjects();
        renderProjectsList(recentProjects);
    }

    function renderProjectsList(projects) {
        projectsCountEl.textContent = projects.length;
        recentProjectsListEl.innerHTML = '';

        if (!projects || projects.length === 0) {
            emptyProjectsStateEl.classList.remove('hidden');
            return;
        }

        emptyProjectsStateEl.classList.add('hidden');

        projects.forEach(project => {
            const card = document.createElement('div');
            card.className = 'project-card';

            card.innerHTML = `
                <div class="project-card-top">
                    <div class="project-name-row">
                        <svg class="project-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                            <polyline points="14 2 14 8 20 8"></polyline>
                        </svg>
                        <span class="project-name" title="${project.name}">${project.name}</span>
                    </div>
                    <button class="btn-remove-project" title="Remove from recent history">&times;</button>
                </div>
                <div class="project-path" title="${project.path}">${project.path}</div>
                <div class="project-meta-row">
                    <span>${formatRelativeTime(project.last_opened)}</span>
                    ${!project.exists ? '<span class="badge-missing">Missing</span>' : `<span>${formatFileSize(project.file_size)}</span>`}
                </div>
            `;

            // Card click: open project
            card.addEventListener('click', async (e) => {
                if (e.target.closest('.btn-remove-project')) return;
                try {
                    const payload = await BackendAdapter.readFileByPath(project.path);
                    if (payload) {
                        showEditorView(payload.path, payload.name, payload.content);
                    }
                } catch (err) {
                    alert('Could not open file: ' + err);
                    loadAndRenderRecentProjects();
                }
            });

            // Remove button click
            const removeBtn = card.querySelector('.btn-remove-project');
            removeBtn.addEventListener('click', async (e) => {
                e.stopPropagation();
                await BackendAdapter.removeRecentProject(project.path);
                loadAndRenderRecentProjects();
            });

            recentProjectsListEl.appendChild(card);
        });
    }

    // Search / Filter Recent Projects
    projectSearchInput.addEventListener('input', () => {
        const query = projectSearchInput.value.toLowerCase().trim();
        if (!query) {
            renderProjectsList(recentProjects);
            return;
        }
        const filtered = recentProjects.filter(p =>
            p.name.toLowerCase().includes(query) || p.path.toLowerCase().includes(query)
        );
        renderProjectsList(filtered);
    });

    // 19. Export Handlers
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
            await BackendAdapter.invokeSavePdf();
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

    // 20. Action Buttons
    document.getElementById('compile-btn').addEventListener('click', compileLatex);

    // 21. Global Keyboard Shortcuts
    document.addEventListener('keydown', async (e) => {
        // Ctrl+S / Cmd+S: Save document to disk and compile
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
            e.preventDefault();
            const saved = await saveDocument();
            if (saved) {
                compileLatex();
            }
        }

        // Ctrl+Enter: Compile LaTeX
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
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

    // 22. Startup: Launch Router
    try {
        const launchFile = await BackendAdapter.getLaunchFile();
        if (launchFile && launchFile.path) {
            showEditorView(launchFile.path, launchFile.name, launchFile.content);
        } else {
            showWelcomeView();
        }
    } catch (err) {
        console.error('Startup launch check failed:', err);
        showWelcomeView();
    }
});
