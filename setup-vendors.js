const fs = require('fs');
const path = require('path');
const https = require('https');

const vendors = [
  // CodeMirror
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/codemirror.min.css',
    dest: 'public/vendor/codemirror/codemirror.min.css'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/theme/monokai.min.css',
    dest: 'public/vendor/codemirror/monokai.min.css'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/codemirror.min.js',
    dest: 'public/vendor/codemirror/codemirror.min.js'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/mode/stex/stex.min.js',
    dest: 'public/vendor/codemirror/stex.min.js'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/addon/selection/active-line.min.js',
    dest: 'public/vendor/codemirror/active-line.min.js'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/addon/hint/show-hint.min.js',
    dest: 'public/vendor/codemirror/show-hint.min.js'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.13/addon/hint/show-hint.min.css',
    dest: 'public/vendor/codemirror/show-hint.min.css'
  },
  // PDF.js
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/2.16.105/pdf.min.js',
    dest: 'public/vendor/pdfjs/pdf.min.js'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/2.16.105/pdf.worker.min.js',
    dest: 'public/vendor/pdfjs/pdf.worker.min.js'
  },
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/2.16.105/pdf_viewer.min.css',
    dest: 'public/vendor/pdfjs/pdf_viewer.min.css'
  },
  // Split.js
  {
    url: 'https://cdnjs.cloudflare.com/ajax/libs/split.js/1.6.5/split.min.js',
    dest: 'public/vendor/split/split.min.js'
  }
];

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const fullDest = path.join(__dirname, dest);
    fs.mkdirSync(path.dirname(fullDest), { recursive: true });
    const file = fs.createWriteStream(fullDest);
    
    https.get(url, (response) => {
      if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
        return download(response.headers.location, dest).then(resolve).catch(reject);
      }
      if (response.statusCode !== 200) {
        return reject(new Error(`Failed to download ${url}: Status ${response.statusCode}`));
      }
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        console.log(`Saved: ${dest}`);
        resolve();
      });
    }).on('error', (err) => {
      fs.unlink(fullDest, () => {});
      reject(err);
    });
  });
}

async function run() {
  console.log('Downloading vendor assets for 100% offline usage...');
  for (const item of vendors) {
    try {
      await download(item.url, item.dest);
    } catch (e) {
      console.error(`Error downloading ${item.url}:`, e.message);
    }
  }
  console.log('All vendor assets downloaded successfully!');
}

run();
