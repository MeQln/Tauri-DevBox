mod tools;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .invoke_handler(tauri::generate_handler![
      tools::url::url_encode,
      tools::url::url_decode,
      tools::base64::base64_encode,
      tools::base64::base64_decode,
      tools::qrcode::qr_encode,
      tools::qrcode::qr_decode,
      tools::port::list_ports,
      tools::port::kill_port,
      tools::net::ping_host,
      tools::net::check_port,
      tools::hash::hash_text,
      tools::hash::hash_bytes,
      tools::hash::hash_file,
      tools::password::generate_passwords,
      tools::uuid::generate_uuids,
      tools::image::image_read,
      tools::image::image_convert,
      tools::image::image_compress,
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
    .expect("error while running tauri application");
}
