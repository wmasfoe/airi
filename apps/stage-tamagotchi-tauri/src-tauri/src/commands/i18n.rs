//! i18n Tauri commands.

use std::sync::Mutex;

/// In-memory locale store.
pub struct LocaleStore {
    locale: String,
}

impl LocaleStore {
    pub fn new(default_locale: &str) -> Self {
        Self {
            locale: default_locale.to_string(),
        }
    }
}

/// Set the application locale.
#[tauri::command]
#[specta::specta]
pub fn i18n_set_locale(
    state: tauri::State<'_, Mutex<LocaleStore>>,
    locale: String,
) -> Result<(), String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    log::info!("Locale changed: {} → {}", store.locale, locale);
    store.locale = locale;
    Ok(())
}

/// Get the current application locale.
#[tauri::command]
#[specta::specta]
pub fn i18n_get_locale(
    state: tauri::State<'_, Mutex<LocaleStore>>,
) -> Result<String, String> {
    let store = state.lock().map_err(|e| e.to_string())?;
    Ok(store.locale.clone())
}
