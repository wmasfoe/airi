## ADDED Requirements

### Requirement: macOS NSPanel window type via tauri-nspanel

The system SHALL 使用 `tauri-nspanel` v2.1 将主窗口、caption 窗口、widgets 窗口转换为 macOS NSPanel 类型。Panel 窗口 MUST 具备以下行为：
- 浮动在所有普通窗口上方
- 在全屏应用上方可见
- 在所有 macOS 工作区（Spaces）可见
- 不出现在 Mission Control 和 App Exposé 中

#### Scenario: Main window floats above fullscreen apps
- **WHEN** 用户在 macOS 上运行全屏应用（如浏览器全屏模式）
- **THEN** AIRI 主窗口仍然可见，浮动在全屏应用上方

#### Scenario: Main window visible on all Spaces
- **WHEN** 用户在 macOS 上切换到不同的 Space/Desktop
- **THEN** AIRI 主窗口在所有 Space 中可见

#### Scenario: Panel not in Mission Control
- **WHEN** 用户触发 Mission Control
- **THEN** AIRI 的 Panel 窗口不作为独立窗口出现在 Mission Control 视图中

### Requirement: Panel level hierarchy

The system SHALL 为不同 Panel 窗口设置不同的浮动层级：
- Main 窗口：`PanelLevel::ScreenSaver`（最高，等同于 Electron 的 `setAlwaysOnTop(true, 'screen-saver', 1)`）
- Caption 窗口：`PanelLevel::Floating` + level 2
- Widgets 窗口：`PanelLevel::Floating` + level 1

#### Scenario: Caption window above widgets but below main
- **WHEN** main、caption、widgets 三个窗口同时显示
- **THEN** main 窗口在最上层，caption 在中间层，widgets 在最下层

### Requirement: Windows/Linux fallback for panel behavior

The system SHALL 在 Windows 和 Linux 上使用 Tauri 的标准 `set_always_on_top(true)` 作为 NSPanel 的替代方案。Windows/Linux 上不支持全屏覆盖行为。

#### Scenario: Windows always-on-top
- **WHEN** 应用在 Windows 上运行
- **THEN** 主窗口使用 `set_always_on_top(true)` 保持在普通窗口上方（但不能覆盖全屏应用）

#### Scenario: Conditional compilation
- **WHEN** 应用在非 macOS 平台编译
- **THEN** `tauri-nspanel` 相关代码通过 `#[cfg(target_os = "macos")]` 条件编译排除
