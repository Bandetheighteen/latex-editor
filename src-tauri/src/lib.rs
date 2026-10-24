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

#[tauri::command]
async fn synctex(page: u32, x: f32, y: f32) -> Result<SyncTexResponse, String> {
    let dir = get_workspace_dir().map_err(|e| e.to_string())?;
    let pdf_path = dir.join("document.pdf");
    
    if !pdf_path.exists() {
        return Ok(SyncTexResponse { line: None });
    }

    let output = Command::new("synctex")
        .arg("edit")
        .arg("-o")
        .arg(format!("{}:{}:{}:{}", page, x, y, pdf_path.display()))
        .output()
        .map_err(|e| format!("Failed to spawn synctex: {}", e))?;

    if !output.status.success() {
        return Ok(SyncTexResponse { line: None });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut line_num = None;
    for line in stdout.lines() {
        if line.starts_with("Line:") {
            if let Some(num_str) = line.split(':').nth(1) {
                if let Ok(num) = num_str.trim().parse::<u32>() {
                    line_num = Some(num);
                    break;
                }
            }
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![compile_latex, synctex, save_pdf])
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
