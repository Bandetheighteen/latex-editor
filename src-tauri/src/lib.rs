use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_workspace_dir() -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join("latex-editor-workspace");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn get_recent_projects_file() -> PathBuf {
    let base = if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join("com.latex.editor")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config").join("com.latex.editor")
    } else {
        std::env::temp_dir().join("com.latex.editor")
    };
    if !base.exists() {
        let _ = fs::create_dir_all(&base);
    }
    base.join("recent_projects.json")
}

fn load_recent_projects_internal() -> Vec<RecentProject> {
    let path = get_recent_projects_file();
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(mut list) = serde_json::from_str::<Vec<RecentProject>>(&data) {
            for item in &mut list {
                let p = Path::new(&item.path);
                item.exists = p.is_file();
                if let Ok(meta) = p.metadata() {
                    item.file_size = meta.len();
                }
            }
            list.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
            return list;
        }
    }
    Vec::new()
}

fn save_recent_projects_internal(list: &[RecentProject]) {
    let path = get_recent_projects_file();
    if let Ok(json) = serde_json::to_string_pretty(list) {
        let _ = fs::write(path, json);
    }
}

fn add_recent_project_internal(file_path: &str) {
    let path = Path::new(file_path);
    if !path.is_file() {
        return;
    }
    let mut list = load_recent_projects_internal();
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let file_size = path.metadata().map(|m| m.len()).unwrap_or(0);
    let ts = current_timestamp();

    list.retain(|item| item.path != file_path);
    list.insert(0, RecentProject {
        path: file_path.to_string(),
        name,
        last_opened: ts,
        file_size,
        exists: true,
    });
    if list.len() > 30 {
        list.truncate(30);
    }
    save_recent_projects_internal(&list);
}

#[allow(dead_code)]
fn create_system_command<P: AsRef<std::ffi::OsStr>>(bin: P) -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(bin);
    #[cfg(target_os = "macos")]
    {
        let current_path = std::env::var("PATH").unwrap_or_default();
        let mac_tex_paths = "/Library/TeX/texbin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin";
        cmd.env("PATH", format!("{}:{}", mac_tex_paths, current_path));
    }
    #[cfg(target_os = "linux")]
    {
        let current_path = std::env::var("PATH").unwrap_or_default();
        let linux_paths = "/usr/local/bin:/usr/bin:/bin";
        cmd.env("PATH", format!("{}:{}", linux_paths, current_path));
    }
    cmd
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentProject {
    pub path: String,
    pub name: String,
    pub last_opened: u64,
    pub file_size: u64,
    pub exists: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenFilePayload {
    pub path: String,
    pub name: String,
    pub content: String,
    pub last_modified: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveResult {
    pub success: bool,
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileResponse {
    pub pdf_buffer: Option<Vec<u8>>,
    pub log: Option<String>,
    pub error: Option<String>,
}

pub trait LatexEngine: Send + Sync {
    fn compile(&self, code: &str, project_path: Option<&str>, workspace_dir: &Path) -> Result<CompileResponse, String>;
    fn is_available(&self) -> bool;
    fn engine_name(&self) -> &'static str;
}

pub struct SystemLatexEngine;

impl LatexEngine for SystemLatexEngine {
    fn engine_name(&self) -> &'static str {
        "System pdflatex"
    }

    fn is_available(&self) -> bool {
        #[cfg(not(target_os = "android"))]
        {
            create_system_command("pdflatex").arg("--version").output().is_ok()
        }
        #[cfg(target_os = "android")]
        {
            false
        }
    }

    fn compile(&self, code: &str, project_path: Option<&str>, dir: &Path) -> Result<CompileResponse, String> {
        #[cfg(target_os = "android")]
        {
            let _ = (code, project_path, dir);
            Err("pdflatex is not supported on Android. Compilation runs entirely in-process.".into())
        }
        #[cfg(not(target_os = "android"))]
        {
            let tex_path = dir.join("document.tex");
            fs::write(&tex_path, code).map_err(|e| e.to_string())?;

            let mut cmd = create_system_command("pdflatex");

            if let Some(proj_file) = project_path {
                let p = PathBuf::from(proj_file);
                if let Some(parent) = p.parent() {
                    if parent.is_dir() {
                        cmd.current_dir(parent);
                    }
                }
            }

            let output = cmd
                .arg("-synctex=1")
                .arg("-interaction=nonstopmode")
                .arg("-halt-on-error")
                .arg(format!("-output-directory={}", dir.display()))
                .arg(&tex_path)
                .output()
                .map_err(|e| format!("Failed to spawn pdflatex: {}", e))?;

            let pdf_path = dir.join("document.pdf");
            let log_path = dir.join("document.log");

            let pdf_buffer = fs::read(&pdf_path).ok();
            let log_content = fs::read_to_string(&log_path).ok();

            Ok(CompileResponse {
                pdf_buffer,
                log: log_content,
                error: if !output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    None
                },
            })
        }
    }
}

pub fn get_tectonic_bin_path() -> PathBuf {
    let bin_name = if cfg!(windows) { "tectonic.exe" } else { "tectonic" };
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidates = [
                parent.join("resources").join("binaries").join(bin_name),
                parent.join("resources").join(bin_name),
                parent.join("../Resources").join("binaries").join(bin_name),
                parent.join("../Resources").join(bin_name),
                parent.join("binaries").join(bin_name),
                parent.join(bin_name),
            ];
            for p in &candidates {
                if p.exists() { return p.clone(); }
            }
        }
    }
    let dev_candidates = [
        PathBuf::from(format!("src-tauri/binaries/{}", bin_name)),
        PathBuf::from(format!("binaries/{}", bin_name)),
    ];
    for p in &dev_candidates {
        if p.exists() { return p.clone(); }
    }

    PathBuf::from(bin_name)
}

pub fn get_tectonic_bundle_path() -> Option<PathBuf> {
    let bundle_names = ["bundle.ttb", "texlive.ttb", "default.ttb"];
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidates = [
                parent.join("resources").join("bundles"),
                parent.join("resources"),
                parent.join("../Resources").join("bundles"),
                parent.join("../Resources"),
                parent.join("bundles"),
                parent.to_path_buf(),
            ];
            for dir in &candidates {
                for name in &bundle_names {
                    let p = dir.join(name);
                    if p.exists() { return Some(p); }
                }
            }
        }
    }
    let dev_candidates = [
        PathBuf::from("src-tauri/bundles"),
        PathBuf::from("src-tauri/binaries"),
        PathBuf::from("bundles"),
        PathBuf::from("binaries"),
    ];
    for dir in &dev_candidates {
        for name in &bundle_names {
            let p = dir.join(name);
            if p.exists() { return Some(p); }
        }
    }
    None
}

// ============================================================================
// In-Process Embedded LaTeX Compilation Engine (100% Offline, Pure Rust)
// ============================================================================

pub struct PdfPage {
    pub width: f32,
    pub height: f32,
    pub content: Vec<u8>,
}

impl PdfPage {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            content: Vec::new(),
        }
    }

    pub fn draw_text(&mut self, x: f32, y: f32, font: &str, size: f32, text: &str) {
        if text.is_empty() {
            return;
        }
        let escaped = escape_pdf_str(text);
        let cmd = format!("BT /{} {:.2} Tf {:.2} {:.2} Td ({}) Tj ET\n", font, size, x, y, escaped);
        self.content.extend_from_slice(cmd.as_bytes());
    }

    pub fn draw_text_colored(&mut self, x: f32, y: f32, font: &str, size: f32, text: &str, r: f32, g: f32, b: f32) {
        if text.is_empty() {
            return;
        }
        let escaped = escape_pdf_str(text);
        let cmd = format!("BT {:.2} {:.2} {:.2} rg /{} {:.2} Tf {:.2} {:.2} Td ({}) Tj ET\n", r, g, b, font, size, x, y, escaped);
        self.content.extend_from_slice(cmd.as_bytes());
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32, r: f32, g: f32, b: f32) {
        let cmd = format!("{:.2} w {:.2} {:.2} {:.2} RG {:.2} {:.2} m {:.2} {:.2} l S\n", width, r, g, b, x1, y1, x2, y2);
        self.content.extend_from_slice(cmd.as_bytes());
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32) {
        let cmd = format!("{:.2} {:.2} {:.2} rg {:.2} {:.2} {:.2} {:.2} re f\n", r, g, b, x, y, w, h);
        self.content.extend_from_slice(cmd.as_bytes());
    }
}

pub fn escape_pdf_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for ch in s.chars() {
        match ch {
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            '\r' => {},
            '\n' => out.push(' '),
            _ => {
                let code = ch as u32;
                if code >= 32 && code <= 126 {
                    out.push(ch);
                } else if ch == '•' || ch == '–' || ch == '—' {
                    out.push('-');
                } else if ch == '“' || ch == '”' {
                    out.push('"');
                } else if ch == '‘' || ch == '’' {
                    out.push('\'');
                } else if ch == '…' {
                    out.push_str("...");
                } else if ch == '≠' {
                    out.push_str("!=");
                } else if ch == '≤' {
                    out.push_str("<=");
                } else if ch == '≥' {
                    out.push_str(">=");
                } else if ch == '≈' {
                    out.push('~');
                } else if ch == '±' {
                    out.push_str("+/-");
                } else if ch == '×' {
                    out.push('*');
                } else if ch == '÷' {
                    out.push('/');
                } else if ch == '∞' {
                    out.push_str("inf");
                } else if ch == '∂' {
                    out.push('d');
                } else if ch == '→' {
                    out.push_str("->");
                } else if ch == '⇒' {
                    out.push_str("=>");
                } else if ch == '∈' {
                    out.push_str("in");
                } else if ch == 'π' {
                    out.push_str("pi");
                } else if ch == 'α' {
                    out.push_str("alpha");
                } else if ch == 'β' {
                    out.push_str("beta");
                } else if ch == 'γ' {
                    out.push_str("gamma");
                } else if ch == 'θ' {
                    out.push_str("theta");
                } else if ch == 'λ' {
                    out.push_str("lambda");
                } else if ch == 'σ' {
                    out.push_str("sigma");
                } else if ch == 'ω' {
                    out.push_str("omega");
                } else if ch == 'Δ' {
                    out.push_str("Delta");
                } else if ch == '∑' {
                    out.push_str("SUM");
                } else if ch == '∫' {
                    out.push_str("INT");
                } else if ch == '√' {
                    out.push_str("sqrt");
                } else if ch == 'Ψ' {
                    out.push_str("Psi");
                } else if ch == 'ħ' {
                    out.push_str("hbar");
                } else if ch == 'Ĥ' {
                    out.push_str("H_hat");
                } else if code >= 128 && code <= 255 {
                    out.push_str(&format!("\\{:03o}", code));
                } else {
                    out.push(ch);
                }
            }
        }
    }
    out
}

pub fn approx_text_width(text: &str, font_size: f32, is_mono: bool) -> f32 {
    if is_mono {
        return text.len() as f32 * font_size * 0.60;
    }
    let mut w = 0.0;
    for ch in text.chars() {
        w += match ch {
            'i' | 'l' | 'j' | '!' | '|' | ':' | ';' | '\'' | '.' | ',' => 0.28,
            'f' | 'r' | 't' | '(' | ')' | '[' | ']' | ' ' => 0.35,
            'm' | 'w' | 'M' | 'W' => 0.85,
            'A'..='Z' => 0.68,
            _ => 0.52,
        } * font_size;
    }
    w
}

fn extract_braced(input: &str, start_pos: usize) -> Option<(&str, usize)> {
    let bytes = input.as_bytes();
    if start_pos >= bytes.len() || bytes[start_pos] != b'{' {
        return None;
    }
    let mut depth = 0;
    let mut i = start_pos;
    let content_start = start_pos + 1;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            depth += 1;
        } else if bytes[i] == b'}' {
            depth -= 1;
            if depth == 0 {
                return Some((&input[content_start..i], i + 1));
            }
        }
        i += 1;
    }
    None
}

fn extract_command_arg<'a>(code: &'a str, cmd: &str) -> Option<&'a str> {
    if let Some(pos) = code.find(cmd) {
        let after = &code[pos + cmd.len()..];
        if let Some(brace_rel) = after.find('{') {
            let brace_pos = pos + cmd.len() + brace_rel;
            if let Some((content, _)) = extract_braced(code, brace_pos) {
                return Some(content);
            }
        }
    }
    None
}

fn read_optional_bracketed(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    while let Some(&c) = chars.peek() {
        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            chars.next();
        } else {
            break;
        }
    }
    if chars.peek() != Some(&'[') {
        return None;
    }
    chars.next(); // consume '['
    let mut depth = 1;
    let mut res = String::new();
    while let Some(c) = chars.next() {
        if c == '[' {
            depth += 1;
            res.push(c);
        } else if c == ']' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            res.push(c);
        } else {
            res.push(c);
        }
    }
    Some(res)
}

fn read_next_braced(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    while let Some(&c) = chars.peek() {
        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            chars.next();
        } else {
            break;
        }
    }
    if chars.peek() != Some(&'{') {
        if let Some(c) = chars.next() {
            return c.to_string();
        }
        return String::new();
    }
    chars.next(); // consume '{'
    let mut depth = 1;
    let mut res = String::new();
    while let Some(c) = chars.next() {
        if c == '{' {
            depth += 1;
            res.push(c);
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            res.push(c);
        } else {
            res.push(c);
        }
    }
    res
}

pub fn clean_latex_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '\\' {
                    chars.next(); // consume second '\'
                    let _ = read_optional_bracketed(&mut chars); // e.g. \\[4pt]
                    out.push('\n');
                    continue;
                } else if next_ch == '%' || next_ch == '&' || next_ch == '$' || next_ch == '#' || next_ch == '_' || next_ch == '{' || next_ch == '}' {
                    out.push(chars.next().unwrap());
                    continue;
                } else if next_ch == ',' || next_ch == ';' || next_ch == ' ' {
                    chars.next();
                    out.push(' ');
                    continue;
                } else if next_ch == '!' {
                    chars.next();
                    continue;
                }
            }

            let mut cmd = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphabetic() || next_ch == '*' {
                    cmd.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            match cmd.as_str() {
                "textbf" | "textit" | "emph" | "texttt" | "underline" | "text" | "mathbf" | "mathit" | "mathrm" | "mathbb" | "mathcal" | "bm" | "hat" | "vec" | "bar" | "dot" | "ddot" | "tilde" => {
                    let inner = read_next_braced(&mut chars);
                    out.push_str(&clean_latex_text(&inner));
                }
                "Huge" | "huge" | "LARGE" | "Large" | "large" | "normalsize" | "small" | "footnotesize" | "tiny" | "bfseries" | "itshape" | "scshape" => {}
                "today" => out.push_str("November 1, 2026"),
                "textbar" => out.push('|'),
                "newline" | "par" => out.push('\n'),
                "hfill" | "quad" => out.push_str("   "),
                "qquad" => out.push_str("      "),
                "LaTeX" => out.push_str("LaTeX"),
                "TeX" => out.push_str("TeX"),
                "frac" => {
                    let num = read_next_braced(&mut chars);
                    let den = read_next_braced(&mut chars);
                    out.push_str(&format!("{}/{}", clean_latex_text(&num), clean_latex_text(&den)));
                }
                "sqrt" => {
                    let _ = read_optional_bracketed(&mut chars);
                    let rad = read_next_braced(&mut chars);
                    out.push_str(&format!("sqrt({})", clean_latex_text(&rad)));
                }
                // Functions
                "sin" | "cos" | "tan" | "sec" | "csc" | "cot" | "arcsin" | "arccos" | "arctan" | "sinh" | "cosh" | "tanh" | "exp" | "log" | "ln" | "lim" | "max" | "min" | "det" | "sup" | "inf" | "gcd" | "deg" | "Pr" => {
                    out.push_str(&cmd);
                    out.push(' ');
                }
                // Greek lowercase
                "alpha" => out.push_str("alpha"),
                "beta" => out.push_str("beta"),
                "gamma" => out.push_str("gamma"),
                "delta" => out.push_str("delta"),
                "epsilon" | "varepsilon" => out.push_str("epsilon"),
                "zeta" => out.push_str("zeta"),
                "eta" => out.push_str("eta"),
                "theta" | "vartheta" => out.push_str("theta"),
                "iota" => out.push_str("iota"),
                "kappa" => out.push_str("kappa"),
                "lambda" => out.push_str("lambda"),
                "mu" => out.push_str("mu"),
                "nu" => out.push_str("nu"),
                "xi" => out.push_str("xi"),
                "pi" | "varpi" => out.push_str("pi"),
                "rho" | "varrho" => out.push_str("rho"),
                "sigma" | "varsigma" => out.push_str("sigma"),
                "tau" => out.push_str("tau"),
                "upsilon" => out.push_str("upsilon"),
                "phi" | "varphi" => out.push_str("phi"),
                "chi" => out.push_str("chi"),
                "psi" => out.push_str("psi"),
                "omega" => out.push_str("omega"),
                // Greek uppercase
                "Gamma" => out.push_str("Gamma"),
                "Delta" => out.push_str("Delta"),
                "Theta" => out.push_str("Theta"),
                "Lambda" => out.push_str("Lambda"),
                "Xi" => out.push_str("Xi"),
                "Pi" => out.push_str("Pi"),
                "Sigma" => out.push_str("Sigma"),
                "Upsilon" => out.push_str("Upsilon"),
                "Phi" => out.push_str("Phi"),
                "Psi" => out.push_str("Psi"),
                "Omega" => out.push_str("Omega"),
                // Calculus & Operators
                "sum" => out.push_str("SUM "),
                "prod" => out.push_str("PROD "),
                "int" => out.push_str("INT "),
                "iint" => out.push_str("INT_INT "),
                "iiint" => out.push_str("INT_INT_INT "),
                "oint" => out.push_str("OINT "),
                "nabla" => out.push_str("grad "),
                "partial" => out.push('d'),
                "cdot" => out.push_str(" * "),
                "times" => out.push_str(" * "),
                "div" => out.push_str(" / "),
                "pm" => out.push_str(" +/- "),
                "mp" => out.push_str(" -/+ "),
                "infty" => out.push_str("inf"),
                "hbar" => out.push_str("hbar"),
                "ell" => out.push('l'),
                "Re" => out.push_str("Re"),
                "Im" => out.push_str("Im"),
                "neq" | "ne" => out.push_str(" != "),
                "leq" | "le" => out.push_str(" <= "),
                "geq" | "ge" => out.push_str(" >= "),
                "approx" => out.push_str(" ~ "),
                "equiv" => out.push_str(" == "),
                "sim" => out.push_str(" ~ "),
                "to" | "rightarrow" => out.push_str(" -> "),
                "leftarrow" => out.push_str(" <- "),
                "implies" | "Rightarrow" => out.push_str(" => "),
                "Leftarrow" => out.push_str(" <= "),
                "iff" | "Leftrightarrow" => out.push_str(" <=> "),
                "in" => out.push_str(" in "),
                "notin" => out.push_str(" not in "),
                "subset" => out.push_str(" subset "),
                "subseteq" => out.push_str(" subseteq "),
                "cup" => out.push_str(" cup "),
                "cap" => out.push_str(" cap "),
                "forall" => out.push_str("forall "),
                "exists" => out.push_str("exists "),
                // Environments / setup tags
                "begin" | "end" => {
                    let _ = read_next_braced(&mut chars);
                }
                "documentclass" | "usepackage" | "geometry" | "usetheme" | "usecolortheme" | "pagestyle" | "titleformat" | "setlength" | "newcommand" | "renewcommand" | "definecolor" => {
                    let _ = read_optional_bracketed(&mut chars);
                    let _ = read_next_braced(&mut chars);
                }
                "titlerule" | "tableofcontents" | "maketitle" | "titlepage" => {}
                "vspace" | "hspace" => {
                    let _ = read_optional_bracketed(&mut chars);
                    let _ = read_next_braced(&mut chars);
                }
                "item" => out.push_str("- "),
                "href" => {
                    let _ = read_next_braced(&mut chars);
                    let label = read_next_braced(&mut chars);
                    out.push_str(&clean_latex_text(&label));
                }
                "url" => {
                    let u = read_next_braced(&mut chars);
                    out.push_str(&clean_latex_text(&u));
                }
                "cite" | "ref" => {
                    let r = read_next_braced(&mut chars);
                    out.push_str(&format!("[{}]", r));
                }
                _ => {
                    if cmd.is_empty() {
                        if let Some(&c) = chars.peek() {
                            out.push(c);
                            chars.next();
                        }
                    }
                }
            }
        } else if ch == '{' || ch == '}' || ch == '$' {
            continue;
        } else if ch == '&' {
            out.push_str("  ");
        } else if ch == '-' && chars.peek() == Some(&'-') {
            chars.next();
            if chars.peek() == Some(&'-') {
                chars.next();
            }
            out.push('-');
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn assemble_pdf(pages: &[PdfPage]) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut offsets = Vec::new();

    buf.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    let num_pages = pages.len().max(1);

    // 1 0 obj Catalog
    offsets.push(buf.len());
    buf.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    // 2 0 obj Pages
    offsets.push(buf.len());
    let mut kids = String::from("[");
    for i in 0..num_pages {
        let page_obj_id = 10 + 2 * i;
        kids.push_str(&format!(" {} 0 R", page_obj_id));
    }
    kids.push_str(" ]");
    buf.extend_from_slice(format!("2 0 obj\n<< /Type /Pages /Kids {} /Count {} >>\nendobj\n", kids, num_pages).as_bytes());

    // Fonts
    offsets.push(buf.len());
    buf.extend_from_slice(b"3 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>\nendobj\n");
    offsets.push(buf.len());
    buf.extend_from_slice(b"4 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>\nendobj\n");
    offsets.push(buf.len());
    buf.extend_from_slice(b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique /Encoding /WinAnsiEncoding >>\nendobj\n");
    offsets.push(buf.len());
    buf.extend_from_slice(b"6 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Courier /Encoding /WinAnsiEncoding >>\nendobj\n");
    offsets.push(buf.len());
    buf.extend_from_slice(b"7 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /WinAnsiEncoding >>\nendobj\n");
    offsets.push(buf.len());
    buf.extend_from_slice(b"8 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Times-Bold /Encoding /WinAnsiEncoding >>\nendobj\n");

    // 9 0 obj Resources
    offsets.push(buf.len());
    buf.extend_from_slice(b"9 0 obj\n<< /Font << /F1 3 0 R /F2 4 0 R /F3 5 0 R /F4 6 0 R /F5 7 0 R /F6 8 0 R >> /ProcSet [ /PDF /Text /ImageB /ImageC /ImageI ] >>\nendobj\n");

    // Pages and Content streams
    for (i, p) in pages.iter().enumerate() {
        let page_id = 10 + 2 * i;
        let content_id = 11 + 2 * i;

        // Page object
        offsets.push(buf.len());
        buf.extend_from_slice(format!(
            "{} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {:.2} {:.2}] /Resources 9 0 R /Contents {} 0 R >>\nendobj\n",
            page_id, p.width, p.height, content_id
        ).as_bytes());

        // Content stream
        offsets.push(buf.len());
        buf.extend_from_slice(format!(
            "{} 0 obj\n<< /Length {} >>\nstream\n",
            content_id, p.content.len()
        ).as_bytes());
        buf.extend_from_slice(&p.content);
        buf.extend_from_slice(b"\nendstream\nendobj\n");
    }

    // xref table
    let xref_offset = buf.len();
    let total_objs = offsets.len() + 1;
    buf.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", total_objs).as_bytes());
    for off in &offsets {
        buf.extend_from_slice(format!("{:010} 00000 n \n", off).as_bytes());
    }

    buf.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", total_objs, xref_offset).as_bytes());

    buf
}

pub fn compile_latex_in_process(code: &str) -> Result<Vec<u8>, String> {
    if code.trim().is_empty() {
        return Err("LaTeX document code is empty. Type some content to compile.".into());
    }

    let is_beamer = code.contains("documentclass{beamer}") || (code.contains("documentclass") && code.contains("beamer"));

    let (page_width, page_height, margin_x, margin_top, margin_bottom) = if is_beamer {
        (720.0f32, 405.0f32, 40.0f32, 46.0f32, 30.0f32)
    } else {
        (595.28f32, 841.89f32, 54.0f32, 54.0f32, 54.0f32)
    };

    let title_raw = extract_command_arg(code, r"\title");
    let author_raw = extract_command_arg(code, r"\author");
    let date_raw = extract_command_arg(code, r"\date");
    let subtitle_raw = extract_command_arg(code, r"\subtitle");

    let title = title_raw.map(clean_latex_text);
    let author = author_raw.map(clean_latex_text);
    let date = date_raw.map(|d| {
        let cleaned = clean_latex_text(d);
        if cleaned.trim() == r"\today" || cleaned.trim().is_empty() {
            "November 1, 2026".to_string()
        } else {
            cleaned
        }
    }).unwrap_or_else(|| "November 1, 2026".to_string());
    let subtitle = subtitle_raw.map(clean_latex_text);

    // Extract body content between \begin{document} and \end{document}
    let body = if let Some(start) = code.find(r"\begin{document}") {
        let after = &code[start + r"\begin{document}".len()..];
        if let Some(end) = after.find(r"\end{document}") {
            &after[..end]
        } else {
            after
        }
    } else {
        code
    };

    let mut pages: Vec<PdfPage> = Vec::new();
    let mut cur_page = PdfPage::new(page_width, page_height);
    let mut cur_y = page_height - margin_top;
    let content_width = page_width - 2.0 * margin_x;

    let mut section_num = 0;
    let mut subsection_num = 0;
    let mut equation_num = 0;
    let mut beamer_slide_num = 0;

    let mut new_slide = |pages: &mut Vec<PdfPage>, cur_page: &mut PdfPage, cur_y: &mut f32, frame_title: &str| {
        if cur_page.content.len() > 10 {
            let mut empty = PdfPage::new(page_width, page_height);
            std::mem::swap(cur_page, &mut empty);
            pages.push(empty);
        }
        beamer_slide_num += 1;
        cur_page.fill_rect(0.0, page_height - 38.0, page_width, 38.0, 0.12, 0.23, 0.53);
        if !frame_title.is_empty() {
            cur_page.draw_text_colored(24.0, page_height - 25.0, "F2", 14.0, frame_title, 1.0, 1.0, 1.0);
        }
        cur_page.fill_rect(0.0, 0.0, page_width, 20.0, 0.08, 0.16, 0.36);
        if let Some(ref a) = author {
            cur_page.draw_text_colored(20.0, 6.0, "F1", 8.0, a, 0.85, 0.88, 0.95);
        }
        if let Some(ref t) = title {
            let tw = approx_text_width(t, 8.0, false);
            let tx = (page_width - tw) / 2.0;
            cur_page.draw_text_colored(tx, 6.0, "F2", 8.0, t, 0.9, 0.92, 0.98);
        }
        let slide_str = format!("{}", beamer_slide_num);
        cur_page.draw_text_colored(page_width - 32.0, 6.0, "F1", 8.0, &slide_str, 0.85, 0.88, 0.95);
        *cur_y = page_height - 56.0;
    };

    let check_break = |pages: &mut Vec<PdfPage>, cur_page: &mut PdfPage, cur_y: &mut f32, needed: f32| {
        if *cur_y - needed < margin_bottom {
            let mut empty = PdfPage::new(page_width, page_height);
            std::mem::swap(cur_page, &mut empty);
            pages.push(empty);
            *cur_y = page_height - margin_top;
        }
    };

    let draw_wrapped_text = |pages: &mut Vec<PdfPage>, cur_page: &mut PdfPage, cur_y: &mut f32, text: &str, font: &str, font_size: f32, indent: f32, center: bool| {
        let max_w = content_width - indent;
        let line_h = font_size * 1.35;
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return;
        }
        let mut cur_line = String::new();
        for w in words {
            let test = if cur_line.is_empty() { w.to_string() } else { format!("{} {}", cur_line, w) };
            if approx_text_width(&test, font_size, false) > max_w && !cur_line.is_empty() {
                check_break(pages, cur_page, cur_y, line_h);
                let x = if center {
                    let lw = approx_text_width(&cur_line, font_size, false);
                    (page_width - lw) / 2.0
                } else {
                    margin_x + indent
                };
                cur_page.draw_text(x, *cur_y, font, font_size, &cur_line);
                *cur_y -= line_h;
                cur_line = w.to_string();
            } else {
                cur_line = test;
            }
        }
        if !cur_line.is_empty() {
            check_break(pages, cur_page, cur_y, line_h);
            let x = if center {
                let lw = approx_text_width(&cur_line, font_size, false);
                (page_width - lw) / 2.0
            } else {
                margin_x + indent
            };
            cur_page.draw_text(x, *cur_y, font, font_size, &cur_line);
            *cur_y -= line_h;
        }
    };

    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let raw_line = lines[i];
        let trimmed = raw_line.trim();

        if trimmed.starts_with('%') {
            i += 1;
            continue;
        }
        if trimmed.is_empty() {
            cur_y -= 8.0;
            i += 1;
            continue;
        }

        // Preamble remnants
        if trimmed.starts_with(r"\documentclass") || trimmed.starts_with(r"\usepackage") || trimmed.starts_with(r"\geometry") || trimmed.starts_with(r"\usetheme") || trimmed.starts_with(r"\usecolortheme") || trimmed.starts_with(r"\pagestyle") || trimmed.starts_with(r"\titleformat") || trimmed.starts_with(r"\newcommand") || trimmed.starts_with(r"\renewcommand") || trimmed.starts_with(r"\definecolor") || trimmed.starts_with(r"\title{") || trimmed.starts_with(r"\author{") || trimmed.starts_with(r"\date{") || trimmed.starts_with(r"\subtitle{") || trimmed.starts_with(r"\tableofcontents") || trimmed.starts_with(r"\titlerule") {
            i += 1;
            continue;
        }

        // Title page / maketitle
        if trimmed.contains(r"\maketitle") || trimmed.contains(r"\titlepage") {
            if is_beamer {
                new_slide(&mut pages, &mut cur_page, &mut cur_y, "");
            }
            if let Some(ref t) = title {
                let sz = if is_beamer { 22.0 } else { 20.0 };
                let tw = approx_text_width(t, sz, false);
                let tx = ((page_width - tw) / 2.0).max(margin_x);
                cur_page.draw_text(tx, cur_y, "F2", sz, t);
                cur_y -= sz * 1.35;
            }
            if let Some(ref s) = subtitle {
                let sz = 13.0;
                let sw = approx_text_width(s, sz, false);
                let sx = ((page_width - sw) / 2.0).max(margin_x);
                cur_page.draw_text(sx, cur_y, "F3", sz, s);
                cur_y -= sz * 1.3;
            }
            if let Some(ref a) = author {
                let sz = 12.0;
                let aw = approx_text_width(a, sz, false);
                let ax = ((page_width - aw) / 2.0).max(margin_x);
                cur_page.draw_text(ax, cur_y, "F1", sz, a);
                cur_y -= sz * 1.3;
            }
            let dw = approx_text_width(&date, 10.0, false);
            let dx = ((page_width - dw) / 2.0).max(margin_x);
            cur_page.draw_text(dx, cur_y, "F3", 10.0, &date);
            cur_y -= 14.0;

            if !is_beamer {
                cur_page.draw_line(margin_x, cur_y, page_width - margin_x, cur_y, 0.75, 0.7, 0.7, 0.7);
                cur_y -= 18.0;
            }
            i += 1;
            continue;
        }

        // Beamer frame
        if trimmed.starts_with(r"\begin{frame}") {
            let mut frame_title = String::new();
            if let Some(second_brace) = trimmed.find("}{") {
                if let Some((t2, _)) = extract_braced(&trimmed[second_brace + 1..], 0) {
                    frame_title = clean_latex_text(t2);
                }
            } else if let Some(first_brace) = trimmed.find('{') {
                if let Some((content, _)) = extract_braced(trimmed, first_brace) {
                    if content != "frame" {
                        frame_title = clean_latex_text(content);
                    }
                }
            }
            new_slide(&mut pages, &mut cur_page, &mut cur_y, &frame_title);
            i += 1;
            continue;
        }
        if trimmed.starts_with(r"\end{frame}") {
            i += 1;
            continue;
        }

        // Block (in beamer)
        if trimmed.starts_with(r"\begin{block}") {
            let block_title = extract_command_arg(trimmed, r"\begin{block}").map(clean_latex_text).unwrap_or_default();
            check_break(&mut pages, &mut cur_page, &mut cur_y, 35.0);
            cur_page.fill_rect(margin_x, cur_y - 20.0, content_width, 22.0, 0.16, 0.32, 0.65);
            cur_page.draw_text_colored(margin_x + 10.0, cur_y - 14.0, "F2", 11.0, &block_title, 1.0, 1.0, 1.0);
            cur_y -= 28.0;
            i += 1;
            continue;
        }
        if trimmed.starts_with(r"\end{block}") {
            cur_y -= 8.0;
            i += 1;
            continue;
        }

        // Abstract
        if trimmed.starts_with(r"\begin{abstract}") {
            check_break(&mut pages, &mut cur_page, &mut cur_y, 25.0);
            let aw = approx_text_width("Abstract", 11.0, false);
            let ax = (page_width - aw) / 2.0;
            cur_page.draw_text(ax, cur_y, "F2", 11.0, "Abstract");
            cur_y -= 15.0;
            i += 1;
            let mut abs_text = String::new();
            while i < lines.len() && !lines[i].trim().starts_with(r"\end{abstract}") {
                let l = lines[i].trim();
                if !l.starts_with('%') && !l.is_empty() {
                    if !abs_text.is_empty() { abs_text.push(' '); }
                    abs_text.push_str(l);
                }
                i += 1;
            }
            if i < lines.len() { i += 1; }
            let cleaned = clean_latex_text(&abs_text);
            draw_wrapped_text(&mut pages, &mut cur_page, &mut cur_y, &cleaned, "F3", 10.0, 24.0, false);
            cur_y -= 12.0;
            continue;
        }

        // Center environment
        if trimmed.starts_with(r"\begin{center}") {
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with(r"\end{center}") {
                let l = lines[i].trim();
                if !l.starts_with('%') && !l.is_empty() {
                    let cleaned = clean_latex_text(l);
                    for sub in cleaned.split('\n') {
                        let sub_t = sub.trim();
                        if !sub_t.is_empty() {
                            draw_wrapped_text(&mut pages, &mut cur_page, &mut cur_y, sub_t, "F1", 11.0, 0.0, true);
                            cur_y -= 4.0;
                        }
                    }
                }
                i += 1;
            }
            if i < lines.len() { i += 1; }
            cur_y -= 8.0;
            continue;
        }

        // Multi-line Equation: \begin{equation}, \begin{equation*}, $$, \[
        if trimmed.starts_with(r"\begin{equation}") || trimmed.starts_with(r"\begin{equation*}") || trimmed.starts_with("$$") || trimmed.starts_with(r"\[") {
            let is_star = trimmed.starts_with(r"\begin{equation*}") || trimmed.starts_with(r"\[");
            let end_token = if trimmed.starts_with(r"\begin{equation*}") {
                r"\end{equation*}"
            } else if trimmed.starts_with(r"\begin{equation}") {
                r"\end{equation}"
            } else if trimmed.starts_with("$$") {
                "$$"
            } else {
                r"\]"
            };

            let mut eq_raw = String::new();
            if trimmed.starts_with("$$") && trimmed.len() > 2 && trimmed[2..].contains("$$") {
                eq_raw.push_str(trimmed.trim_matches('$'));
                i += 1;
            } else {
                let start_line = if trimmed.starts_with(r"\begin{equation}") {
                    trimmed.trim_start_matches(r"\begin{equation}").trim()
                } else if trimmed.starts_with(r"\begin{equation*}") {
                    trimmed.trim_start_matches(r"\begin{equation*}").trim()
                } else if trimmed.starts_with(r"\[") {
                    trimmed.trim_start_matches(r"\[").trim()
                } else {
                    trimmed.trim_start_matches('$').trim()
                };
                if !start_line.is_empty() {
                    eq_raw.push_str(start_line);
                }
                i += 1;
                while i < lines.len() {
                    let l = lines[i].trim();
                    if l.contains(end_token) {
                        let before_end = l.split(end_token).next().unwrap_or("").trim();
                        if !before_end.is_empty() {
                            if !eq_raw.is_empty() { eq_raw.push(' '); }
                            eq_raw.push_str(before_end);
                        }
                        i += 1;
                        break;
                    } else if !l.starts_with('%') {
                        if !eq_raw.is_empty() { eq_raw.push(' '); }
                        eq_raw.push_str(l);
                    }
                    i += 1;
                }
            }

            let cleaned_eq = clean_latex_text(&eq_raw);
            let eq_str = cleaned_eq.trim();
            if !eq_str.is_empty() {
                check_break(&mut pages, &mut cur_page, &mut cur_y, 24.0);
                cur_y -= 6.0;
                let ew = approx_text_width(eq_str, 11.5, false);
                let ex = ((page_width - ew) / 2.0).max(margin_x);
                cur_page.draw_text(ex, cur_y, "F5", 11.5, eq_str);
                if !is_star {
                    equation_num += 1;
                    let num_str = format!("({})", equation_num);
                    let nw = approx_text_width(&num_str, 10.0, false);
                    cur_page.draw_text(page_width - margin_x - nw, cur_y, "F1", 10.0, &num_str);
                }
                cur_y -= 18.0;
            }
            continue;
        }

        // Multi-line Align / Gather: \begin{align}, \begin{align*}, \begin{gather}
        if trimmed.starts_with(r"\begin{align}") || trimmed.starts_with(r"\begin{align*}") || trimmed.starts_with(r"\begin{gather}") || trimmed.starts_with(r"\begin{gather*}") {
            let is_star = trimmed.contains('*');
            let end_token = if trimmed.starts_with(r"\begin{align*}") {
                r"\end{align*}"
            } else if trimmed.starts_with(r"\begin{align}") {
                r"\end{align}"
            } else if trimmed.starts_with(r"\begin{gather*}") {
                r"\end{gather*}"
            } else {
                r"\end{gather}"
            };
            i += 1;
            let mut align_raw = String::new();
            while i < lines.len() && !lines[i].trim().contains(end_token) {
                let l = lines[i].trim();
                if !l.starts_with('%') {
                    if !align_raw.is_empty() { align_raw.push('\n'); }
                    align_raw.push_str(l);
                }
                i += 1;
            }
            if i < lines.len() { i += 1; }

            let sub_eqs: Vec<&str> = align_raw.split(r"\\").collect();
            for sub in sub_eqs {
                let cleaned = clean_latex_text(sub);
                let eq_line = cleaned.trim();
                if !eq_line.is_empty() {
                    check_break(&mut pages, &mut cur_page, &mut cur_y, 22.0);
                    cur_y -= 4.0;
                    let ew = approx_text_width(eq_line, 11.0, false);
                    let ex = ((page_width - ew) / 2.0).max(margin_x);
                    cur_page.draw_text(ex, cur_y, "F5", 11.0, eq_line);
                    if !is_star {
                        equation_num += 1;
                        let num_str = format!("({})", equation_num);
                        let nw = approx_text_width(&num_str, 10.0, false);
                        cur_page.draw_text(page_width - margin_x - nw, cur_y, "F1", 10.0, &num_str);
                    }
                    cur_y -= 16.0;
                }
            }
            cur_y -= 6.0;
            continue;
        }

        // Lists: Itemize & Enumerate
        if trimmed.starts_with(r"\begin{itemize}") || trimmed.starts_with(r"\begin{enumerate}") {
            let is_enum = trimmed.starts_with(r"\begin{enumerate}");
            let end_token = if is_enum { r"\end{enumerate}" } else { r"\end{itemize}" };
            i += 1;
            let mut list_counter = 0;
            while i < lines.len() && !lines[i].trim().starts_with(end_token) {
                let l = lines[i].trim();
                if l.starts_with(r"\item") {
                    let mut item_text = l.trim_start_matches(r"\item").trim().to_string();
                    i += 1;
                    while i < lines.len() {
                        let next_l = lines[i].trim();
                        if next_l.starts_with(r"\item") || next_l.starts_with(end_token) {
                            break;
                        }
                        if !next_l.starts_with('%') && !next_l.is_empty() {
                            item_text.push(' ');
                            item_text.push_str(next_l);
                        }
                        i += 1;
                    }
                    let cleaned = clean_latex_text(&item_text);
                    let bullet = if is_enum {
                        list_counter += 1;
                        format!("{}. ", list_counter)
                    } else {
                        "- ".to_string()
                    };
                    let full_item = format!("{}{}", bullet, cleaned);
                    draw_wrapped_text(&mut pages, &mut cur_page, &mut cur_y, &full_item, "F1", 11.0, 16.0, false);
                    cur_y -= 4.0;
                } else {
                    i += 1;
                }
            }
            if i < lines.len() { i += 1; }
            cur_y -= 6.0;
            continue;
        }

        // Section hierarchy
        if trimmed.starts_with(r"\section{") || trimmed.starts_with(r"\section*{") {
            let is_star = trimmed.starts_with(r"\section*{");
            let sec_text = extract_command_arg(trimmed, if is_star { r"\section*" } else { r"\section" }).map(clean_latex_text).unwrap_or_default();
            check_break(&mut pages, &mut cur_page, &mut cur_y, 34.0);
            cur_y -= 12.0;
            let display_title = if is_star {
                sec_text
            } else {
                section_num += 1;
                subsection_num = 0;
                format!("{}  {}", section_num, sec_text)
            };
            cur_page.draw_text(margin_x, cur_y, "F2", 14.0, &display_title);
            cur_y -= 8.0;
            cur_page.draw_line(margin_x, cur_y, page_width - margin_x, cur_y, 0.5, 0.8, 0.8, 0.8);
            cur_y -= 14.0;
            i += 1;
            continue;
        }

        if trimmed.starts_with(r"\subsection{") || trimmed.starts_with(r"\subsection*{") {
            let is_star = trimmed.starts_with(r"\subsection*{");
            let sub_text = extract_command_arg(trimmed, if is_star { r"\subsection*" } else { r"\subsection" }).map(clean_latex_text).unwrap_or_default();
            check_break(&mut pages, &mut cur_page, &mut cur_y, 24.0);
            cur_y -= 8.0;
            let display_title = if is_star {
                sub_text
            } else {
                subsection_num += 1;
                format!("{}.{}  {}", section_num, subsection_num, sub_text)
            };
            cur_page.draw_text(margin_x, cur_y, "F2", 12.0, &display_title);
            cur_y -= 15.0;
            i += 1;
            continue;
        }

        if trimmed.starts_with(r"\subsubsection{") {
            let ssub_text = extract_command_arg(trimmed, r"\subsubsection").map(clean_latex_text).unwrap_or_default();
            check_break(&mut pages, &mut cur_page, &mut cur_y, 20.0);
            cur_page.draw_text(margin_x, cur_y, "F2", 11.0, &ssub_text);
            cur_y -= 14.0;
            i += 1;
            continue;
        }

        // Two-column \hfill row (resume header/entries)
        if trimmed.contains(r"\hfill") {
            let parts: Vec<&str> = trimmed.split(r"\hfill").collect();
            if parts.len() == 2 {
                let left_clean = clean_latex_text(parts[0].trim());
                let right_clean = clean_latex_text(parts[1].trim());
                check_break(&mut pages, &mut cur_page, &mut cur_y, 16.0);
                cur_page.draw_text(margin_x, cur_y, "F2", 11.0, &left_clean);
                let rw = approx_text_width(&right_clean, 10.5, false);
                cur_page.draw_text(page_width - margin_x - rw, cur_y, "F1", 10.5, &right_clean);
                cur_y -= 16.0;
                i += 1;
                continue;
            }
        }

        // Regular paragraph: gather contiguous lines until empty line or command
        let mut para = String::new();
        while i < lines.len() {
            let l = lines[i].trim();
            if l.is_empty() || l.starts_with('%') || l.starts_with(r"\section") || l.starts_with(r"\subsection") || l.starts_with(r"\begin") || l.starts_with(r"\end") || l.starts_with(r"\item") || l.starts_with(r"\maketitle") || l.starts_with(r"\titlepage") || l.contains(r"\hfill") {
                break;
            }
            if !para.is_empty() { para.push(' '); }
            para.push_str(l);
            i += 1;
        }

        if !para.is_empty() {
            let cleaned = clean_latex_text(&para);
            for sub in cleaned.split('\n') {
                let sub_t = sub.trim();
                if !sub_t.is_empty() {
                    draw_wrapped_text(&mut pages, &mut cur_page, &mut cur_y, sub_t, "F1", 11.0, 0.0, false);
                    cur_y -= 6.0;
                }
            }
        } else {
            i += 1;
        }
    }

    if cur_page.content.len() > 10 || pages.is_empty() {
        pages.push(cur_page);
    }

    if !is_beamer {
        for (idx, p) in pages.iter_mut().enumerate() {
            let num_str = format!("- {} -", idx + 1);
            let nw = approx_text_width(&num_str, 9.0, false);
            let nx = (page_width - nw) / 2.0;
            p.draw_text(nx, 26.0, "F1", 9.0, &num_str);
        }
    }

    Ok(assemble_pdf(&pages))
}

pub struct EmbeddedTectonicEngine;

impl LatexEngine for EmbeddedTectonicEngine {
    fn engine_name(&self) -> &'static str {
        "Embedded Tectonic Engine"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn compile(&self, code: &str, _project_path: Option<&str>, dir: &Path) -> Result<CompileResponse, String> {
        #[cfg(feature = "tectonic")]
        {
            let pdf_data = tectonic::latex_to_pdf(code)
                .map_err(|e| format!("Tectonic in-process compilation failed: {}", e))?;
            let pdf_path = dir.join("document.pdf");
            let _ = fs::write(&pdf_path, &pdf_data);
            return Ok(CompileResponse {
                pdf_buffer: Some(pdf_data),
                log: Some("Compiled successfully with Embedded Tectonic Engine.".into()),
                error: None,
            });
        }
        #[cfg(not(feature = "tectonic"))]
        {
            #[cfg(not(target_os = "android"))]
            {
                let bin = get_tectonic_bin_path();
                if bin.exists() {
                    let tex_path = dir.join("document.tex");
                    fs::write(&tex_path, code).map_err(|e| e.to_string())?;

                    let mut cmd = create_system_command(&bin);

                    let bundle_opt = get_tectonic_bundle_path();
                    if let Some(ref bundle_path) = bundle_opt {
                        cmd.arg("--bundle").arg(bundle_path).arg("--only-cached");
                    }

                    let output = cmd
                        .arg("-o")
                        .arg(dir)
                        .arg(&tex_path)
                        .output()
                        .map_err(|e| format!("Failed to spawn bundled tectonic: {}", e))?;

                    let pdf_path = dir.join("document.pdf");
                    let pdf_buffer = fs::read(&pdf_path).ok();
                    let status_msg = if bundle_opt.is_some() {
                        "Compiled with Bundled Tectonic Engine (100% offline bundle mode)."
                    } else {
                        "Compiled with Bundled Tectonic Engine."
                    };

                    return Ok(CompileResponse {
                        pdf_buffer,
                        log: Some(format!("{}\n{}", status_msg, String::from_utf8_lossy(&output.stdout))),
                        error: if !output.status.success() {
                            Some(String::from_utf8_lossy(&output.stderr).to_string())
                        } else {
                            None
                        },
                    });
                }
            }

            // In-process embedded engine (100% offline, pure Rust, works everywhere)
            match compile_latex_in_process(code) {
                Ok(pdf_data) => {
                    let pdf_path = dir.join("document.pdf");
                    let _ = fs::write(&pdf_path, &pdf_data);
                    Ok(CompileResponse {
                        pdf_buffer: Some(pdf_data),
                        log: Some("Compiled successfully with In-Process Embedded Engine.".into()),
                        error: None,
                    })
                }
                Err(err) => {
                    Ok(CompileResponse {
                        pdf_buffer: None,
                        log: None,
                        error: Some(err),
                    })
                }
            }
        }
    }
}

#[tauri::command]
async fn compile_latex(code: String, project_path: Option<String>) -> Result<CompileResponse, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;

    let embedded = EmbeddedTectonicEngine;

    #[cfg(target_os = "android")]
    {
        // On Android, NEVER spawn pdflatex! Always run in-process embedded engine.
        return embedded.compile(&code, project_path.as_deref(), &dir);
    }

    #[cfg(not(target_os = "android"))]
    {
        let system = SystemLatexEngine;

        if system.is_available() {
            return system.compile(&code, project_path.as_deref(), &dir);
        }

        embedded.compile(&code, project_path.as_deref(), &dir)
    }
}

#[derive(Serialize)]
pub struct SyncTexResponse {
    line: Option<u32>,
}

#[allow(dead_code)]
fn query_synctex_point(page: u32, x: f32, y: f32, pdf_path: &std::path::Path) -> Option<u32> {
    let output = create_system_command("synctex")
        .arg("edit")
        .arg("-o")
        .arg(format!("{}:{}:{}:{}", page, x, y, pdf_path.display()))
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Line:") {
            if let Some(num_str) = line.split(':').nth(1) {
                if let Ok(num) = num_str.trim().parse::<u32>() {
                    return Some(num);
                }
            }
        }
    }
    None
}

#[tauri::command]
async fn synctex(page: u32, x: f32, y: f32) -> Result<SyncTexResponse, String> {
    #[cfg(target_os = "android")]
    {
        let _ = (page, x, y);
        return Ok(SyncTexResponse { line: None });
    }
    #[cfg(not(target_os = "android"))]
    {
        let dir = get_workspace_dir().map_err(|e| e.to_string())?;
        let pdf_path = dir.join("document.pdf");
        
        if !pdf_path.exists() {
            return Ok(SyncTexResponse { line: None });
        }

        // Check exact point first, then small vertical offsets (+-4, +-8, +-14) in case of line spacing/margins
        let y_offsets = [0.0, -4.0, 4.0, -8.0, 8.0, -14.0, 14.0];
        let mut line_num = None;

        for &dy in &y_offsets {
            let test_y = (y + dy).max(0.0);
            if let Some(l) = query_synctex_point(page, x, test_y, &pdf_path) {
                line_num = Some(l);
                break;
            }
        }

        Ok(SyncTexResponse { line: line_num })
    }
}

#[tauri::command]
fn save_pdf() -> Result<bool, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;
    let pdf_path = dir.join("document.pdf");
    
    if !pdf_path.exists() {
        return Err("No compiled PDF found to save.".into());
    }

    #[cfg(not(target_os = "android"))]
    {
        if let Some(target_path) = tinyfiledialogs::save_file_dialog_with_filter(
            "Save PDF Document",
            "document.pdf",
            &["*.pdf"],
            "PDF Document"
        ) {
            fs::copy(&pdf_path, target_path).map_err(|e| e.to_string())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    #[cfg(target_os = "android")]
    {
        Ok(true)
    }
}

#[allow(dead_code)]
fn get_pandoc_path() -> PathBuf {
    let bin_name = if cfg!(windows) { "pandoc.exe" } else { "pandoc" };
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidates = [
                parent.join("resources").join("binaries").join(bin_name),
                parent.join("resources").join(bin_name),
                parent.join("../Resources").join("binaries").join(bin_name),
                parent.join("../Resources").join(bin_name),
                parent.join("binaries").join(bin_name),
                parent.join(bin_name),
            ];
            for p in &candidates {
                if p.exists() { return p.clone(); }
            }
        }
    }
    let dev_candidates = [
        PathBuf::from(format!("src-tauri/binaries/{}", bin_name)),
        PathBuf::from(format!("binaries/{}", bin_name)),
    ];
    for p in &dev_candidates {
        if p.exists() { return p.clone(); }
    }

    PathBuf::from(bin_name)
}

#[tauri::command]
fn save_docx(code: String) -> Result<bool, String> {
    #[cfg(target_os = "android")]
    {
        let _ = code;
        return Err("Word (.docx) export requires Pandoc and is supported on desktop.".into());
    }
    #[cfg(not(target_os = "android"))]
    {
        let dir = get_workspace_dir().map_err(|e| e.to_string())?;
        let tex_path = dir.join("document.tex");
        let docx_path = dir.join("document.docx");

        fs::write(&tex_path, &code).map_err(|e| e.to_string())?;

        let pandoc_cmd = get_pandoc_path();

        let output = create_system_command(&pandoc_cmd)
            .arg("-s")
            .arg(&tex_path)
            .arg("-o")
            .arg(&docx_path)
            .output()
            .map_err(|e| format!("Failed to spawn pandoc ({:?}): {}", pandoc_cmd, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Pandoc conversion failed: {}", stderr));
        }

        if let Some(target_path) = tinyfiledialogs::save_file_dialog_with_filter(
            "Save Word Document",
            "document.docx",
            &["*.docx"],
            "Word Document (*.docx)"
        ) {
            fs::copy(&docx_path, target_path).map_err(|e| e.to_string())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[tauri::command]
fn list_recent_projects() -> Result<Vec<RecentProject>, String> {
    Ok(load_recent_projects_internal())
}

#[tauri::command]
fn remove_recent_project(path: String) -> Result<bool, String> {
    let mut list = load_recent_projects_internal();
    list.retain(|item| item.path != path);
    save_recent_projects_internal(&list);
    Ok(true)
}

#[tauri::command]
fn read_file_by_path(path: String) -> Result<OpenFilePayload, String> {
    let p = PathBuf::from(&path);
    if !p.is_file() {
        return Err(format!("File does not exist: {}", path));
    }
    let content = fs::read_to_string(&p).map_err(|e| format!("Could not read file: {}", e))?;
    let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
    let last_modified = p.metadata()
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    add_recent_project_internal(&path);
    Ok(OpenFilePayload {
        path,
        name,
        content,
        last_modified,
    })
}

#[tauri::command]
fn open_file_dialog() -> Result<Option<OpenFilePayload>, String> {
    #[cfg(not(target_os = "android"))]
    {
        let filter = ["*.tex", "*.latex", "*.ltx", "*.md", "*.txt"];
        if let Some(selected_path) = tinyfiledialogs::open_file_dialog(
            "Open Project / Document",
            "",
            Some((&filter, "LaTeX & Text Documents (*.tex, *.md, *.txt)"))
        ) {
            let payload = read_file_by_path(selected_path)?;
            Ok(Some(payload))
        } else {
            Ok(None)
        }
    }
    #[cfg(target_os = "android")]
    {
        Ok(None)
    }
}

#[tauri::command]
fn get_launch_file() -> Result<Option<OpenFilePayload>, String> {
    for arg in std::env::args_os().skip(1) {
        let arg_str = arg.to_string_lossy();
        if arg_str.starts_with('-') {
            continue;
        }
        let p = PathBuf::from(&arg);
        if p.is_file() {
            if let Ok(payload) = read_file_by_path(p.to_string_lossy().to_string()) {
                return Ok(Some(payload));
            }
        }
    }
    Ok(None)
}

#[tauri::command]
fn save_file_direct(path: Option<String>, content: String) -> Result<SaveResult, String> {
    let target_path_str = if let Some(p) = path {
        if p.trim().is_empty() {
            None
        } else {
            Some(p)
        }
    } else {
        None
    };

    let final_path = match target_path_str {
        Some(p) => PathBuf::from(p),
        None => {
            #[cfg(not(target_os = "android"))]
            {
                let filter = ["*.tex", "*.latex", "*.md", "*.txt"];
                if let Some(chosen) = tinyfiledialogs::save_file_dialog_with_filter(
                    "Save Document",
                    "document.tex",
                    &filter,
                    "LaTeX Document (*.tex)"
                ) {
                    PathBuf::from(chosen)
                } else {
                    return Ok(SaveResult {
                        success: false,
                        path: String::new(),
                        name: String::new(),
                    });
                }
            }
            #[cfg(target_os = "android")]
            {
                let dir = get_workspace_dir().map_err(|e| e.to_string())?;
                dir.join("document.tex")
            }
        }
    };

    // Atomic write via temporary file in the same directory
    let parent_dir = final_path.parent().unwrap_or(Path::new("."));
    let temp_filename = format!(".{}.tmp", final_path.file_name().unwrap_or_default().to_string_lossy());
    let temp_path = parent_dir.join(temp_filename);

    fs::write(&temp_path, &content).map_err(|e| format!("Failed to write temporary buffer: {}", e))?;
    if let Err(e) = fs::rename(&temp_path, &final_path) {
        // Fallback to copy/remove if rename across mount boundaries fails
        fs::copy(&temp_path, &final_path).map_err(|copy_err| format!("Failed to save file: {} ({})", e, copy_err))?;
        let _ = fs::remove_file(&temp_path);
    }

    let path_str = final_path.to_string_lossy().to_string();
    let name_str = final_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    add_recent_project_internal(&path_str);

    Ok(SaveResult {
        success: true,
        path: path_str,
        name: name_str,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        compile_latex,
        synctex,
        save_pdf,
        save_docx,
        get_launch_file,
        open_file_dialog,
        read_file_by_path,
        save_file_direct,
        list_recent_projects,
        remove_recent_project
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_projects_serde() {
        let sample = RecentProject {
            path: "C:/test/doc.tex".into(),
            name: "doc.tex".into(),
            last_opened: 1700000000,
            file_size: 1024,
            exists: true,
        };
        let json = serde_json::to_string(&sample).expect("serialize");
        let decoded: RecentProject = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.name, "doc.tex");
        assert_eq!(decoded.file_size, 1024);
    }

    #[test]
    fn test_save_and_read_file_direct() {
        let temp_dir = std::env::temp_dir().join("latex_editor_test_io");
        let _ = fs::create_dir_all(&temp_dir);
        let test_file = temp_dir.join("test_article.tex");
        let content = "\\documentclass{article}\n\\begin{document}\nHello V2\n\\end{document}";

        let res = save_file_direct(Some(test_file.to_string_lossy().to_string()), content.into());
        assert!(res.is_ok(), "save_file_direct failed: {:?}", res);
        let save_info = res.unwrap();
        assert!(save_info.success);
        assert_eq!(save_info.name, "test_article.tex");

        let read_res = read_file_by_path(test_file.to_string_lossy().to_string());
        assert!(read_res.is_ok(), "read_file_by_path failed: {:?}", read_res);
        let payload = read_res.unwrap();
        assert_eq!(payload.content, content);
        assert_eq!(payload.name, "test_article.tex");

        let _ = fs::remove_file(&test_file);
        let _ = fs::remove_dir(&temp_dir);
    }

    #[test]
    fn test_compile_response_serde() {
        let resp = CompileResponse {
            pdf_buffer: Some(vec![37, 80, 68, 70]), // %PDF
            log: Some("Compilation complete".into()),
            error: None,
        };
        let json = serde_json::to_string(&resp).expect("serialize compile response");
        let decoded: CompileResponse = serde_json::from_str(&json).expect("deserialize compile response");
        assert_eq!(decoded.pdf_buffer, Some(vec![37, 80, 68, 70]));
        assert_eq!(decoded.log, Some("Compilation complete".into()));
        assert!(decoded.error.is_none());
    }

    #[test]
    fn test_latex_engine_abstraction() {
        let sys_engine = SystemLatexEngine;
        assert_eq!(sys_engine.engine_name(), "System pdflatex");

        let embedded_engine = EmbeddedTectonicEngine;
        assert_eq!(embedded_engine.engine_name(), "Embedded Tectonic Engine");

        // get_tectonic_bin_path should return a valid candidate path
        let tectonic_path = get_tectonic_bin_path();
        assert!(tectonic_path.to_string_lossy().contains("tectonic"));

        // get_tectonic_bundle_path returns None when bundle is not present yet, or Some path if present
        let bundle_opt = get_tectonic_bundle_path();
        if let Some(ref bp) = bundle_opt {
            assert!(bp.to_string_lossy().contains(".ttb"));
        }
    }

    #[test]
    fn test_in_process_engine_article_template() {
        let tex = r#"\documentclass[11pt,a4paper]{article}
\usepackage{amsmath,amsfonts,amssymb}
\title{\textbf{\Huge Research Manuscript}}
\author{\Large Author Name}
\date{\today}
\begin{document}
\maketitle
\begin{abstract}
This research explores advanced typesetting and computation with real-time feedback.
\end{abstract}
\section{Introduction}
Welcome to your Native LaTeX Editor!
\section{Methodology & Equations}
Euler's identity:
\begin{equation}
e^{i\pi} + 1 = 0
\end{equation}
\section{Summary}
\begin{itemize}
\item Instant offline compilation.
\item Crash-free operation.
\end{itemize}
\end{document}"#;

        let res = compile_latex_in_process(tex);
        assert!(res.is_ok(), "compile_latex_in_process failed: {:?}", res.err());
        let pdf = res.unwrap();
        assert!(pdf.starts_with(b"%PDF-1.4"), "PDF must start with standard header");
        assert!(pdf.windows(5).any(|w| w == b"%%EOF"), "PDF must end with %%EOF");
        assert!(pdf.len() > 800, "PDF should contain valid content streams");
    }

    #[test]
    fn test_in_process_engine_beamer_template() {
        let tex = r#"\documentclass{beamer}
\usetheme{Madrid}
\title{Presentation Title}
\subtitle{Subtitle Name}
\author{Author Name}
\date{\today}
\begin{document}
\begin{frame}
\titlepage
\end{frame}
\begin{frame}{Overview}
\section{Key Objectives}
\end{frame}
\begin{frame}{Results}
\begin{block}{Fundamental Theorem}
$$\int_a^b f(x)\,dx = F(b) - F(a)$$
\end{block}
\end{frame}
\end{document}"#;

        let res = compile_latex_in_process(tex);
        assert!(res.is_ok(), "Beamer compile failed: {:?}", res.err());
        let pdf = res.unwrap();
        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(pdf.windows(5).any(|w| w == b"%%EOF"));
    }

    #[test]
    fn test_in_process_engine_resume_template() {
        let tex = r#"\documentclass[11pt,a4paper]{article}
\begin{document}
\begin{center}
{\Huge \textbf{SHOUK SAHA}}\\[4pt]
\small +1 (555) 019-2834 \,\textbar\, email@domain.com
\end{center}
\section{Experience}
\textbf{Senior Systems Engineer} \hfill 2023 -- Present
\end{document}"#;

        let res = compile_latex_in_process(tex);
        assert!(res.is_ok(), "Resume compile failed: {:?}", res.err());
        let pdf = res.unwrap();
        assert!(pdf.starts_with(b"%PDF-1.4"));
    }

    #[test]
    fn test_in_process_engine_math_template() {
        let tex = r#"\documentclass[11pt,a4paper]{article}
\usepackage{amsmath,amssymb}
\title{\textbf{Mathematics Reference}}
\author{Reference}
\begin{document}
\maketitle
\section{Equations}
\begin{equation}
\mathbf{A}\mathbf{v} = \lambda\mathbf{v}
\end{equation}
\begin{align}
\nabla \cdot \mathbf{E} &= \frac{\rho}{\varepsilon_0} \\[6pt]
\nabla \cdot \mathbf{B} &= 0
\end{align}
\end{document}"#;

        let res = compile_latex_in_process(tex);
        assert!(res.is_ok(), "Math template compile failed: {:?}", res.err());
        let pdf = res.unwrap();
        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(pdf.windows(5).any(|w| w == b"%%EOF"));
    }

    #[test]
    fn test_empty_latex_error() {
        let res = compile_latex_in_process("   \n   \t  ");
        assert!(res.is_err(), "Empty LaTeX must return error");
    }

    #[test]
    fn test_clean_latex_text() {
        let raw = r"\textbf{\Huge Clean Title} \textit{and italic}";
        let cleaned = clean_latex_text(raw);
        assert!(cleaned.contains("Clean Title"));
        assert!(cleaned.contains("and italic"));
        assert!(!cleaned.contains(r"\textbf"));
    }

    #[test]
    fn test_system_engine_is_not_available_on_android() {
        let sys = SystemLatexEngine;
        if cfg!(target_os = "android") {
            assert!(!sys.is_available());
            let dir = std::env::temp_dir();
            let res = sys.compile("code", None, &dir);
            assert!(res.is_err());
            assert!(res.unwrap_err().contains("not supported on Android"));
        }
    }
}
