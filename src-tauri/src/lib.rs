pub mod error;
pub use error::*;
pub mod commands;
pub mod network;
pub mod utils;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::init_logger();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let salt_path = app.path().app_local_data_dir()?.join("salt.txt");
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::generate_keypair,
            commands::register_keypair,

            commands::network::start,
            commands::network::dial,
            commands::network::close,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
