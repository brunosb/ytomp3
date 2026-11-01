use std::{fs, path::{Path, PathBuf}, process::Command};
use tauri::{AppHandle, Manager};

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> { Ok(app.path().app_local_data_dir().map_err(|e| e.to_string())?.join("settings.json")) }
fn default_folder() -> PathBuf { dirs::download_dir().unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))) }
fn tool(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
  let root = app.path().resource_dir().map_err(|e| e.to_string())?;
  let bundled = root.join("bin").join(name);
  if bundled.is_file() { return Ok(bundled) }
  if name == "yt-dlp.exe" {
    let update = app.path().app_local_data_dir().map_err(|e| e.to_string())?.join("yt-dlp.exe");
    if update.is_file() { return Ok(update) }
  }
  Err(format!("Componente {name} não encontrado. Reinstale o YToMP3."))
}

#[tauri::command]
pub fn get_download_folder(app: AppHandle) -> Result<String, String> {
  let path = settings_path(&app)?;
  if let Ok(text) = fs::read_to_string(path) { if let Ok(value) = serde_json::from_str::<String>(&text) { return Ok(value) } }
  Ok(default_folder().to_string_lossy().into_owned())
}

#[tauri::command]
pub fn set_download_folder(app: AppHandle, path: String) -> Result<(), String> {
  let target = PathBuf::from(&path);
  if !target.is_dir() { return Err("A pasta selecionada não existe.".into()) }
  let file = settings_path(&app)?; if let Some(parent) = file.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
  fs::write(file, serde_json::to_string(&path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn download_mp3(app: AppHandle, url: String, output_dir: String) -> Result<String, String> {
  let parsed = url::Url::parse(&url).map_err(|_| "Link inválido do YouTube.".to_string())?;
  if !matches!(parsed.host_str(), Some("youtube.com" | "www.youtube.com" | "m.youtube.com" | "youtu.be")) { return Err("Informe uma URL válida do YouTube.".into()) }
  let output = Path::new(&output_dir); if !output.is_dir() { return Err("A pasta de destino não existe.".into()) }
  let ytdlp = tool(&app, "yt-dlp.exe")?; let ffmpeg = tool(&app, "ffmpeg.exe")?;
  let template = output.join("%(title)s.%(ext)s");
  let result = Command::new(&ytdlp).args(["--ignore-config", "--no-playlist", "--no-warnings", "-x", "--audio-format", "mp3", "--audio-quality", "0", "--ffmpeg-location", ffmpeg.parent().unwrap_or(output).to_string_lossy().as_ref(), "-o", template.to_string_lossy().as_ref(), &url]).output().map_err(|e| format!("Não foi possível iniciar o yt-dlp: {e}"))?;
  if !result.status.success() { return Err(String::from_utf8_lossy(&result.stderr).trim().to_string()) }
  let text = String::from_utf8_lossy(&result.stdout); Ok(text.lines().last().unwrap_or("Download concluído").trim().to_string())
}
