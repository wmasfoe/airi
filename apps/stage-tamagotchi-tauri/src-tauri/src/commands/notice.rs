//! Notice window request/response Tauri commands.

use std::collections::HashMap;
use std::sync::Mutex;

use tauri::{Emitter, Manager};

use crate::types::{RequestWindowPayload, RequestWindowPending};

/// In-memory store for pending notice requests.
pub struct NoticeStore {
    pending: HashMap<String, RequestWindowPending>,
    counter: u64,
}

impl NoticeStore {
    pub fn new() -> Self {
        Self {
            pending: HashMap::new(),
            counter: 0,
        }
    }

    fn next_id(&mut self) -> String {
        self.counter += 1;
        format!("notice-{}", self.counter)
    }
}

/// Open a notice window with the given route and payload.
#[tauri::command]
#[specta::specta]
pub fn notice_open(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<NoticeStore>>,
    payload: RequestWindowPayload,
) -> Result<bool, String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    let id = payload.id.unwrap_or_else(|| store.next_id());

    store.pending.insert(
        id.clone(),
        RequestWindowPending {
            id: id.clone(),
            kind: payload.kind,
            payload: payload.payload,
        },
    );

    let route = payload.route;
    let label = format!("notice-{}", id);

    if let Some(window) = app.get_webview_window(&label) {
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        let _window = tauri::WebviewWindowBuilder::new(
            &app,
            &label,
            tauri::WebviewUrl::App(route.into()),
        )
        .title("Notice")
        .inner_size(400.0, 300.0)
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;
    }

    Ok(true)
}

/// Handle a user action in the notice window.
#[tauri::command]
#[specta::specta]
pub fn notice_action(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<NoticeStore>>,
    id: String,
    action: String,
) -> Result<(), String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    store.pending.remove(&id);

    // Close the notice window
    let label = format!("notice-{}", id);
    if let Some(window) = app.get_webview_window(&label) {
        let _ = window.close();
    }

    log::info!("Notice action: id={}, action={}", id, action);
    Ok(())
}

/// Called when a notice page mounts — returns the pending request data.
#[tauri::command]
#[specta::specta]
pub fn notice_page_mounted(
    state: tauri::State<'_, Mutex<NoticeStore>>,
    id: Option<String>,
) -> Result<Option<RequestWindowPending>, String> {
    let store = state.lock().map_err(|e| e.to_string())?;
    if let Some(id) = id {
        Ok(store.pending.get(&id).cloned())
    } else {
        // Return the most recent pending notice
        Ok(store.pending.values().next().cloned())
    }
}

/// Called when a notice page unmounts.
#[tauri::command]
#[specta::specta]
pub fn notice_page_unmounted(
    state: tauri::State<'_, Mutex<NoticeStore>>,
    id: Option<String>,
) -> Result<(), String> {
    if let Some(id) = id {
        let mut store = state.lock().map_err(|e| e.to_string())?;
        store.pending.remove(&id);
    }
    Ok(())
}
