//! Continuous event stream commands (mouse tracking, window bounds polling).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::types::{Point, Rectangle};

use tauri::Emitter;

/// Start tracking mouse position, emitting `cursor_screen_point` events at ≤50ms intervals.
#[tauri::command]
#[specta::specta]
pub fn start_tracking_mouse_position(app: tauri::AppHandle, window: tauri::WebviewWindow) {
    let label = window.label().to_string();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // Stop tracking when the window is destroyed
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Destroyed = event {
            running_clone.store(false, Ordering::Relaxed);
        }
    });

    tauri::async_runtime::spawn(async move {
        use tokio::time::{interval, Duration};
        let mut tick = interval(Duration::from_millis(50));

        while running.load(Ordering::Relaxed) {
            tick.tick().await;
            if let Ok(pos) = window_clone.cursor_position() {
                let point = Point { x: pos.x, y: pos.y };
                let _ = app.emit("cursor_screen_point", &point);
            }
        }
        log::debug!("Mouse tracking stopped for window {}", label);
    });
}

/// Start polling window bounds, emitting `window_bounds` events on change.
#[tauri::command]
#[specta::specta]
pub fn start_loop_get_bounds(app: tauri::AppHandle, window: tauri::WebviewWindow) {
    let label = window.label().to_string();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Destroyed = event {
            running_clone.store(false, Ordering::Relaxed);
        }
    });

    tauri::async_runtime::spawn(async move {
        use tokio::time::{interval, Duration};
        let mut tick = interval(Duration::from_millis(50));
        let mut last: Option<(i32, i32, u32, u32)> = None;

        while running.load(Ordering::Relaxed) {
            tick.tick().await;
            let pos = window_clone.outer_position();
            let size = window_clone.outer_size();
            if let (Ok(p), Ok(s)) = (pos, size) {
                let current = (p.x, p.y, s.width, s.height);
                if last.as_ref() != Some(&current) {
                    last = Some(current);
                    let rect = Rectangle {
                        x: p.x as f64,
                        y: p.y as f64,
                        width: s.width as f64,
                        height: s.height as f64,
                    };
                    let _ = app.emit("window_bounds", &rect);
                }
            }
        }
        log::debug!("Bounds tracking stopped for window {}", label);
    });
}

/// Start dragging the window (Tauri's built-in drag).
#[tauri::command]
#[specta::specta]
pub fn start_dragging_window(window: tauri::WebviewWindow) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}
