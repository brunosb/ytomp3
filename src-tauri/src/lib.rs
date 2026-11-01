#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod updater;

pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| { updater::start_update(app.handle().clone()); Ok(()) })
    .invoke_handler(tauri::generate_handler![commands::get_download_folder, commands::set_download_folder, commands::download_mp3])
    .run(tauri::generate_context!())
    .expect("error while running YToMP3");
}
