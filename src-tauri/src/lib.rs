use serde::Serialize;
use std::fs;
use std::process::Command;
use std::path::PathBuf;

fn get_workspace_dir() -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join("latex-editor-workspace");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

#[derive(Serialize)]
pub struct CompileResponse {
    pdf_buffer: Option<Vec<u8>>,
    log: Option<String>,
    error: Option<String>,
}

#[tauri::command]
async fn compile_latex(code: String) -> Result<CompileResponse, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;
    let tex_path = dir.join("document.tex");
    
    fs::write(&tex_path, code).map_err(|e| e.to_string())?;

    let output = Command::new("pdflatex")
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
    let output = Command::new("synctex")
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
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let p1 = parent.join("resources").join("binaries").join("pandoc.exe");
            if p1.exists() { return p1; }
            let p2 = parent.join("resources").join("pandoc.exe");
            if p2.exists() { return p2; }
            let p3 = parent.join("binaries").join("pandoc.exe");
            if p3.exists() { return p3; }
            let p4 = parent.join("pandoc.exe");
            if p4.exists() { return p4; }
        }
    }
    let dev_path = PathBuf::from("src-tauri/binaries/pandoc.exe");
    if dev_path.exists() { return dev_path; }
    let dev_path2 = PathBuf::from("binaries/pandoc.exe");
    if dev_path2.exists() { return dev_path2; }

    PathBuf::from("pandoc")
}

#[tauri::command]
fn save_docx(code: String) -> Result<bool, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;
    let tex_path = dir.join("document.tex");
    let docx_path = dir.join("document.docx");

    fs::write(&tex_path, &code).map_err(|e| e.to_string())?;

    let pandoc_cmd = get_pandoc_path();

    let output = Command::new(&pandoc_cmd)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![compile_latex, synctex, save_pdf, save_docx])
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
