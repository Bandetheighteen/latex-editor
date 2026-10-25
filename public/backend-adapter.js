class BackendAdapter {
    static get isTauri() {
        return typeof window !== 'undefined' && window.__TAURI__ !== undefined;
    }

    static async invokeCompile(latexCode, projectPath = null) {
        if (!this.isTauri) {
            return { error: 'LaTeX Editor must be run as a native desktop application.' };
        }
        try {
            const response = await window.__TAURI__.core.invoke('compile_latex', {
                code: latexCode,
                projectPath: projectPath || null
            });
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

    static async invokeSaveDocx(latexCode) {
        if (!this.isTauri) {
            alert('Saving Word document requires the native desktop application.');
            return false;
        }
        try {
            return await window.__TAURI__.core.invoke('save_docx', { code: latexCode });
        } catch (err) {
            console.error("Tauri save DOCX error:", err);
            alert("Failed to export Word document: " + err);
            return false;
        }
    }

    static async getLaunchFile() {
        if (!this.isTauri) return null;
        try {
            return await window.__TAURI__.core.invoke('get_launch_file');
        } catch (err) {
            console.error("Tauri getLaunchFile error:", err);
            return null;
        }
    }

    static async openFileDialog() {
        if (!this.isTauri) return null;
        try {
            return await window.__TAURI__.core.invoke('open_file_dialog');
        } catch (err) {
            console.error("Tauri openFileDialog error:", err);
            return null;
        }
    }

    static async readFileByPath(path) {
        if (!this.isTauri) return null;
        try {
            return await window.__TAURI__.core.invoke('read_file_by_path', { path });
        } catch (err) {
            console.error("Tauri readFileByPath error:", err);
            throw err;
        }
    }

    static async saveFileDirect(path, content) {
        if (!this.isTauri) return { success: false };
        try {
            return await window.__TAURI__.core.invoke('save_file_direct', { path: path || null, content });
        } catch (err) {
            console.error("Tauri saveFileDirect error:", err);
            throw err;
        }
    }

    static async listRecentProjects() {
        if (!this.isTauri) return [];
        try {
            return await window.__TAURI__.core.invoke('list_recent_projects');
        } catch (err) {
            console.error("Tauri listRecentProjects error:", err);
            return [];
        }
    }

    static async removeRecentProject(path) {
        if (!this.isTauri) return false;
        try {
            return await window.__TAURI__.core.invoke('remove_recent_project', { path });
        } catch (err) {
            console.error("Tauri removeRecentProject error:", err);
            return false;
        }
    }
}

window.BackendAdapter = BackendAdapter;
