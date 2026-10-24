class BackendAdapter {
    static get isTauri() {
        return typeof window !== 'undefined' && window.__TAURI__ !== undefined;
    }

    static async invokeCompile(latexCode) {
        if (!this.isTauri) {
            return { error: 'LaTeX Editor must be run as a native desktop application.' };
        }
        try {
            const response = await window.__TAURI__.core.invoke('compile_latex', { code: latexCode });
            if (response.pdf_buffer) {
                const arrayBuffer = new Uint8Array(response.pdf_buffer).buffer;
                return { pdfBuffer: arrayBuffer, log: response.log };
            }
            return { error: response.error || response.log || 'Compilation failed' };
        } catch (err) {
            return { error: err.toString() };
        }
    }

    static async invokeSyncTeX(pageNum, pdfX, pdfY) {
        if (!this.isTauri) return null;
        try {
            return await window.__TAURI__.core.invoke('synctex', { page: pageNum, x: pdfX, y: pdfY });
        } catch (err) {
            console.error("Tauri SyncTeX error:", err);
            return null;
        }
    }

    static async invokeSavePdf() {
        if (!this.isTauri) {
            alert('Saving PDF requires the native desktop application.');
            return false;
        }
        try {
            return await window.__TAURI__.core.invoke('save_pdf');
        } catch (err) {
            console.error("Tauri save PDF error:", err);
            alert("Failed to save PDF: " + err);
            return false;
        }
    }
}

window.BackendAdapter = BackendAdapter;
