//! Centralized window management for the AIRI Tauri app.
//!
//! Provides a `WindowManager` that owns references to all application windows
//! and handles creation, lifecycle, and platform-specific behavior (NSPanel on
//! macOS, standard always-on-top on other platforms).

#[cfg(all(target_os = "macos", feature = "plugin-nspanel"))]
mod macos;

#[cfg(not(all(target_os = "macos", feature = "plugin-nspanel")))]
mod fallback;

mod persistence;

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::types::Rectangle;

pub use persistence::WindowStatePersistence;

/// Window labels used throughout the app.
pub mod labels {
    pub const MAIN: &str = "main";
    pub const CAPTION: &str = "caption";
    pub const WIDGETS: &str = "widgets";
    pub const SETTINGS: &str = "settings";
    pub const CHAT: &str = "chat";
    pub const ABOUT: &str = "about";
    pub const NOTICE: &str = "notice";
    pub const INLAY: &str = "inlay";
    pub const DASHBOARD: &str = "dashboard";
    pub const BEAT_SYNC: &str = "beat-sync";
}

/// Centralized manager for all application windows.
///
/// Holds the app handle and provides unified create/get/focus/close
/// operations. On macOS, panel windows use NSPanel via `tauri-nspanel`;
/// on other platforms, standard always-on-top windows are used.
pub struct WindowManager {
    app: AppHandle,
    persistence: WindowStatePersistence,
}

impl WindowManager {
    pub fn new(app: AppHandle) -> Self {
        let persistence = WindowStatePersistence::new(&app);
        Self { app, persistence }
    }

    // ── Panel windows ───────────────────────────────────────────

    /// Configure the main window as an NSPanel (macOS) or always-on-top (other).
    /// The main window is created by `tauri.conf.json` on startup; this method
    /// converts it to the appropriate panel type.
    pub fn create_main_window(&self) -> Result<(), String> {
        #[cfg(all(target_os = "macos", feature = "plugin-nspanel"))]
        {
            macos::convert_main_to_panel(&self.app)?;
        }

        #[cfg(not(all(target_os = "macos", feature = "plugin-nspanel")))]
        {
            if let Some(window) = self.app.get_webview_window(labels::MAIN) {
                window.set_always_on_top(true).map_err(|e| e.to_string())?;
            }
        }

        // Show the main window (it starts hidden per tauri.conf.json)
        if let Some(window) = self.app.get_webview_window(labels::MAIN) {
            window.show().map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Create the caption window (subtitle/speech bubble overlay).
    pub fn create_caption_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::CAPTION).is_some() {
            return self.focus_window(labels::CAPTION);
        }

        let saved = self.persistence.load_bounds(labels::CAPTION);
        let (width, height) = saved.as_ref().map_or((480.0, 180.0), |b| (b.width, b.height));

        #[cfg(all(target_os = "macos", feature = "plugin-nspanel"))]
        {
            macos::create_caption_panel(&self.app, width, height)?;
        }

        #[cfg(not(all(target_os = "macos", feature = "plugin-nspanel")))]
        {
            let window = WebviewWindowBuilder::new(
                &self.app,
                labels::CAPTION,
                WebviewUrl::App("/caption".into()),
            )
            .title("Caption")
            .inner_size(width, height)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .resizable(true)
            .visible(false)
            .build()
            .map_err(|e| e.to_string())?;

            window.set_always_on_top(true).map_err(|e| e.to_string())?;
            window.show().map_err(|e| e.to_string())?;
        }

        // Restore saved position
        if let Some(bounds) = saved {
            if let Some(window) = self.app.get_webview_window(labels::CAPTION) {
                use tauri::LogicalPosition;
                let _ = window.set_position(LogicalPosition::new(bounds.x, bounds.y));
            }
        }

        self.setup_bounds_persistence(labels::CAPTION);
        Ok(())
    }

    /// Create the widgets overlay window.
    pub fn create_widgets_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::WIDGETS).is_some() {
            return self.focus_window(labels::WIDGETS);
        }

        let saved = self.persistence.load_bounds(labels::WIDGETS);
        let (width, height) = saved.as_ref().map_or((500.0, 500.0), |b| (b.width, b.height));

        #[cfg(all(target_os = "macos", feature = "plugin-nspanel"))]
        {
            macos::create_widgets_panel(&self.app, width, height)?;
        }

        #[cfg(not(all(target_os = "macos", feature = "plugin-nspanel")))]
        {
            let window = WebviewWindowBuilder::new(
                &self.app,
                labels::WIDGETS,
                WebviewUrl::App("/widgets".into()),
            )
            .title("Widgets")
            .inner_size(width, height)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .resizable(true)
            .visible(false)
            .build()
            .map_err(|e| e.to_string())?;

            window.set_always_on_top(true).map_err(|e| e.to_string())?;
            window.set_ignore_cursor_events(true).map_err(|e| e.to_string())?;
            window.show().map_err(|e| e.to_string())?;
        }

        if let Some(bounds) = saved {
            if let Some(window) = self.app.get_webview_window(labels::WIDGETS) {
                use tauri::LogicalPosition;
                let _ = window.set_position(LogicalPosition::new(bounds.x, bounds.y));
            }
        }

        self.setup_bounds_persistence(labels::WIDGETS);
        Ok(())
    }

    /// Create the inlay overlay window.
    pub fn create_inlay_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::INLAY).is_some() {
            return self.focus_window(labels::INLAY);
        }

        #[cfg(all(target_os = "macos", feature = "plugin-nspanel"))]
        {
            macos::create_inlay_panel(&self.app)?;
        }

        #[cfg(not(all(target_os = "macos", feature = "plugin-nspanel")))]
        {
            let window = WebviewWindowBuilder::new(
                &self.app,
                labels::INLAY,
                WebviewUrl::App("/inlay".into()),
            )
            .title("Inlay")
            .inner_size(300.0, 200.0)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .resizable(true)
            .visible(false)
            .build()
            .map_err(|e| e.to_string())?;

            window.set_always_on_top(true).map_err(|e| e.to_string())?;
            window.show().map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    // ── Standard windows ────────────────────────────────────────

    /// Open the settings window (singleton — create or focus).
    pub fn create_settings_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::SETTINGS).is_some() {
            return self.focus_window(labels::SETTINGS);
        }

        WebviewWindowBuilder::new(
            &self.app,
            labels::SETTINGS,
            WebviewUrl::App("/settings".into()),
        )
        .title("Settings")
        .inner_size(800.0, 600.0)
        .build()
        .map_err(|e| e.to_string())?;

        self.setup_bounds_persistence(labels::SETTINGS);
        Ok(())
    }

    /// Open the chat window (singleton — create or focus).
    pub fn create_chat_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::CHAT).is_some() {
            return self.focus_window(labels::CHAT);
        }

        WebviewWindowBuilder::new(
            &self.app,
            labels::CHAT,
            WebviewUrl::App("/chat".into()),
        )
        .title("Chat")
        .inner_size(400.0, 600.0)
        .build()
        .map_err(|e| e.to_string())?;

        self.setup_bounds_persistence(labels::CHAT);
        Ok(())
    }

    /// Open a devtools window (supports multiple instances via route-based labels).
    pub fn create_devtools_window(&self, route: Option<String>) -> Result<(), String> {
        let path = route.unwrap_or_else(|| "/devtools".to_string());
        let label = format!(
            "devtools-{}",
            path.replace('/', "-").trim_start_matches('-')
        );

        if self.app.get_webview_window(&label).is_some() {
            return self.focus_window(&label);
        }

        WebviewWindowBuilder::new(
            &self.app,
            &label,
            WebviewUrl::App(path.into()),
        )
        .title("DevTools")
        .inner_size(1200.0, 800.0)
        .build()
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Open the about window (singleton).
    pub fn create_about_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::ABOUT).is_some() {
            return self.focus_window(labels::ABOUT);
        }

        WebviewWindowBuilder::new(
            &self.app,
            labels::ABOUT,
            WebviewUrl::App("/about".into()),
        )
        .title("About AIRI")
        .inner_size(400.0, 300.0)
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Open the notice/request window.
    pub fn create_notice_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::NOTICE).is_some() {
            return self.focus_window(labels::NOTICE);
        }

        WebviewWindowBuilder::new(
            &self.app,
            labels::NOTICE,
            WebviewUrl::App("/notice".into()),
        )
        .title("Notice")
        .inner_size(400.0, 200.0)
        .decorations(false)
        .transparent(true)
        .build()
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Open the dashboard window.
    pub fn create_dashboard_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::DASHBOARD).is_some() {
            return self.focus_window(labels::DASHBOARD);
        }

        WebviewWindowBuilder::new(
            &self.app,
            labels::DASHBOARD,
            WebviewUrl::App("/dashboard".into()),
        )
        .title("Dashboard")
        .inner_size(900.0, 600.0)
        .build()
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Open the beat-sync window.
    pub fn create_beat_sync_window(&self) -> Result<(), String> {
        if self.app.get_webview_window(labels::BEAT_SYNC).is_some() {
            return self.focus_window(labels::BEAT_SYNC);
        }

        WebviewWindowBuilder::new(
            &self.app,
            labels::BEAT_SYNC,
            WebviewUrl::App("/beat-sync".into()),
        )
        .title("Beat Sync")
        .inner_size(400.0, 300.0)
        .decorations(false)
        .transparent(true)
        .build()
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    // ── Helpers ─────────────────────────────────────────────────

    /// Focus an existing window by label.
    pub fn focus_window(&self, label: &str) -> Result<(), String> {
        if let Some(window) = self.app.get_webview_window(label) {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Window '{}' not found", label))
        }
    }

    /// Set up automatic bounds persistence for a window.
    fn setup_bounds_persistence(&self, label: &str) {
        let app = self.app.clone();
        let label_owned = label.to_string();

        if let Some(window) = self.app.get_webview_window(label) {
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_) => {
                        if let Some(win) = app.get_webview_window(&label_owned) {
                            if let (Ok(pos), Ok(size)) = (win.outer_position(), win.outer_size()) {
                                let bounds = Rectangle {
                                    x: pos.x as f64,
                                    y: pos.y as f64,
                                    width: size.width as f64,
                                    height: size.height as f64,
                                };
                                let persistence = WindowStatePersistence::new(&app);
                                persistence.save_bounds(&label_owned, &bounds);
                            }
                        }
                    }
                    _ => {}
                }
            });
        }
    }
}
