//! Screen information Tauri commands.

use crate::types::{Display, Point, Rectangle};

/// Get all connected displays/monitors.
#[tauri::command]
#[specta::specta]
pub fn screen_get_all_displays(window: tauri::WebviewWindow) -> Result<Vec<Display>, String> {
    let monitors = window.available_monitors().map_err(|e| e.to_string())?;
    Ok(monitors
        .into_iter()
        .enumerate()
        .map(|(i, m)| {
            let pos = m.position();
            let size = m.size();
            let scale = m.scale_factor();
            Display {
                id: i as u32,
                bounds: Rectangle {
                    x: pos.x as f64,
                    y: pos.y as f64,
                    width: size.width as f64,
                    height: size.height as f64,
                },
                // NOTICE: Tauri doesn't expose work area separately;
                // we approximate it as the full monitor bounds.
                work_area: Rectangle {
                    x: pos.x as f64,
                    y: pos.y as f64,
                    width: size.width as f64,
                    height: size.height as f64,
                },
                scale_factor: scale,
            }
        })
        .collect())
}

/// Get the primary display.
#[tauri::command]
#[specta::specta]
pub fn screen_get_primary_display(window: tauri::WebviewWindow) -> Result<Display, String> {
    let monitor = window
        .primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No primary monitor found".to_string())?;
    let pos = monitor.position();
    let size = monitor.size();
    let scale = monitor.scale_factor();
    Ok(Display {
        id: 0,
        bounds: Rectangle {
            x: pos.x as f64,
            y: pos.y as f64,
            width: size.width as f64,
            height: size.height as f64,
        },
        work_area: Rectangle {
            x: pos.x as f64,
            y: pos.y as f64,
            width: size.width as f64,
            height: size.height as f64,
        },
        scale_factor: scale,
    })
}

/// Get the current cursor screen position.
#[tauri::command]
#[specta::specta]
pub fn screen_get_cursor_screen_point(window: tauri::WebviewWindow) -> Result<Point, String> {
    let pos = window.cursor_position().map_err(|e| e.to_string())?;
    Ok(Point {
        x: pos.x,
        y: pos.y,
    })
}

/// Convert a DIP point to screen coordinates.
/// On most platforms this is a no-op since Tauri already works in logical (DIP) coordinates.
#[tauri::command]
#[specta::specta]
pub fn screen_dip_to_screen_point(point: Point) -> Point {
    // NOTICE: Tauri's coordinate system is already DIP-based.
    // This is kept for API compatibility with the Electron version.
    point
}

/// Convert a DIP rectangle to screen coordinates.
#[tauri::command]
#[specta::specta]
pub fn screen_dip_to_screen_rect(rect: Rectangle) -> Rectangle {
    rect
}

/// Convert screen coordinates to DIP point.
#[tauri::command]
#[specta::specta]
pub fn screen_screen_to_dip_point(point: Point) -> Point {
    point
}

/// Convert a screen rectangle to DIP coordinates.
#[tauri::command]
#[specta::specta]
pub fn screen_screen_to_dip_rect(rect: Rectangle) -> Rectangle {
    rect
}
