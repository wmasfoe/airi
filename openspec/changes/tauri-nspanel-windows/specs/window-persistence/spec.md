## ADDED Requirements

### Requirement: Window bounds persistence

The system SHALL 持久化窗口的位置和尺寸，使用 `tauri-plugin-window-state` 或自定义配置文件。以下窗口的 bounds MUST 被持久化：
- Main 窗口（x、y、width、height）
- Settings 窗口
- Chat 窗口
- Devtools 窗口

#### Scenario: Restore window position on restart
- **WHEN** 用户关闭并重新启动应用
- **THEN** 主窗口在上次关闭时的位置和尺寸恢复显示

#### Scenario: First launch without saved state
- **WHEN** 应用首次启动，无持久化数据
- **THEN** 窗口使用默认尺寸和居中位置

#### Scenario: Bounds update on move/resize
- **WHEN** 用户移动或调整主窗口大小
- **THEN** 新的 bounds 自动保存到持久化存储

### Requirement: Window state storage format

The system SHALL 将窗口状态存储在应用的 userData 目录中，格式与现有 Electron 版本的配置兼容（JSON 格式，包含 `windows` 数组，每个元素有 `title`、`tag`、`x`、`y`、`width`、`height`）。

#### Scenario: Config file structure
- **WHEN** 窗口状态被保存
- **THEN** 配置文件包含 `{ windows: [{ title: "AIRI", tag: "main", x: 100, y: 200, width: 450, height: 600 }] }` 格式的数据

### Requirement: Multi-monitor awareness

The system SHALL 在恢复窗口位置时检查目标位置是否在当前可用显示器范围内。如果保存的位置在屏幕外（例如外接显示器已断开），MUST 将窗口重置到主显示器居中位置。

#### Scenario: Saved position on disconnected monitor
- **WHEN** 窗口上次保存在外接显示器上，但当前只有内置显示器
- **THEN** 窗口在内置显示器居中显示，而非在屏幕外不可见
