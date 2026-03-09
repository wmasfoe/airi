//! Window management Tauri commands.

use crate::types::{Rectangle, ResizeDirection};

/// Get the current window bounds.
#[tauri::command]
#[specta::specta]
pub fn window_get_bounds(window: tauri::WebviewWindow) -> Result<Rectangle, String> {
    let pos = window.outer_position().map_err(|e| e.to_string())?;
    let size = window.outer_size().map_err(|e| e.to_string())?;
    Ok(Rectangle {
        x: pos.x as f64,
        y: pos.y as f64,
        width: size.width as f64,
        height: size.height as f64,
    })
}

/// Set the window bounds (position and size).
#[tauri::command]
#[specta::specta]
pub fn window_set_bounds(window: tauri::WebviewWindow, bounds: Rectangle) -> Result<(), String> {
    use tauri::{LogicalPosition, LogicalSize};
    window
        .set_position(LogicalPosition::new(bounds.x, bounds.y))
        .map_err(|e| e.to_string())?;
    window
        .set_size(LogicalSize::new(bounds.width, bounds.height))
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Set whether the window ignores mouse events (click-through).
#[tauri::command]
#[specta::specta]
pub fn window_set_ignore_mouse_events(
    window: tauri::WebviewWindow,
    ignore: bool,
    forward: Option<bool>,
) -> Result<(), String> {
    // NOTICE: Tauri's set_ignore_cursor_events doesn't support the `forward`
    // parameter directly. On macOS, forwarding is the default behavior when
    // ignoring cursor events. The `forward` param is kept for API compatibility.
    let _ = forward;
    window
        .set_ignore_cursor_events(ignore)
        .map_err(|e| e.to_string())
}

/// Set macOS vibrancy effect on the window.
#[tauri::command]
#[specta::specta]
pub fn window_set_vibrancy(
    window: tauri::WebviewWindow,
    effect: Option<String>,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use tauri::utils::TitleBarStyle;
        // NOTICE: Tauri v2 doesn't expose a direct vibrancy API like Electron.
        // The vibrancy effect is typically set via window creation config or
        // the tauri-plugin-vibrancy crate. For now, we log the request.
        log::info!("window_set_vibrancy requested: {:?}", effect);
        let _ = (&window, effect);
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (&window, effect);
        Ok(())
    }
}

/// Set Windows background material (acrylic/mica).
#[tauri::command]
#[specta::specta]
pub fn window_set_background_material(
    window: tauri::WebviewWindow,
    material: Option<String>,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        log::info!("window_set_background_material requested: {:?}", material);
        let _ = (&window, material);
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (&window, material);
        Ok(())
    }
}

/// Resize the window by a delta in a given direction.
#[tauri::command]
#[specta::specta]
pub fn window_resize(
    window: tauri::WebviewWindow,
    delta_x: f64,
    delta_y: f64,
    direction: ResizeDirection,
) -> Result<(), String> {
    let size = window.outer_size().map_err(|e| e.to_string())?;
    let pos = window.outer_position().map_err(|e| e.to_string())?;

    let (mut w, mut h) = (size.width as f64, size.height as f64);
    let (mut x, mut y) = (pos.x as f64, pos.y as f64);

    match direction {
        ResizeDirection::N => {
            y += delta_y;
            h -= delta_y;
        }
        ResizeDirection::S => {
            h += delta_y;
        }
        ResizeDirection::E => {
            w += delta_x;
        }
        ResizeDirection::W => {
            x += delta_x;
            w -= delta_x;
        }
        ResizeDirection::Ne => {
            y += delta_y;
            h -= delta_y;
            w += delta_x;
        }
        ResizeDirection::Nw => {
            y += delta_y;
            h -= delta_y;
            x += delta_x;
            w -= delta_x;
        }
        ResizeDirection::Se => {
            h += delta_y;
            w += delta_x;
        }
        ResizeDirection::Sw => {
            h += delta_y;
            x += delta_x;
            w -= delta_x;
        }
    }

    use tauri::{LogicalPosition, LogicalSize};
    window
        .set_position(LogicalPosition::new(x, y))
        .map_err(|e| e.to_string())?;
    window
        .set_size(LogicalSize::new(w.max(1.0), h.max(1.0)))
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Close the current window.
#[tauri::command]
#[specta::specta]
pub fn window_close(window: tauri::WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}
