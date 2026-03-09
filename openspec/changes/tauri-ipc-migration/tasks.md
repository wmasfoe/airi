## 1. Tauri Adapter 基础设施

- [x] 1.1 创建 `packages/tauri-eventa/`（或 `@moeru/eventa/adapters/tauri`），实现 `createContext`、`invoke`、`listen`、`emit` 函数
- [x] 1.2 实现 eventa channel name → Tauri command name 的映射函数（`eventa:invoke:electron:window:close` → `window_close`）
- [x] 1.3 实现 eventa event name → Tauri event name 的映射函数
- [x] 1.4 添加平台检测逻辑（`import.meta.env.TAURI`），使前端代码可在构建时选择 Electron 或 Tauri adapter
- [x] 1.5 为 Tauri adapter 编写单元测试（mock `@tauri-apps/api`）

## 2. Rust 类型定义（tauri-specta）

- [x] 2.1 在 `apps/stage-tamagotchi-tauri/src-tauri/` 添加 `specta` 和 `tauri-specta` 依赖
- [x] 2.2 定义 Window 相关 Rust 类型：`Rectangle`、`Point`、`Display`、`ResizeDirection`
- [x] 2.3 定义 AutoUpdater 相关 Rust 类型：`AutoUpdaterState`、`AutoUpdaterStatus`、`AutoUpdaterProgress`、`AutoUpdaterError`
- [x] 2.4 定义 MCP 相关 Rust 类型：`McpStdioConfigFile`、`McpStdioServerConfig`、`McpApplyResult`、`McpRuntimeStatus`、`McpToolDescriptor`、`McpCallToolPayload`、`McpCallToolResult`
- [x] 2.5 定义 Plugin 相关 Rust 类型：`PluginRegistrySnapshot`、`PluginManifestSummary`、`PluginHostDebugSnapshot`、`PluginCapabilityPayload`、`PluginCapabilityState`
- [x] 2.6 定义 Widget 相关 Rust 类型：`WidgetSnapshot`、`WidgetsAddPayload`
- [x] 2.7 定义 Notice/Request 相关 Rust 类型：`RequestWindowPayload`、`RequestWindowPending`
- [x] 2.8 配置 tauri-specta 自动生成 TypeScript bindings，验证生成的类型与 `eventa.ts` 一致

## 3. Window 管理 Commands

- [x] 3.1 实现 `window_get_bounds` Tauri command
- [x] 3.2 实现 `window_set_bounds` Tauri command
- [x] 3.3 实现 `window_set_ignore_mouse_events` Tauri command（含 forward 参数）
- [x] 3.4 实现 `window_set_vibrancy` Tauri command（macOS 条件编译）
- [x] 3.5 实现 `window_set_background_material` Tauri command（Windows 条件编译）
- [x] 3.6 实现 `window_resize` Tauri command（delta + direction）
- [x] 3.7 实现 `window_close` Tauri command

## 4. Screen 信息 Commands

- [x] 4.1 实现 `screen_get_all_displays` Tauri command（使用 Tauri 的 monitor API 或平台原生 API）
- [x] 4.2 实现 `screen_get_primary_display` Tauri command
- [x] 4.3 实现 `screen_get_cursor_screen_point` Tauri command
- [x] 4.4 实现 DIP 转换 commands：`screen_dip_to_screen_point`、`screen_dip_to_screen_rect`、`screen_screen_to_dip_point`、`screen_screen_to_dip_rect`

## 5. 持续事件流

- [x] 5.1 实现 `start_tracking_mouse_position` command + `cursor_screen_point` Tauri event（tokio interval task，≤50ms）
- [x] 5.2 实现 `start_loop_get_bounds` command + `window_bounds` Tauri event
- [x] 5.3 实现 `start_dragging_window` command（Tauri 的 `window.start_dragging()`）

## 6. App 生命周期 & 窗口打开 Commands

- [x] 6.1 实现 `app_quit` Tauri command（含清理逻辑）
- [x] 6.2 实现 `open_main_devtools` Tauri command
- [x] 6.3 实现 `open_settings` Tauri command（创建或聚焦设置窗口）
- [x] 6.4 实现 `open_chat` Tauri command
- [x] 6.5 实现 `open_devtools_window` Tauri command（含可选 route 参数）

## 7. Widget & Notice & Caption Commands

- [x] 7.1 实现 widgets commands：`widgets_open_window`、`widgets_add`、`widgets_remove`、`widgets_clear`、`widgets_update`、`widgets_fetch`、`widgets_prepare`
- [x] 7.2 实现 widgets Tauri events：`widgets_render`、`widgets_remove_event`、`widgets_clear_event`、`widgets_update_event`
- [x] 7.3 实现 notice commands：`notice_open`、`notice_action`、`notice_page_mounted`、`notice_page_unmounted`
- [x] 7.4 实现 caption commands：`caption_get_is_following_window` + `caption_is_following_window_changed` event

## 8. i18n Commands

- [x] 8.1 实现 `i18n_set_locale` Tauri command
- [x] 8.2 实现 `i18n_get_locale` Tauri command

## 9. 前端适配

- [x] 9.1 在 `apps/stage-tamagotchi-tauri/src/` 中创建 eventa 初始化代码，使用 Tauri adapter
- [x] 9.2 验证 `packages/stage-ui` 中使用 eventa 的 composables 可通过 Tauri adapter 正常工作
- [x] 9.3 验证 `packages/stage-ui` 中使用 eventa 的 stores 可通过 Tauri adapter 正常工作
- [x] 9.4 处理 `packages/electron-eventa` 中 re-export 的 `electron` 对象在 Tauri 环境下的替代

## 10. 验证

- [x] 10.1 编写集成测试：验证 window commands 在 Tauri 中正常工作
- [x] 10.2 编写集成测试：验证 screen commands 返回正确的显示器信息
- [x] 10.3 编写集成测试：验证事件流（鼠标追踪、窗口 bounds）正常推送
- [x] 10.4 验证 tauri-specta 生成的 TypeScript 类型与 `eventa.ts` 中的接口兼容
- [x] 10.5 运行 `pnpm typecheck` 确保无类型错误
