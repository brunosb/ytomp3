use std::{fs, path::PathBuf, thread, time::Duration};
use tauri::{AppHandle, Manager};

pub fn start_update(app: AppHandle) {
  thread::spawn(move || {
    let _ = update(&app);
  });
}
fn update(app: &AppHandle) -> Result<(), String> {
  let root = app.path().app_local_data_dir().map_err(|e| e.to_string())?; fs::create_dir_all(&root).map_err(|e| e.to_string())?;
  let client = reqwest::blocking::Client::builder().user_agent("YToMP3").timeout(Duration::from_secs(45)).build().map_err(|e| e.to_string())?;
  let release: serde_json::Value = client.get("https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest").send().map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.json().map_err(|e| e.to_string())?;
  let tag = release["tag_name"].as_str().ok_or("Versão do yt-dlp indisponível")?;
  let marker = root.join("yt-dlp.version"); if fs::read_to_string(&marker).ok().as_deref() == Some(tag) && root.join("yt-dlp.exe").is_file() { return Ok(()) }
  let url = format!("https://github.com/yt-dlp/yt-dlp/releases/download/{tag}/yt-dlp.exe");
  let bytes = client.get(url).send().map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.bytes().map_err(|e| e.to_string())?;
  if bytes.len() < 100_000 { return Err("Download do yt-dlp inválido".into()) }
  let temp = root.join("yt-dlp.exe.tmp"); fs::write(&temp, &bytes).map_err(|e| e.to_string())?; fs::rename(temp, root.join("yt-dlp.exe")).map_err(|e| e.to_string())?; fs::write(marker, tag).map_err(|e| e.to_string())?; Ok(())
}
