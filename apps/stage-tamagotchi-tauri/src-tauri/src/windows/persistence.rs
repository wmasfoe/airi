//! Custom window state persistence for panel windows.
//!
//! Standard windows use `tauri-plugin-window-state` for automatic
//! bounds persistence. Panel windows (NSPanel) need custom persistence
//! because they may not be compatible with the standard plugin.
//!
//! State is stored as JSON in `{appDataDir}/window-state.json`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::types::Rectangle;

/// Persisted state for all windows.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct WindowStateFile {
    windows: HashMap<String, Rectangle>,
}

/// Handles reading/writing window bounds to disk.
pub struct WindowStatePersistence {
    path: PathBuf,
}

impl WindowStatePersistence {
    pub fn new(app: &AppHandle) -> Self {
        let path = app
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("window-state.json");
        Self { path }
    }

    /// Load saved bounds for a specific window label.
    pub fn load_bounds(&self, label: &str) -> Option<Rectangle> {
        let data = fs::read_to_string(&self.path).ok()?;
        let state: WindowStateFile = serde_json::from_str(&data).ok()?;
        state.windows.get(label).cloned()
    }

    /// Save bounds for a specific window label.
    pub fn save_bounds(&self, label: &str, bounds: &Rectangle) {
        let mut state = self.load_all().unwrap_or_default();
        state.windows.insert(label.to_string(), bounds.clone());

        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(json) = serde_json::to_string_pretty(&state) {
            let _ = fs::write(&self.path, json);
        }
    }

    /// Load the entire state file.
    fn load_all(&self) -> Option<WindowStateFile> {
        let data = fs::read_to_string(&self.path).ok()?;
        serde_json::from_str(&data).ok()
    }
}
