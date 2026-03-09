//! macOS-specific window management using `tauri-nspanel`.
//!
//! Converts standard Tauri windows into NSPanel instances for floating
//! overlay behavior (visible above fullscreen apps, on all Spaces).
//! Uses `PanelBuilder` for new panel windows and `to_panel()` for
//! converting the main window created by `tauri.conf.json`.

use tauri::{AppHandle, Manager};
use tauri_nspanel::{
    tauri_panel, CollectionBehavior, PanelBuilder, PanelLevel,
    StyleMask, WebviewWindowExt,
};

// ── Panel class definitions ─────────────────────────────────────

// NOTICE: Each panel class defines the NSPanel swizzle behavior.
// `can_become_key_window: true` allows the panel to receive keyboard input.
// `is_floating_panel: true` keeps it above normal windows.

tauri_panel! {
    panel!(AiriMainPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })

    panel!(AiriCaptionPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })

    panel!(AiriWidgetsPanel {
        config: {
            can_become_key_window: false,
            is_floating_panel: true
        }
    })

    panel!(AiriInlayPanel {
        config: {
            can_become_key_window: false,
            is_floating_panel: true
        }
    })
}

/// Convert the existing main window (created by tauri.conf.json) to an NSPanel.
pub fn convert_main_to_panel(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found for panel conversion".to_string())?;

    let panel = window
        .to_panel::<AiriMainPanel>()
        .map_err(|e| format!("Failed to convert main to panel: {}", e))?;

    panel.set_level(PanelLevel::ScreenSaver);
    panel.set_collection_behavior(
        CollectionBehavior::new()
            .can_join_all_spaces()
            .full_screen_auxiliary(),
    );

    log::info!("Converted main window to NSPanel (ScreenSaver level, all Spaces)");
    Ok(())
}

/// Create a caption panel window using PanelBuilder.
pub fn create_caption_panel(
    app: &AppHandle,
    width: f64,
    height: f64,
) -> Result<(), String> {
    PanelBuilder::<_, AiriCaptionPanel>::new(app, "caption")
        .url(tauri::WebviewUrl::App("/caption".into()))
        .title("Caption")
        .size(width, height)
        .transparent(true)
        .no_activate(true)
        .level(PanelLevel::ScreenSaver)
        .style_mask(StyleMask::empty().borderless().non_activating_panel())
        .collection_behavior(
            CollectionBehavior::new()
                .can_join_all_spaces()
                .full_screen_auxiliary(),
        )
        .build()
        .map_err(|e| format!("Failed to create caption panel: {}", e))?;

    log::info!("Created caption NSPanel (ScreenSaver level)");
    Ok(())
}

/// Create a widgets panel window using PanelBuilder.
pub fn create_widgets_panel(
    app: &AppHandle,
    width: f64,
    height: f64,
) -> Result<(), String> {
    PanelBuilder::<_, AiriWidgetsPanel>::new(app, "widgets")
        .url(tauri::WebviewUrl::App("/widgets".into()))
        .title("Widgets")
        .size(width, height)
        .transparent(true)
        .no_activate(true)
        .level(PanelLevel::Floating)
        .style_mask(StyleMask::empty().borderless().non_activating_panel())
        .collection_behavior(
            CollectionBehavior::new()
                .can_join_all_spaces()
                .full_screen_auxiliary(),
        )
        .build()
        .map_err(|e| format!("Failed to create widgets panel: {}", e))?;

    log::info!("Created widgets NSPanel (Floating level)");
    Ok(())
}

/// Create an inlay panel window using PanelBuilder.
pub fn create_inlay_panel(app: &AppHandle) -> Result<(), String> {
    PanelBuilder::<_, AiriInlayPanel>::new(app, "inlay")
        .url(tauri::WebviewUrl::App("/inlay".into()))
        .title("Inlay")
        .size(300.0, 200.0)
        .transparent(true)
        .no_activate(true)
        .level(PanelLevel::Floating)
        .style_mask(StyleMask::empty().borderless().non_activating_panel())
        .collection_behavior(
            CollectionBehavior::new()
                .can_join_all_spaces()
                .full_screen_auxiliary(),
        )
        .build()
        .map_err(|e| format!("Failed to create inlay panel: {}", e))?;

    log::info!("Created inlay NSPanel (Floating level)");
    Ok(())
}
