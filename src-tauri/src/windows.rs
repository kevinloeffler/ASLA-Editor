use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn start_app(app_handle: AppHandle) {
    if let Some(launch_window) = app_handle.get_window("launch") {
        if let Err(e) = launch_window.close() {
            println!("Failed to close launch window: {}", e);
        }
    } else {
        println!("Launch window not found");
    }

    if let Some(main_window) = app_handle.get_window("main") {
        if let Err(e) = main_window.show() {
            println!("Failed to show main window: {}", e);
        }
    } else {
        println!("Main window not found");
    }
}

#[tauri::command]
pub fn show_welcome_window(app_handle: AppHandle) {
    if let Some(welcome_window) = app_handle.get_window("welcome") {
        if let Err(e) = welcome_window.show() {
            println!("Failed to show welcome window: {}", e);
        }
    } else {
        println!("Welcome window not found");
    }
}

#[tauri::command]
pub fn hide_welcome_window(app_handle: AppHandle) {
    if let Some(welcome_window) = app_handle.get_window("welcome") {
        if let Err(e) = welcome_window.hide() {
            println!("Failed to hide welcome window: {}", e);
        }
    } else {
        println!("Welcome window not found");
    }
}