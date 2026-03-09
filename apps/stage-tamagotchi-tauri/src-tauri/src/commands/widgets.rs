//! Widget management Tauri commands and events.

use std::collections::HashMap;
use std::sync::Mutex;

use tauri::{Emitter, Manager};

use crate::types::{WidgetSnapshot, WidgetsAddPayload};

/// In-memory widget store. In production this would be managed by a proper service.
pub struct WidgetStore {
    widgets: HashMap<String, WidgetSnapshot>,
    counter: u64,
}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            counter: 0,
        }
    }

    fn next_id(&mut self) -> String {
        self.counter += 1;
        format!("widget-{}", self.counter)
    }
}

/// Open the widgets window.
#[tauri::command]
#[specta::specta]
pub fn widgets_open_window(app: tauri::AppHandle, id: Option<String>) -> Result<(), String> {
    let label = id.as_deref().unwrap_or("widgets");
    if let Some(window) = app.get_webview_window(label) {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        label,
        tauri::WebviewUrl::App("/widgets".into()),
    )
    .title("Widgets")
    .decorations(false)
    .always_on_top(true)
    .inner_size(400.0, 600.0)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Add a widget and return its ID.
#[tauri::command]
#[specta::specta]
pub fn widgets_add(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<WidgetStore>>,
    payload: WidgetsAddPayload,
) -> Result<Option<String>, String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    let id = payload.id.unwrap_or_else(|| store.next_id());
    let snapshot = WidgetSnapshot {
        id: id.clone(),
        component_name: payload.component_name,
        component_props: payload.component_props.unwrap_or(serde_json::Value::Object(Default::default())),
        size: payload.size.unwrap_or(serde_json::Value::String("m".to_string())),
        ttl_ms: payload.ttl_ms.unwrap_or(0.0),
    };
    let _ = app.emit("widgets_render", &snapshot);
    store.widgets.insert(id.clone(), snapshot);
    Ok(Some(id))
}

/// Remove a widget by ID.
#[tauri::command]
#[specta::specta]
pub fn widgets_remove(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<WidgetStore>>,
    id: String,
) -> Result<(), String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    store.widgets.remove(&id);
    let _ = app.emit("widgets_remove_event", &serde_json::json!({ "id": id }));
    Ok(())
}

/// Clear all widgets.
#[tauri::command]
#[specta::specta]
pub fn widgets_clear(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<WidgetStore>>,
) -> Result<(), String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    store.widgets.clear();
    let _ = app.emit("widgets_clear_event", &());
    Ok(())
}

/// Update a widget's component props.
#[tauri::command]
#[specta::specta]
pub fn widgets_update(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<WidgetStore>>,
    id: String,
    component_props: Option<serde_json::Value>,
) -> Result<(), String> {
    let mut store = state.lock().map_err(|e| e.to_string())?;
    if let Some(widget) = store.widgets.get_mut(&id) {
        if let Some(props) = &component_props {
            widget.component_props = props.clone();
        }
    }
    let _ = app.emit(
        "widgets_update_event",
        &serde_json::json!({ "id": id, "componentProps": component_props }),
    );
    Ok(())
}

/// Fetch a widget snapshot by ID.
#[tauri::command]
#[specta::specta]
pub fn widgets_fetch(
    state: tauri::State<'_, Mutex<WidgetStore>>,
    id: String,
) -> Result<Option<WidgetSnapshot>, String> {
    let store = state.lock().map_err(|e| e.to_string())?;
    Ok(store.widgets.get(&id).cloned())
}

/// Prepare the widgets window (create if needed, return window label).
#[tauri::command]
#[specta::specta]
pub fn widgets_prepare(app: tauri::AppHandle, id: Option<String>) -> Result<Option<String>, String> {
    let label = id.unwrap_or_else(|| "widgets".to_string());
    if app.get_webview_window(&label).is_none() {
        let _window = tauri::WebviewWindowBuilder::new(
            &app,
            &label,
            tauri::WebviewUrl::App("/widgets".into()),
        )
        .title("Widgets")
        .decorations(false)
        .always_on_top(true)
        .visible(false)
        .inner_size(400.0, 600.0)
        .build()
        .map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(Some(label))
}
