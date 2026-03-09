## Why

AIRI 桌面端依赖多个系统级集成功能：系统托盘（右键菜单、窗口尺寸预设、对齐）、自动更新（electron-updater、GitHub releases）、WebSocket TLS 服务器（H3 + CrossWS、mkcert 证书生成、系统证书安装）、以及插件系统（PluginHost、manifest 发现、会话管理）。这些功能在 Tauri 中都有对应方案，但 API 和实现方式不同。

本 change 将这些系统级功能逐一迁移到 Tauri 生态。

## What Changes

- 迁移系统托盘到 Tauri v2 tray API：
  - 右键菜单（窗口尺寸预设、对齐选项）
  - 动态菜单重建（窗口 resize/move 时更新）
  - 国际化标签
  - 双击支持（macOS）
- 迁移自动更新到 Tauri updater plugin：
  - GitHub releases provider
  - 状态机（idle → checking → available → downloading → downloaded）
  - 下载进度追踪
- 迁移 WebSocket TLS 服务器：
  - 在 Rust 侧实现 WebSocket 服务器（`tokio-tungstenite` 或 `axum`）
  - TLS 证书生成（`rcgen` 替代 `mkcert`）
  - 系统证书安装（macOS Keychain、Windows CertUtil、Linux ca-certificates）
  - 本地 IP 枚举
- 迁移插件系统：
  - Rust 侧插件宿主（或复用现有 JS 插件宿主通过 IPC）
  - Manifest 发现和验证
  - 插件生命周期管理

## Capabilities

### New Capabilities

- `tauri-tray`: Tauri v2 系统托盘实现，动态菜单、国际化、窗口控制集成
- `tauri-auto-updater`: Tauri updater plugin 集成，状态机、进度追踪、GitHub releases
- `tauri-websocket-server`: Rust 原生 WebSocket TLS 服务器，证书生成和系统安装
- `tauri-plugin-host`: 插件系统的 Tauri 适配，manifest 发现、生命周期管理、能力声明

### Modified Capabilities

（无现有 specs 需要修改）

## Impact

- 修改 `apps/stage-tamagotchi-tauri/src-tauri/src/` 添加 tray、updater、server、plugin 模块
- 依赖：`tauri-plugin-updater`、`tokio-tungstenite`/`axum`、`rcgen`、`tauri-plugin-shell`
- 修改 renderer 侧的 updater composables、server channel stores
- 平台特定代码：证书安装（3 个平台各不同）、托盘行为差异
