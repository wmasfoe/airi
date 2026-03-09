## ADDED Requirements

### Requirement: Tauri v2 application entry point

The system SHALL provide a Tauri v2 application at `apps/stage-tamagotchi-tauri/` with a valid `src-tauri/` directory containing `tauri.conf.json`, `Cargo.toml`, `src/main.rs`, and `capabilities/` configuration. The application MUST target Tauri 2.x (>=2.3) and use edition 2021 or later Rust.

#### Scenario: Application compiles and launches
- **WHEN** developer runs `cargo tauri dev` in `apps/stage-tamagotchi-tauri/`
- **THEN** the application compiles without errors and opens a single webview window rendering the Vue frontend

#### Scenario: Production build succeeds
- **WHEN** developer runs `cargo tauri build` in `apps/stage-tamagotchi-tauri/`
- **THEN** the system produces platform-specific installers (`.dmg` on macOS, `.msi`/`.exe` on Windows, `.deb`/`.AppImage` on Linux)

### Requirement: Tauri configuration file

The system SHALL include a `tauri.conf.json` with the following configuration:
- `identifier`: `ai.moeru.airi`
- `productName`: `AIRI`
- Default window: transparent, frameless, 450x600 initial size
- Security: appropriate CSP for loading local assets and connecting to external APIs
- Bundle configuration for all three desktop platforms

#### Scenario: App identifier matches existing Electron app
- **WHEN** the Tauri app is built
- **THEN** the bundle identifier is `ai.moeru.airi`, matching the existing Electron app's `appId`

#### Scenario: Default window is transparent and frameless
- **WHEN** the application launches
- **THEN** the main window has no native title bar, has a transparent background, and is 450x600 pixels

### Requirement: Capability permissions

The system SHALL define Tauri capability files in `src-tauri/capabilities/` that grant the webview access to:
- Window management APIs (create, close, resize, position, always-on-top)
- Shell plugin (subprocess spawning for MCP)
- File system access (user data directory for config/plugins)
- HTTP/WebSocket connections (for API calls and server channel)

#### Scenario: Webview can invoke window commands
- **WHEN** the Vue frontend calls `@tauri-apps/api/window` APIs
- **THEN** the calls succeed without permission errors

#### Scenario: Webview can spawn subprocesses
- **WHEN** the Vue frontend invokes shell commands via `tauri-plugin-shell`
- **THEN** the subprocess spawns successfully with stdio access

### Requirement: Rust main entry point

The system SHALL provide a `src/main.rs` (or `src/lib.rs`) that:
- Initializes the Tauri application with `tauri::Builder::default()`
- Registers all required plugins (shell, window-state, etc.)
- Sets up a minimal invoke handler for health-check
- Configures logging via `log` crate

#### Scenario: Health check command works
- **WHEN** the Vue frontend invokes the `health_check` Tauri command
- **THEN** the command returns a success response with the app version

#### Scenario: Plugins are registered
- **WHEN** the application starts
- **THEN** all registered plugins initialize without errors and their commands are available to the webview
