## Context

AIRI 桌面端依赖四个系统级集成：系统托盘（动态菜单、窗口控制）、自动更新（electron-updater + GitHub releases）、WebSocket TLS 服务器（H3 + CrossWS + mkcert）、插件系统（PluginHost + manifest 发现）。这些在 Tauri 中都有对应方案但 API 不同。

## Goals / Non-Goals

**Goals:**
- 迁移系统托盘到 Tauri tray API
- 迁移自动更新到 tauri-plugin-updater
- 在 Rust 侧实现 WebSocket TLS 服务器
- 适配插件系统到 Tauri 环境

**Non-Goals:**
- 不改变更新发布流程（仍用 GitHub releases）
- 不重写插件 SDK 的 JavaScript API
- 不实现新的插件格式

## Decisions

### 1. 系统托盘：Tauri tray API + alien-signals 响应式

**选择**: 使用 Tauri v2 内置的 `TrayIconBuilder` API。菜单更新逻辑复用现有的 `alien-signals` effect 模式。

**理由**: Tauri v2 的 tray API 功能完整，支持动态菜单重建。

### 2. 自动更新：tauri-plugin-updater + GitHub releases

**选择**: 使用 `tauri-plugin-updater`，配置 GitHub releases 作为更新源。在 Rust 侧实现状态机，通过 Tauri event 推送状态变化。

**替代方案**:
- 自定义更新逻辑：重复造轮子
- 保留 electron-updater 通过 Node.js sidecar：过度复杂

**理由**: Tauri updater 是官方方案，直接支持 GitHub releases。

### 3. WebSocket 服务器：tokio + axum + rcgen

**选择**: 使用 `axum` 的 WebSocket 支持 + `tokio-tungstenite` 作为 WebSocket 引擎，`rcgen` 生成自签名证书，`tokio-rustls` 提供 TLS。

**替代方案**:
- 继续用 Node.js（H3 + CrossWS）通过 sidecar：增加包体积，违背迁移目的
- warp：功能不如 axum 完整

**理由**: axum 是 Rust 生态最成熟的 Web 框架，与 tokio 深度集成。

### 4. 插件系统：Rust 宿主 + JS 运行时

**选择**: 插件宿主的核心逻辑（manifest 解析、文件扫描、状态管理）在 Rust 侧实现。插件的 JavaScript 代码通过 webview 执行（与现有模式一致）。

**理由**: 保持与现有插件 SDK 的兼容性，最小化迁移成本。

## Risks / Trade-offs

- **[Risk] Tauri updater 与 electron-updater 的发布格式不同**: Tauri 需要特定的 JSON manifest → 需要调整 CI/CD 发布流程
- **[Risk] 证书安装需要管理员权限**: macOS Keychain 和 Windows CertUtil 可能需要用户确认 → 提供清晰的 UI 引导
- **[Trade-off] axum 增加二进制体积**: 约 1-2MB → 相比 Electron 的 150MB 可忽略
- **[Risk] 插件 JS 运行时在 webview 中的隔离性**: 插件代码与主应用共享 webview → 考虑 iframe sandbox

## Open Questions

- Tauri updater 是否支持与 electron-updater 相同的 `latest.yml` 格式，还是需要单独的发布 manifest？
- 插件系统是否需要支持 Rust 原生插件（除了 JS 插件），利用 Tauri 的 plugin 机制？
