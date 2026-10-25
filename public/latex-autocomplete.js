(function() {
    'use strict';

    if (typeof CodeMirror === 'undefined') return;

    // Common Environments with full templates and cursor placement
    const ENVIRONMENTS = [
        {
            name: 'itemize',
            desc: 'Bullet list',
            snippet: "\\begin{itemize}\n    \\item \n\\end{itemize}",
            cursorOffset: { line: 1, ch: 10 }
        },
        {
            name: 'enumerate',
            desc: 'Numbered list',
            snippet: "\\begin{enumerate}\n    \\item \n\\end{enumerate}",
            cursorOffset: { line: 1, ch: 10 }
        },
        {
            name: 'description',
            desc: 'Labelled description list',
            snippet: "\\begin{description}\n    \\item[] \n\\end{description}",
            cursorOffset: { line: 1, ch: 11 }
        },
        {
            name: 'equation',
            desc: 'Numbered equation',
            snippet: "\\begin{equation}\n    \n\\end{equation}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'equation*',
            desc: 'Unnumbered equation',
            snippet: "\\begin{equation*}\n    \n\\end{equation*}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'align',
            desc: 'Aligned math equations',
            snippet: "\\begin{align}\n    \n\\end{align}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'align*',
            desc: 'Unnumbered aligned math',
            snippet: "\\begin{align*}\n    \n\\end{align*}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'figure',
            desc: 'Figure with caption and label',
            snippet: "\\begin{figure}[htbp]\n    \\centering\n    \\includegraphics[width=0.8\\textwidth]{}\n    \\caption{}\n    \\label{fig:}\n\\end{figure}",
            cursorOffset: { line: 2, ch: 43 }
        },
        {
            name: 'table',
            desc: 'Table with centered tabular block',
            snippet: "\\begin{table}[htbp]\n    \\centering\n    \\begin{tabular}{ccc}\n        \\hline\n        Header 1 & Header 2 & Header 3 \\\\\n        \\hline\n        Row 1 & Value & Data \\\\\n        \\hline\n    \\end{tabular}\n    \\caption{}\n    \\label{tab:}\n\\end{table}",
            cursorOffset: { line: 9, ch: 13 }
        },
        {
            name: 'tabular',
            desc: 'Basic tabular table grid',
            snippet: "\\begin{tabular}{cc}\n    \\hline\n     &  \\\\\n    \\hline\n\\end{tabular}",
            cursorOffset: { line: 2, ch: 4 }
        },
        {
            name: 'abstract',
            desc: 'Paper abstract summary',
            snippet: "\\begin{abstract}\n    \n\\end{abstract}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'center',
            desc: 'Centered text block',
            snippet: "\\begin{center}\n    \n\\end{center}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'quote',
            desc: 'Block quote indentation',
            snippet: "\\begin{quote}\n    \n\\end{quote}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'verbatim',
            desc: 'Monospaced literal code block',
            snippet: "\\begin{verbatim}\n\n\\end{verbatim}",
            cursorOffset: { line: 1, ch: 0 }
        },
        {
            name: 'matrix',
            desc: 'Plain matrix without delimiters',
            snippet: "\\begin{matrix}\n    a & b \\\\\n    c & d\n\\end{matrix}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'pmatrix',
            desc: 'Matrix with parentheses ()',
            snippet: "\\begin{pmatrix}\n    a & b \\\\\n    c & d\n\\end{pmatrix}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'bmatrix',
            desc: 'Matrix with brackets []',
            snippet: "\\begin{bmatrix}\n    a & b \\\\\n    c & d\n\\end{bmatrix}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'theorem',
            desc: 'Mathematical theorem',
            snippet: "\\begin{theorem}\n    \n\\end{theorem}",
            cursorOffset: { line: 1, ch: 4 }
        },
        {
            name: 'proof',
            desc: 'Mathematical proof block',
            snippet: "\\begin{proof}\n    \n\\end{proof}",
            cursorOffset: { line: 1, ch: 4 }
        }
    ];

    // Popular LaTeX Packages
    const POPULAR_PACKAGES = [
        'amsmath', 'amsfonts', 'amssymb', 'graphicx', 'hyperref',
        'xcolor', 'geometry', 'booktabs', 'tikz', 'cite',
        'biblatex', 'listings', 'microtype', 'tabularx', 'subcaption',
        'float', 'algorithm', 'algpseudocode', 'enumitem', 'siunitx',
        'fancyhdr', 'setspace', 'cleveref', 'multicol', 'titlesec'
    ];

    // Standard Commands
    const COMMANDS = [
        { name: 'section', desc: 'Top-level section heading', insert: '\\section{}', offset: 9 },
        { name: 'subsection', desc: 'Second-level subsection heading', insert: '\\subsection{}', offset: 12 },
        { name: 'subsubsection', desc: 'Third-level subsubsection heading', insert: '\\subsubsection{}', offset: 15 },
        { name: 'paragraph', desc: 'Inline paragraph title', insert: '\\paragraph{}', offset: 11 },
        { name: 'textbf', desc: 'Bold text formatting', insert: '\\textbf{}', offset: 8 },
        { name: 'textit', desc: 'Italic text formatting', insert: '\\textit{}', offset: 8 },
        { name: 'underline', desc: 'Underlined text formatting', insert: '\\underline{}', offset: 11 },
        { name: 'emph', desc: 'Emphasized text', insert: '\\emph{}', offset: 6 },
        { name: 'texttt', desc: 'Monospace typewriter text', insert: '\\texttt{}', offset: 8 },
        { name: 'usepackage', desc: 'Import LaTeX package', insert: '\\usepackage{}', offset: 12 },
        { name: 'documentclass', desc: 'Declare document class', insert: '\\documentclass{}', offset: 14 },
        { name: 'title', desc: 'Document title metadata', insert: '\\title{}', offset: 7 },
        { name: 'author', desc: 'Document author metadata', insert: '\\author{}', offset: 8 },
        { name: 'date', desc: 'Document date (e.g. \\today)', insert: '\\date{\\today}', offset: 13 },
        { name: 'maketitle', desc: 'Render title and author block', insert: '\\maketitle', offset: 9 },
        { name: 'tableofcontents', desc: 'Generate table of contents', insert: '\\tableofcontents', offset: 16 },
        { name: 'newpage', desc: 'Insert hard page break', insert: '\\newpage', offset: 8 },
        { name: 'label', desc: 'Anchor reference label', insert: '\\label{}', offset: 7 },
        { name: 'ref', desc: 'Reference to section or figure', insert: '\\ref{}', offset: 5 },
        { name: 'cite', desc: 'Bibliography citation reference', insert: '\\cite{}', offset: 6 },
        { name: 'footnote', desc: 'Add bottom-of-page footnote', insert: '\\footnote{}', offset: 10 },
        { name: 'href', desc: 'Clickable hyperlink with URL', insert: '\\href{}{}', offset: 6 },
        { name: 'url', desc: 'Raw clickable URL', insert: '\\url{}', offset: 5 },
        { name: 'includegraphics', desc: 'Include image file', insert: '\\includegraphics[width=0.8\\textwidth]{}', offset: 37 },
        { name: 'centering', desc: 'Center align current block', insert: '\\centering', offset: 10 },
        { name: 'noindent', desc: 'Suppress paragraph indentation', insert: '\\noindent', offset: 9 },
        { name: 'vspace', desc: 'Vertical space spacing', insert: '\\vspace{}', offset: 8 },
        { name: 'hspace', desc: 'Horizontal space spacing', insert: '\\hspace{}', offset: 8 },
        { name: 'rule', desc: 'Horizontal rule line', insert: '\\rule{\\textwidth}{0.4pt}', offset: 24 }
    ];

    // Math Mode Commands & Greek Symbols
    const MATH_COMMANDS = [
        { name: 'frac', desc: 'Fraction (num/den)', insert: '\\frac{}{}', offset: 6 },
        { name: 'sqrt', desc: 'Square root', insert: '\\sqrt{}', offset: 6 },
        { name: 'sum', desc: 'Summation notation', insert: '\\sum_{i=1}^{n}', offset: 14 },
        { name: 'int', desc: 'Definite integral', insert: '\\int_{a}^{b}', offset: 12 },
        { name: 'prod', desc: 'Product operator', insert: '\\prod_{i=1}^{n}', offset: 15 },
        { name: 'lim', desc: 'Limit expression', insert: '\\lim_{x \\to 0}', offset: 14 },
        { name: 'partial', desc: 'Partial derivative symbol', insert: '\\partial', offset: 8 },
        { name: 'nabla', desc: 'Del / vector gradient symbol', insert: '\\nabla', offset: 6 },
        { name: 'infty', desc: 'Infinity symbol', insert: '\\infty', offset: 6 },
        { name: 'alpha', desc: 'Greek letter alpha', insert: '\\alpha', offset: 6 },
        { name: 'beta', desc: 'Greek letter beta', insert: '\\beta', offset: 5 },
        { name: 'gamma', desc: 'Greek letter gamma', insert: '\\gamma', offset: 6 },
        { name: 'delta', desc: 'Greek letter delta', insert: '\\delta', offset: 6 },
        { name: 'epsilon', desc: 'Greek letter epsilon', insert: '\\epsilon', offset: 8 },
        { name: 'theta', desc: 'Greek letter theta', insert: '\\theta', offset: 6 },
        { name: 'lambda', desc: 'Greek letter lambda', insert: '\\lambda', offset: 7 },
        { name: 'mu', desc: 'Greek letter mu', insert: '\\mu', offset: 3 },
        { name: 'pi', desc: 'Greek letter pi', insert: '\\pi', offset: 3 },
        { name: 'sigma', desc: 'Greek letter sigma', insert: '\\sigma', offset: 6 },
        { name: 'tau', desc: 'Greek letter tau', insert: '\\tau', offset: 4 },
        { name: 'omega', desc: 'Greek letter omega', insert: '\\omega', offset: 6 },
        { name: 'times', desc: 'Multiplication cross', insert: '\\times', offset: 6 },
        { name: 'cdot', desc: 'Centered multiplication dot', insert: '\\cdot', offset: 5 },
        { name: 'approx', desc: 'Approximately equal', insert: '\\approx', offset: 7 },
        { name: 'neq', desc: 'Not equal sign', insert: '\\neq', offset: 4 },
        { name: 'le', desc: 'Less than or equal', insert: '\\le', offset: 3 },
        { name: 'ge', desc: 'Greater than or equal', insert: '\\ge', offset: 3 },
        { name: 'pm', desc: 'Plus-minus sign', insert: '\\pm', offset: 3 },
        { name: 'in', desc: 'Set membership in', insert: '\\in', offset: 3 },
        { name: 'forall', desc: 'Universal quantifier for all', insert: '\\forall', offset: 7 },
        { name: 'exists', desc: 'Existential quantifier exists', insert: '\\exists', offset: 7 },
        { name: 'to', desc: 'Right arrow to', insert: '\\to', offset: 3 },
        { name: 'implies', desc: 'Logical implies arrow', insert: '\\implies', offset: 8 },
        { name: 'mathbf', desc: 'Bold math font', insert: '\\mathbf{}', offset: 8 },
        { name: 'mathbb', desc: 'Blackboard bold font', insert: '\\mathbb{}', offset: 8 },
        { name: 'mathcal', desc: 'Calligraphic math font', insert: '\\mathcal{}', offset: 9 }
    ];

    // Helper: Find defined \label{...} tags in the current editor content
    function findDocumentLabels(cm) {
        const text = cm.getValue();
        const regex = /\\label\{([^}]+)\}/g;
        const labels = [];
        let match;
        while ((match = regex.exec(text)) !== null) {
            if (!labels.includes(match[1])) {
                labels.push(match[1]);
            }
        }
        return labels;
    }

    // CodeMirror LaTeX hint helper
    CodeMirror.registerHelper('hint', 'latex', function(cm) {
        const cur = cm.getCursor();
        const line = cm.getLine(cur.line);
        const lineUpToCursor = line.slice(0, cur.ch);

        // 1. Context: inside \begin{...}
        const beginMatch = lineUpToCursor.match(/\\begin\{([a-zA-Z0-9*]*)$/);
        if (beginMatch) {
            const typed = beginMatch[1].toLowerCase();
            const startCh = cur.ch - typed.length;
            const matches = ENVIRONMENTS
                .filter(env => env.name.toLowerCase().startsWith(typed))
                .map(env => ({
                    text: env.name,
                    displayText: `[env] ${env.name} — ${env.desc}`,
                    className: 'cm-hint-env',
                    hint: function(cmInstance, data, completion) {
                        // Check if closing brace already exists right after cursor
                        const afterCursor = line.slice(cur.ch);
                        const hasClosingBrace = afterCursor.startsWith('}');
                        const endCh = hasClosingBrace ? cur.ch + 1 : cur.ch;

                        // Insert full snippet block
                        const snippet = env.snippet;
                        // Replace the \begin{...} with the complete template
                        const beginStartCh = lineUpToCursor.lastIndexOf('\\begin{');
                        cmInstance.replaceRange(snippet, { line: cur.line, ch: beginStartCh }, { line: cur.line, ch: endCh });

                        // Position cursor at specified snippet offset
                        cmInstance.setCursor({
                            line: cur.line + env.cursorOffset.line,
                            ch: env.cursorOffset.ch
                        });
                    }
                }));

            if (matches.length > 0) {
                return {
                    list: matches,
                    from: CodeMirror.Pos(cur.line, startCh),
                    to: CodeMirror.Pos(cur.line, cur.ch)
                };
            }
        }

        // 2. Context: inside \usepackage{...}
        const pkgMatch = lineUpToCursor.match(/\\usepackage(\[[^\]]*\])?\{([a-zA-Z0-9_-]*)$/);
        if (pkgMatch) {
            const typed = pkgMatch[2].toLowerCase();
            const startCh = cur.ch - typed.length;
            const matches = POPULAR_PACKAGES
                .filter(pkg => pkg.toLowerCase().startsWith(typed))
                .map(pkg => ({
                    text: pkg,
                    displayText: `[pkg] ${pkg}`,
                    className: 'cm-hint-pkg',
                    hint: function(cmInstance) {
                        const after = line.slice(cur.ch);
                        const appendBrace = !after.startsWith('}');
                        const insertText = pkg + (appendBrace ? '}' : '');
                        cmInstance.replaceRange(insertText, { line: cur.line, ch: startCh }, { line: cur.line, ch: cur.ch });
                    }
                }));

            if (matches.length > 0) {
                return {
                    list: matches,
                    from: CodeMirror.Pos(cur.line, startCh),
                    to: CodeMirror.Pos(cur.line, cur.ch)
                };
            }
        }

        // 3. Context: inside \ref{...} or \cite{...}
        const refMatch = lineUpToCursor.match(/\\(ref|cite|pageref)\{([a-zA-Z0-9_:-]*)$/);
        if (refMatch) {
            const typed = refMatch[2].toLowerCase();
            const startCh = cur.ch - typed.length;
            const labels = findDocumentLabels(cm);
            const matches = labels
                .filter(lbl => lbl.toLowerCase().startsWith(typed))
                .map(lbl => ({
                    text: lbl,
                    displayText: `[ref] ${lbl}`,
                    className: 'cm-hint-ref',
                    hint: function(cmInstance) {
                        const after = line.slice(cur.ch);
                        const appendBrace = !after.startsWith('}');
                        cmInstance.replaceRange(lbl + (appendBrace ? '}' : ''), { line: cur.line, ch: startCh }, { line: cur.line, ch: cur.ch });
                    }
                }));

            if (matches.length > 0) {
                return {
                    list: matches,
                    from: CodeMirror.Pos(cur.line, startCh),
                    to: CodeMirror.Pos(cur.line, cur.ch)
                };
            }
        }

        // 4. Context: General LaTeX command starting with "\"
        const cmdMatch = lineUpToCursor.match(/\\([a-zA-Z0-9*]*)$/);
        if (cmdMatch) {
            const typed = cmdMatch[1].toLowerCase();
            const startCh = cur.ch - typed.length - 1; // include the backslash
            const list = [];

            // If user typed \b or \begin, offer all environments as full snippets
            if ('begin'.startsWith(typed) || typed === '') {
                ENVIRONMENTS.forEach(env => {
                    list.push({
                        text: env.name,
                        displayText: `[env] \\begin{${env.name}} — ${env.desc}`,
                        className: 'cm-hint-env',
                        hint: function(cmInstance) {
                            cmInstance.replaceRange(env.snippet, { line: cur.line, ch: startCh }, { line: cur.line, ch: cur.ch });
                            cmInstance.setCursor({
                                line: cur.line + env.cursorOffset.line,
                                ch: env.cursorOffset.ch
                            });
                        }
                    });
                });
            }

            // Command suggestions
            COMMANDS
                .filter(cmd => cmd.name.toLowerCase().startsWith(typed))
                .forEach(cmd => {
                    list.push({
                        text: cmd.name,
                        displayText: `[cmd] \\${cmd.name} — ${cmd.desc}`,
                        className: 'cm-hint-cmd',
                        hint: function(cmInstance) {
                            cmInstance.replaceRange(cmd.insert, { line: cur.line, ch: startCh }, { line: cur.line, ch: cur.ch });
                            cmInstance.setCursor({
                                line: cur.line,
                                ch: startCh + cmd.offset
                            });
                        }
                    });
                });

            // Math suggestions
            MATH_COMMANDS
                .filter(math => math.name.toLowerCase().startsWith(typed))
                .forEach(math => {
                    list.push({
                        text: math.name,
                        displayText: `[math] \\${math.name} — ${math.desc}`,
                        className: 'cm-hint-math',
                        hint: function(cmInstance) {
                            cmInstance.replaceRange(math.insert, { line: cur.line, ch: startCh }, { line: cur.line, ch: cur.ch });
                            cmInstance.setCursor({
                                line: cur.line,
                                ch: startCh + math.offset
                            });
                        }
                    });
                });

            if (list.length > 0) {
                return {
                    list: list,
                    from: CodeMirror.Pos(cur.line, startCh),
                    to: CodeMirror.Pos(cur.line, cur.ch)
                };
            }
        }

        return null;
    });
})();
