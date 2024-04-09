use tauri::{Manager, Window};

#[tauri::command]
pub fn start_app(window: Window) {
    window.get_window("launch").expect("launch window not found").close().unwrap();
    window.get_window("main").expect("main window not found").show().unwrap();
}

#[tauri::command]
pub fn show_welcome_window(window: Window) {
    window.get_window("welcome").expect("welcome window not found").show().unwrap();
}

#[tauri::command]
pub fn hide_welcome_window(window: Window) {
    window.get_window("welcome").expect("welcome window not fount").show().unwrap();
}
