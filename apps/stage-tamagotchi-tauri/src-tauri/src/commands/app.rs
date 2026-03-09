//! Application lifecycle Tauri commands.

use std::sync::Mutex;

use tauri::Manager;

use crate::windows::WindowManager;

/// Quit the application with cleanup.
#[tauri::command]
#[specta::specta]
pub fn app_quit(app: tauri::AppHandle) {
    log::info!("app_quit requested — shutting down");
    app.exit(0);
}

/// Open the main window's DevTools.
#[tauri::command]
#[specta::specta]
pub fn open_main_devtools(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.open_devtools();
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

/// Open the settings window (create or focus).
#[tauri::command]
#[specta::specta]
pub fn open_settings(app: tauri::AppHandle) -> Result<(), String> {
    let wm = app.state::<Mutex<WindowManager>>();
    let wm = wm.lock().map_err(|e| e.to_string())?;
    wm.create_settings_window()
}

/// Open the chat window.
#[tauri::command]
#[specta::specta]
pub fn open_chat(app: tauri::AppHandle) -> Result<(), String> {
    let wm = app.state::<Mutex<WindowManager>>();
    let wm = wm.lock().map_err(|e| e.to_string())?;
    wm.create_chat_window()
}

/// Open a devtools window with an optional route.
#[tauri::command]
#[specta::specta]
pub fn open_devtools_window(app: tauri::AppHandle, route: Option<String>) -> Result<(), String> {
    let wm = app.state::<Mutex<WindowManager>>();
    let wm = wm.lock().map_err(|e| e.to_string())?;
    wm.create_devtools_window(route)
}
