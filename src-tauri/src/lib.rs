pub mod device;
pub mod error;
pub mod identify;
pub use error::*;
pub mod commands;
pub mod connection;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::init_logger();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::register,
            commands::discover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
