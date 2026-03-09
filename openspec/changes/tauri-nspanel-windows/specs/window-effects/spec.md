## ADDED Requirements

### Requirement: Transparent frameless windows

The system SHALL 支持创建透明无边框窗口，配置等同于现有 Electron 的 `transparentWindowConfig()`：
- `frame: false`（无原生窗口框架）
- `transparent: true`（窗口背景透明）
- macOS 上 `titleBarStyle: 'hidden'`
- `hasShadow: false`（无窗口阴影）

#### Scenario: Main window is fully transparent
- **WHEN** 主窗口创建并显示
- **THEN** 窗口背景完全透明，只有 webview 中渲染的内容可见，无原生标题栏和边框

#### Scenario: Non-panel windows have standard frame
- **WHEN** settings 或 chat 窗口创建
- **THEN** 窗口有标准的操作系统窗口框架（或自定义标题栏）

### Requirement: Vibrancy and blur effects

The system SHALL 支持 macOS vibrancy 和 Windows acrylic/mica 背景模糊效果：
- macOS：`vibrancy: 'hud'`（等同于 Electron 的 `setVibrancy('hud')`）
- Windows：`backgroundMaterial: 'acrylic'`（等同于 Electron 的 `setBackgroundMaterial('acrylic')`）

#### Scenario: macOS HUD vibrancy
- **WHEN** 窗口在 macOS 上应用 vibrancy 效果
- **THEN** 窗口背景显示 HUD 风格的半透明模糊效果

#### Scenario: Windows acrylic effect
- **WHEN** 窗口在 Windows 上应用 background material
- **THEN** 窗口背景显示 acrylic 半透明模糊效果

### Requirement: Click-through windows

The system SHALL 支持窗口点击穿透功能，使用现有的 `tauri-plugin-window-pass-through-on-hover`：
- 设置 `ignore_mouse_events(true)` 使鼠标事件穿透窗口
- 支持 `forward` 参数，穿透时仍转发鼠标移动事件（用于 hover 检测）

#### Scenario: Widgets window click-through
- **WHEN** widgets 窗口设置为点击穿透模式
- **THEN** 鼠标点击穿过 widgets 窗口到达下方的应用

#### Scenario: Click-through with hover detection
- **WHEN** widgets 窗口设置为点击穿透且 forward=true
- **THEN** 鼠标点击穿透，但鼠标移动事件仍被 widgets 窗口接收（用于显示 hover 效果）

### Requirement: Window dragging

The system SHALL 支持无边框窗口的拖拽移动，替代 Electron 的 `electron-click-drag-plugin`。使用 Tauri 的 `window.start_dragging()` API 或 CSS `data-tauri-drag-region` 属性。

#### Scenario: Drag main window by content area
- **WHEN** 用户在主窗口的可拖拽区域按住鼠标并移动
- **THEN** 窗口跟随鼠标移动

#### Scenario: Non-draggable interactive elements
- **WHEN** 用户点击窗口内的按钮或输入框
- **THEN** 不触发窗口拖拽，正常处理交互事件

### Requirement: macOS window button visibility

The system SHALL 在 macOS 上隐藏 Panel 窗口的红绿灯按钮（关闭/最小化/最大化），等同于 Electron 的 `setWindowButtonVisibility(false)`。

#### Scenario: No traffic lights on main window
- **WHEN** 主窗口在 macOS 上显示
- **THEN** 窗口左上角不显示红绿灯按钮
