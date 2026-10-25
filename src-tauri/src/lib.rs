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

#[derive(Serialize, Deserialize, Debug)]
pub struct CompileResponse {
    pdf_buffer: Option<Vec<u8>>,
    log: Option<String>,
    error: Option<String>,
}

#[tauri::command]
async fn compile_latex(code: String, project_path: Option<String>) -> Result<CompileResponse, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;
    let tex_path = dir.join("document.tex");
    
    fs::write(&tex_path, code).map_err(|e| e.to_string())?;

    let mut cmd = create_system_command("pdflatex");
    
    // Set current working directory to document's parent folder for relative assets
    if let Some(ref proj_file) = project_path {
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

#[derive(Serialize)]
pub struct SyncTexResponse {
    line: Option<u32>,
}

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

#[tauri::command]
fn save_pdf() -> Result<bool, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;
    let pdf_path = dir.join("document.pdf");
    
    if !pdf_path.exists() {
        return Err("No compiled PDF found to save.".into());
    }

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
}
