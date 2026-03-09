//! Shared IPC types for Tauri commands and events.
//!
//! These types mirror the TypeScript interfaces defined in
//! `apps/stage-tamagotchi/src/shared/eventa.ts` and
//! `packages/electron-eventa/src/`.
//!
//! All types derive `specta::Type` for automatic TypeScript binding generation.

use serde::{Deserialize, Serialize};

// ── Window types ──────────────────────────────────────────────

/// Window bounds rectangle (matches Electron's `Rectangle`).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// A 2D point in screen coordinates.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Display/monitor information (matches Electron's `Display`).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Display {
    pub id: u32,
    pub bounds: Rectangle,
    #[serde(rename = "workArea")]
    pub work_area: Rectangle,
    #[serde(rename = "scaleFactor")]
    pub scale_factor: f64,
}

/// Direction for window resize operations.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ResizeDirection {
    N,
    S,
    E,
    W,
    Ne,
    Nw,
    Se,
    Sw,
}

// ── AutoUpdater types ─────────────────────────────────────────

/// Status of the auto-updater (matches TypeScript `AutoUpdaterStatus` union).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum AutoUpdaterStatus {
    Idle,
    Disabled,
    Checking,
    Available,
    NotAvailable,
    Downloading,
    Downloaded,
    Error,
}

/// Download progress information.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AutoUpdaterProgress {
    pub percent: f64,
    #[serde(rename = "bytesPerSecond")]
    pub bytes_per_second: f64,
    pub transferred: f64,
    pub total: f64,
}

/// Auto-updater error information.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AutoUpdaterError {
    pub message: String,
}

/// Aggregate auto-updater state.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AutoUpdaterState {
    pub status: AutoUpdaterStatus,
    pub info: Option<serde_json::Value>,
    pub progress: Option<AutoUpdaterProgress>,
    pub error: Option<AutoUpdaterError>,
}

// ── MCP types ─────────────────────────────────────────────────

/// MCP stdio server configuration entry.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpStdioServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub env: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

/// MCP configuration file structure.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpStdioConfigFile {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: std::collections::HashMap<String, McpStdioServerConfig>,
}

/// Result of applying MCP configuration.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpApplyResult {
    pub path: String,
    pub started: Vec<McpNameEntry>,
    pub failed: Vec<McpFailedEntry>,
    pub skipped: Vec<McpSkippedEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpNameEntry {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpFailedEntry {
    pub name: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpSkippedEntry {
    pub name: String,
    pub reason: String,
}

/// Runtime status of a single MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpServerRuntimeStatus {
    pub name: String,
    pub state: String,
    pub command: String,
    pub args: Vec<String>,
    pub pid: Option<u32>,
    #[serde(rename = "lastError")]
    pub last_error: Option<String>,
}

/// Aggregate MCP runtime status.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpRuntimeStatus {
    pub path: String,
    pub servers: Vec<McpServerRuntimeStatus>,
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// Descriptor for an MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpToolDescriptor {
    #[serde(rename = "serverName")]
    pub server_name: String,
    pub name: String,
    #[serde(rename = "toolName")]
    pub tool_name: String,
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: serde_json::Value,
}

/// Payload for calling an MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpCallToolPayload {
    pub name: String,
    pub arguments: Option<serde_json::Value>,
}

/// Result of an MCP tool call.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct McpCallToolResult {
    pub content: Option<Vec<serde_json::Value>>,
    #[serde(rename = "structuredContent")]
    pub structured_content: Option<serde_json::Value>,
    #[serde(rename = "toolResult")]
    pub tool_result: Option<serde_json::Value>,
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
}

// ── Plugin types ──────────────────────────────────────────────

/// Summary of a plugin manifest.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PluginManifestSummary {
    pub name: String,
    pub entrypoints: std::collections::HashMap<String, Option<String>>,
    pub path: String,
    pub enabled: bool,
    pub loaded: bool,
    #[serde(rename = "isNew")]
    pub is_new: bool,
}

/// Snapshot of the plugin registry.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PluginRegistrySnapshot {
    pub root: String,
    pub plugins: Vec<PluginManifestSummary>,
}

/// Debug snapshot of the plugin host.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PluginHostDebugSnapshot {
    pub registry: PluginRegistrySnapshot,
    pub sessions: Vec<PluginHostSessionSummary>,
    pub capabilities: Vec<PluginCapabilityState>,
    #[serde(rename = "refreshedAt")]
    pub refreshed_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PluginHostSessionSummary {
    pub id: String,
    #[serde(rename = "manifestName")]
    pub manifest_name: String,
    pub phase: String,
    pub runtime: String,
    #[serde(rename = "moduleId")]
    pub module_id: String,
}

/// Payload for updating a plugin capability.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PluginCapabilityPayload {
    pub key: String,
    pub state: String,
    pub metadata: Option<serde_json::Value>,
}

/// State of a plugin capability.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PluginCapabilityState {
    pub key: String,
    pub state: String,
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

// ── Widget types ──────────────────────────────────────────────

/// Snapshot of a widget instance.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct WidgetSnapshot {
    pub id: String,
    #[serde(rename = "componentName")]
    pub component_name: String,
    #[serde(rename = "componentProps")]
    pub component_props: serde_json::Value,
    pub size: serde_json::Value,
    #[serde(rename = "ttlMs")]
    pub ttl_ms: f64,
}

/// Payload for adding a widget.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct WidgetsAddPayload {
    pub id: Option<String>,
    #[serde(rename = "componentName")]
    pub component_name: String,
    #[serde(rename = "componentProps")]
    pub component_props: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
    #[serde(rename = "ttlMs")]
    pub ttl_ms: Option<f64>,
}

// ── Notice / Request types ────────────────────────────────────

/// Payload for opening a request/notice window.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct RequestWindowPayload {
    pub id: Option<String>,
    pub route: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub payload: Option<serde_json::Value>,
}

/// Pending request data returned when a notice page mounts.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct RequestWindowPending {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub payload: Option<serde_json::Value>,
}
