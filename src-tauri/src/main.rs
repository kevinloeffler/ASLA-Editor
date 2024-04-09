// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::exit;
use std::sync::Mutex;
use tauri::Manager;

mod backend_error;
mod image_handling;
mod windows;
mod config_manager;
mod editor;
mod metadata;

fn main() {
  tauri::Builder::default()
      .manage(Mutex::new(editor::Editor::default()))
      .setup(|app| {
          // Close app when main window is closed
          let window = app.get_window("main").expect("main window not found");
          window.on_window_event(|event| match event {
              tauri::WindowEvent::CloseRequested { .. } => { exit(0) }
              _ => {}
          });
          Ok(())
      })
      .invoke_handler(tauri::generate_handler![
          image_handling::process_image,
          windows::start_app, windows::show_welcome_window, windows::hide_welcome_window,
          config_manager::load_environment_file, config_manager::load_config_file,
            config_manager::update_environment_project, config_manager::update_environment_upload,
          editor::get_image, editor::update_image, editor::update_entities,
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
