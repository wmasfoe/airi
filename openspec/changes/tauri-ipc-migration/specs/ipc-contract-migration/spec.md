## ADDED Requirements

### Requirement: Window management IPC contracts

The system SHALL 实现以下 window 管理 Tauri commands，行为与现有 Electron eventa 合约一致：

- `window_get_bounds`: 返回当前窗口的 `{ x, y, width, height }`
- `window_set_bounds`: 设置窗口位置和尺寸
- `window_set_ignore_mouse_events`: 设置窗口点击穿透（forward 参数控制是否转发事件）
- `window_set_vibrancy`: 设置 macOS vibrancy 效果（`hud`、`sidebar` 等或 `null`）
- `window_set_background_material`: 设置 Windows acrylic/mica 效果
- `window_resize`: 按 delta 和方向调整窗口尺寸
- `window_close`: 关闭当前窗口

#### Scenario: Get and set window bounds
- **WHEN** 前端调用 `window_get_bounds`
- **THEN** 返回 `{ x: number, y: number, width: number, height: number }` 格式的窗口位置和尺寸

#### Scenario: Click-through with event forwarding
- **WHEN** 前端调用 `window_set_ignore_mouse_events(true, { forward: true })`
- **THEN** 窗口变为点击穿透，但鼠标移动事件仍被转发到窗口

#### Scenario: macOS vibrancy effect
- **WHEN** 前端调用 `window_set_vibrancy("hud")` on macOS
- **THEN** 窗口背景应用 HUD vibrancy 模糊效果

### Requirement: Screen information IPC contracts

The system SHALL 实现以下 screen 信息 Tauri commands：

- `screen_get_all_displays`: 返回所有显示器信息（分辨率、缩放、位置）
- `screen_get_primary_display`: 返回主显示器信息
- `screen_get_cursor_screen_point`: 返回当前鼠标光标的屏幕坐标
- `screen_dip_to_screen_point`: DIP 坐标转屏幕坐标
- `screen_dip_to_screen_rect`: DIP 矩形转屏幕矩形
- `screen_screen_to_dip_point`: 屏幕坐标转 DIP 坐标
- `screen_screen_to_dip_rect`: 屏幕矩形转 DIP 矩形

#### Scenario: Multi-monitor display enumeration
- **WHEN** 前端调用 `screen_get_all_displays` on a multi-monitor setup
- **THEN** 返回所有显示器的信息数组，每个包含 `bounds`、`workArea`、`scaleFactor`

#### Scenario: Cursor position tracking
- **WHEN** 前端调用 `screen_get_cursor_screen_point`
- **THEN** 返回 `{ x: number, y: number }` 格式的当前鼠标屏幕坐标

### Requirement: Continuous event streams

The system SHALL 实现以下持续事件流，替代 Electron 的 eventa event 合约：

- `start_tracking_mouse_position`: 启动鼠标位置追踪，定期通过 Tauri event 发送 `cursor_screen_point` 事件
- `start_loop_get_bounds`: 启动窗口 bounds 追踪，窗口变化时通过 Tauri event 发送 `window_bounds` 事件

#### Scenario: Mouse position stream
- **WHEN** 前端调用 `start_tracking_mouse_position` 并监听 `cursor_screen_point` 事件
- **THEN** 每隔固定间隔（≤50ms）收到鼠标位置更新事件

#### Scenario: Window bounds change stream
- **WHEN** 前端调用 `start_loop_get_bounds` 并监听 `window_bounds` 事件
- **THEN** 窗口移动或调整大小时收到 bounds 更新事件

### Requirement: Application lifecycle IPC contracts

The system SHALL 实现以下应用生命周期 Tauri commands：

- `app_quit`: 退出应用
- `window_close`: 关闭当前窗口（已在 window 合约中定义）
- `open_main_devtools`: 打开主窗口 DevTools
- `open_settings`: 打开设置窗口
- `open_chat`: 打开聊天窗口
- `open_devtools_window`: 打开独立 DevTools 窗口（可选 route 参数）

#### Scenario: Quit application
- **WHEN** 前端调用 `app_quit`
- **THEN** 应用执行清理（关闭 WebSocket 服务器、停止 MCP 进程等）后退出

#### Scenario: Open settings window
- **WHEN** 前端调用 `open_settings`
- **THEN** 设置窗口创建并显示（如已存在则聚焦）

### Requirement: Widget management IPC contracts

The system SHALL 实现以下 widget 管理 Tauri commands：

- `widgets_open_window`: 打开 widgets 窗口
- `widgets_add`: 添加 widget，返回 widget ID
- `widgets_remove`: 移除指定 widget
- `widgets_clear`: 清除所有 widgets
- `widgets_update`: 更新 widget 的 componentProps
- `widgets_fetch`: 获取指定 widget 的快照
- `widgets_prepare`: 准备 widget 窗口

以及 Tauri events（Rust → 前端）：
- `widgets_render`: 通知 renderer 渲染新 widget
- `widgets_remove_event`: 通知 renderer 移除 widget
- `widgets_clear_event`: 通知 renderer 清除所有 widgets
- `widgets_update_event`: 通知 renderer 更新 widget props

#### Scenario: Add and render widget
- **WHEN** 前端调用 `widgets_add({ componentName: "WeatherWidget", size: "m" })`
- **THEN** 返回 widget ID，且 widgets 窗口的 renderer 收到 `widgets_render` 事件包含完整 `WidgetSnapshot`

#### Scenario: Remove widget with cleanup
- **WHEN** 前端调用 `widgets_remove({ id: "widget-123" })`
- **THEN** widget 从内部状态移除，widgets renderer 收到 `widgets_remove_event`

### Requirement: i18n IPC contracts

The system SHALL 实现以下国际化 Tauri commands：

- `i18n_set_locale`: 设置应用语言
- `i18n_get_locale`: 获取当前语言

#### Scenario: Set locale propagates to all windows
- **WHEN** 前端调用 `i18n_set_locale("ja")`
- **THEN** 所有窗口的 locale 更新为日语

#### Scenario: Get locale returns current setting
- **WHEN** 前端调用 `i18n_get_locale`
- **THEN** 返回当前设置的 locale 字符串（如 `"en"`、`"zh-CN"`、`"ja"`）

### Requirement: Notice window IPC contracts

The system SHALL 实现 notice 窗口的请求/响应模式 Tauri commands：

- `notice_open`: 打开 notice 窗口，传入 route 和 payload
- `notice_action`: 用户在 notice 窗口中执行操作（confirm/cancel/close）
- `notice_page_mounted`: notice 页面挂载后获取 pending 数据
- `notice_page_unmounted`: notice 页面卸载通知

#### Scenario: Open notice and receive user action
- **WHEN** 前端调用 `notice_open({ route: "/confirm-delete", payload: { itemId: "123" } })`
- **THEN** notice 窗口打开并显示对应路由，用户操作后通过 `notice_action` 返回结果

### Requirement: Caption overlay IPC contracts

The system SHALL 实现 caption overlay 窗口的 Tauri commands：

- `caption_get_is_following_window`: 获取 caption 是否跟随主窗口
- Tauri event `caption_is_following_window_changed`: caption 跟随状态变化通知

#### Scenario: Caption follows main window
- **WHEN** caption overlay 设置为跟随主窗口
- **THEN** `caption_get_is_following_window` 返回 `true`，且主窗口移动时 caption 窗口同步移动
