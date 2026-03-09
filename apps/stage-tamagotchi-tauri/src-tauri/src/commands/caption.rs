//! Caption overlay Tauri commands.

use std::sync::atomic::{AtomicBool, Ordering};

/// Global state for caption following behavior.
pub static CAPTION_IS_FOLLOWING: AtomicBool = AtomicBool::new(false);

/// Get whether the caption overlay is following the main window.
#[tauri::command]
#[specta::specta]
pub fn caption_get_is_following_window() -> bool {
    CAPTION_IS_FOLLOWING.load(Ordering::Relaxed)
}
