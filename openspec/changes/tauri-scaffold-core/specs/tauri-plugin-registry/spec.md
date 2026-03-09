## ADDED Requirements

### Requirement: Plugin registration mechanism

The system SHALL register existing `crates/` Tauri plugins in the Tauri application's `Cargo.toml` as workspace dependencies and initialize them in the Rust entry point. The following plugins MUST be registered:
- `tauri-plugin-mcp` (MCP server interaction)
- `tauri-plugin-rdev` (input device events)
- `tauri-plugin-window-pass-through-on-hover` (click-through windows)
- `tauri-plugin-window-router-link` (multi-window routing)
- `tauri-plugin-ipc-audio-transcription-ort` (audio transcription)
- `tauri-plugin-ipc-audio-vad-ort` (voice activity detection)

#### Scenario: All plugins initialize on startup
- **WHEN** the Tauri application starts
- **THEN** all 6 plugins initialize without errors and log their initialization status

#### Scenario: MCP plugin commands are accessible
- **WHEN** the Vue frontend calls `connectServer`, `listTools`, or `callTool` from `@proj-airi/tauri-plugin-mcp`
- **THEN** the commands execute successfully via the registered plugin

#### Scenario: Rdev plugin emits input events
- **WHEN** the user moves the mouse or presses a key
- **THEN** the rdev plugin emits corresponding events that the frontend can listen to

### Requirement: Plugin dependency management

The Tauri app's `Cargo.toml` SHALL reference plugins via workspace dependencies (path references to `crates/`), not external crate versions. Plugin versions MUST stay in sync with the workspace version.

#### Scenario: Workspace dependency resolution
- **WHEN** `cargo build` runs in the Tauri app directory
- **THEN** all plugin dependencies resolve from the local `crates/` directory without fetching from crates.io

#### Scenario: Version consistency
- **WHEN** the workspace version is updated in the root `Cargo.toml`
- **THEN** all plugin versions inherit the workspace version automatically

### Requirement: Plugin feature flags

The system SHALL support conditional plugin compilation via Cargo feature flags. Plugins with heavy native dependencies (e.g., `tauri-plugin-ipc-audio-transcription-ort` with ONNX Runtime) MUST be behind optional feature flags to allow faster development builds.

#### Scenario: Dev build without heavy plugins
- **WHEN** developer runs `cargo tauri dev` without the `full` feature flag
- **THEN** heavy plugins (audio-transcription-ort, audio-vad-ort) are excluded, reducing compile time

#### Scenario: Release build with all plugins
- **WHEN** developer runs `cargo tauri build --features full`
- **THEN** all plugins including heavy native dependencies are compiled and bundled
