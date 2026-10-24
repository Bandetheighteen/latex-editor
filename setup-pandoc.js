const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const BIN_DIR = path.join(__dirname, 'src-tauri', 'binaries');
const PANDOC_EXE = path.join(BIN_DIR, 'pandoc.exe');

async function setup() {
    if (fs.existsSync(PANDOC_EXE)) {
        console.log('Pandoc binary already exists at:', PANDOC_EXE);
        return;
    }

    console.log('Downloading Pandoc standalone binary for offline Word export...');
    fs.mkdirSync(BIN_DIR, { recursive: true });

    // Download URL for Pandoc Windows x86_64
    const url = 'https://github.com/jgm/pandoc/releases/download/3.12.1/pandoc-3.12.1-windows-x86_64.zip';
    const tempZip = path.join(process.env.TEMP || '.', 'pandoc-temp.zip');
    const tempDir = path.join(process.env.TEMP || '.', 'pandoc-temp-extracted');

    const download = (src, dest) => new Promise((resolve, reject) => {
        const file = fs.createWriteStream(dest);
        const get = (uri) => {
            https.get(uri, (res) => {
                if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
                    return get(res.headers.location);
                }
                if (res.statusCode !== 200) {
                    return reject(new Error(`Failed to download ${uri} (Status: ${res.statusCode})`));
                }
                res.pipe(file);
                file.on('finish', () => { file.close(); resolve(); });
            }).on('error', (err) => { fs.unlink(dest, () => {}); reject(err); });
        };
        get(src);
    });

    await download(url, tempZip);
    console.log('Downloaded Pandoc zip. Extracting...');

    try {
        execSync(`powershell -Command "Expand-Archive -Path '${tempZip}' -DestinationPath '${tempDir}' -Force"`, { stdio: 'inherit' });
        
        // Find pandoc.exe inside extracted directory
        const findExe = (dir) => {
            const entries = fs.readdirSync(dir, { withFileTypes: true });
            for (const entry of entries) {
                const fullPath = path.join(dir, entry.name);
                if (entry.isDirectory()) {
                    const found = findExe(fullPath);
                    if (found) return found;
                } else if (entry.name.toLowerCase() === 'pandoc.exe') {
                    return fullPath;
                }
            }
            return null;
        };

        const found = findExe(tempDir);
        if (found) {
            fs.copyFileSync(found, PANDOC_EXE);
            console.log('Successfully installed Pandoc to:', PANDOC_EXE);
        } else {
            console.error('Could not locate pandoc.exe in extracted archive.');
        }
    } finally {
        try { fs.unlinkSync(tempZip); } catch (_) {}
        try { fs.rmSync(tempDir, { recursive: true, force: true }); } catch (_) {}
    }
}

setup().catch((err) => {
    console.error('Failed to setup Pandoc:', err);
});
