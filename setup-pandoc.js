const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const BIN_DIR = path.join(__dirname, 'src-tauri', 'binaries');
const IS_WINDOWS = process.platform === 'win32';
const PANDOC_BIN_NAME = IS_WINDOWS ? 'pandoc.exe' : 'pandoc';
const PANDOC_DEST = path.join(BIN_DIR, PANDOC_BIN_NAME);

function getDownloadUrl() {
    const arch = process.arch; // 'x64' or 'arm64'
    const platform = process.platform; // 'win32', 'darwin', 'linux'

    if (platform === 'win32') {
        return {
            url: 'https://github.com/jgm/pandoc/releases/download/3.12.1/pandoc-3.12.1-windows-x86_64.zip',
            ext: 'zip'
        };
    } else if (platform === 'darwin') {
        const macArch = arch === 'arm64' ? 'arm64' : 'x86_64';
        return {
            url: `https://github.com/jgm/pandoc/releases/download/3.12.1/pandoc-3.12.1-${macArch}-macOS.zip`,
            ext: 'zip'
        };
    } else if (platform === 'linux') {
        const linuxArch = arch === 'arm64' ? 'arm64' : 'amd64';
        return {
            url: `https://github.com/jgm/pandoc/releases/download/3.12.1/pandoc-3.12.1-linux-${linuxArch}.tar.gz`,
            ext: 'tar.gz'
        };
    } else {
        throw new Error(`Unsupported platform: ${platform}`);
    }
}

async function setup() {
    if (fs.existsSync(PANDOC_DEST)) {
        console.log(`Pandoc binary already exists at: ${PANDOC_DEST}`);
        return;
    }

    console.log(`Downloading Pandoc standalone binary for ${process.platform} (${process.arch})...`);
    fs.mkdirSync(BIN_DIR, { recursive: true });

    const { url, ext } = getDownloadUrl();
    const tempArchive = path.join(process.env.TEMP || process.env.TMPDIR || '/tmp', `pandoc-temp.${ext}`);
    const tempDir = path.join(process.env.TEMP || process.env.TMPDIR || '/tmp', 'pandoc-temp-extracted');

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

    await download(url, tempArchive);
    console.log(`Downloaded Pandoc archive. Extracting...`);

    fs.mkdirSync(tempDir, { recursive: true });

    try {
        if (ext === 'zip') {
            if (process.platform === 'win32') {
                execSync(`tar -xf "${tempArchive}" -C "${tempDir}"`, { stdio: 'inherit' });
            } else {
                try {
                    execSync(`unzip -q "${tempArchive}" -d "${tempDir}"`, { stdio: 'inherit' });
                } catch {
                    execSync(`tar -xf "${tempArchive}" -C "${tempDir}"`, { stdio: 'inherit' });
                }
            }
        } else {
            execSync(`tar -xzf "${tempArchive}" -C "${tempDir}"`, { stdio: 'inherit' });
        }

        const findBinary = (dir) => {
            const entries = fs.readdirSync(dir, { withFileTypes: true });
            for (const entry of entries) {
                const fullPath = path.join(dir, entry.name);
                if (entry.isDirectory()) {
                    const found = findBinary(fullPath);
                    if (found) return found;
                } else if (entry.name.toLowerCase() === PANDOC_BIN_NAME.toLowerCase()) {
                    return fullPath;
                }
            }
            return null;
        };

        const found = findBinary(tempDir);
        if (found) {
            fs.copyFileSync(found, PANDOC_DEST);
            if (!IS_WINDOWS) {
                fs.chmodSync(PANDOC_DEST, 0o755);
            }
            console.log(`Successfully installed Pandoc to: ${PANDOC_DEST}`);
        } else {
            console.error(`Could not locate ${PANDOC_BIN_NAME} in extracted archive.`);
        }
    } finally {
        try { fs.unlinkSync(tempArchive); } catch (_) {}
        try { fs.rmSync(tempDir, { recursive: true, force: true }); } catch (_) {}
    }
}

setup().catch((err) => {
    console.error('Failed to setup Pandoc:', err);
});
