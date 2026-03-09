## 1. 系统托盘

- [ ] 1.1 使用 `TrayIconBuilder` 创建系统托盘图标
- [ ] 1.2 实现右键上下文菜单（窗口尺寸预设、对齐、显示/隐藏、设置、DevTools、退出）
- [ ] 1.3 实现动态菜单更新（窗口 resize/move 时重建菜单）
- [ ] 1.4 实现国际化菜单标签（从 `@proj-airi/i18n` 加载翻译）
- [ ] 1.5 实现 macOS 双击切换窗口显示/隐藏
- [ ] 1.6 实现菜单项与 WindowManager 的联动（尺寸预设、对齐操作）

## 2. 自动更新

- [ ] 2.1 添加 `tauri-plugin-updater` 依赖并注册
- [ ] 2.2 配置 GitHub releases 作为更新源（`tauri.conf.json` 的 updater 配置）
- [ ] 2.3 实现 Rust 侧更新状态机（idle/checking/available/downloading/downloaded/error）
- [ ] 2.4 实现 `auto_updater_get_state` Tauri command
- [ ] 2.5 实现 `auto_updater_check` Tauri command
- [ ] 2.6 实现 `auto_updater_download` Tauri command + 进度 event 推送
- [ ] 2.7 实现 `auto_updater_quit_and_install` Tauri command
- [ ] 2.8 实现 `auto_updater_state_changed` Tauri event
- [ ] 2.9 调整 CI/CD 发布流程生成 Tauri updater 所需的 JSON manifest

## 3. WebSocket TLS 服务器

- [ ] 3.1 添加 `axum`、`tokio-tungstenite`、`rcgen`、`tokio-rustls` 依赖
- [ ] 3.2 实现自签名 CA 和服务器证书生成（`rcgen`），SAN 包含 localhost + 本地 IP
- [ ] 3.3 实现证书持久化（保存到 userData 目录）
- [ ] 3.4 实现 macOS 证书安装（`security add-trusted-cert` 命令）
- [ ] 3.5 实现 Windows 证书安装（`certutil` 命令）
- [ ] 3.6 实现 Linux 证书安装（`ca-certificates` 更新）
- [ ] 3.7 实现 WebSocket 服务器（axum + TLS），端口 6121
- [ ] 3.8 实现 server channel 配置的 Tauri commands（get-config、apply-config）
- [ ] 3.9 实现 graceful shutdown（app quit 时关闭所有连接）

## 4. 插件系统

- [ ] 4.1 实现 Rust 侧 manifest v1 解析器
- [ ] 4.2 实现插件目录扫描（`{userData}/plugins/v1/`）
- [ ] 4.3 实现插件启用/禁用状态持久化（JSON 配置文件）
- [ ] 4.4 实现 `plugin_list` Tauri command
- [ ] 4.5 实现 `plugin_set_enabled` Tauri command
- [ ] 4.6 实现 `plugin_load` / `plugin_unload` Tauri commands
- [ ] 4.7 实现 `plugin_inspect` Tauri command（debug snapshot）
- [ ] 4.8 实现 `plugin_capability_update` Tauri command
- [ ] 4.9 实现 `plugin_list_providers` Tauri command

## 5. 验证

- [ ] 5.1 验证托盘图标显示和右键菜单功能
- [ ] 5.2 验证菜单国际化（切换语言后菜单标签更新）
- [ ] 5.3 验证自动更新检查和下载流程
- [ ] 5.4 验证 WebSocket 服务器 TLS 连接
- [ ] 5.5 验证证书安装流程（macOS）
- [ ] 5.6 验证插件发现和加载
- [ ] 5.7 运行 `pnpm typecheck` 和 `pnpm lint:fix`
