## ADDED Requirements

### Requirement: Auto-update via Tauri updater plugin

The system SHALL 使用 `tauri-plugin-updater` 实现自动更新，状态机与现有 Electron 版本一致：
- `idle` → `checking` → `available` / `not-available`
- `available` → `downloading` → `downloaded`
- 任何状态 → `error`

#### Scenario: Check for updates
- **WHEN** 前端调用 `auto_updater_check`
- **THEN** 状态从 `idle` 变为 `checking`，检查完成后变为 `available`（有更新）或 `not-available`

#### Scenario: Download and install update
- **WHEN** 前端调用 `auto_updater_download` 且有可用更新
- **THEN** 状态变为 `downloading`，通过 event 推送下载进度（percent、bytesPerSecond、transferred、total），完成后变为 `downloaded`

#### Scenario: Quit and install
- **WHEN** 前端调用 `auto_updater_quit_and_install` 且更新已下载
- **THEN** 应用退出并安装更新，重启后运行新版本

### Requirement: Update state event stream

The system SHALL 通过 Tauri event 推送更新状态变化，前端可通过 `listen` 订阅。

#### Scenario: Frontend receives state changes
- **WHEN** 更新状态发生变化
- **THEN** 前端收到 `auto_updater_state_changed` event，payload 为 `AutoUpdaterState` 结构
