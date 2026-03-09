## ADDED Requirements

### Requirement: System tray with context menu

The system SHALL 创建系统托盘图标，提供右键上下文菜单，功能等同于现有 Electron 实现：
- 窗口尺寸预设（推荐、全高、半高、全屏）
- 窗口对齐（居中、四角）
- 显示/隐藏主窗口
- 打开设置/DevTools
- 退出应用

#### Scenario: Right-click shows context menu
- **WHEN** 用户右键点击系统托盘图标
- **THEN** 显示包含窗口控制选项的上下文菜单

#### Scenario: Window size preset
- **WHEN** 用户选择"推荐尺寸"菜单项
- **THEN** 主窗口调整为 450x600 尺寸

#### Scenario: Double-click toggles window on macOS
- **WHEN** 用户在 macOS 上双击托盘图标
- **THEN** 主窗口在显示/隐藏之间切换

### Requirement: Dynamic menu updates

The system SHALL 在窗口状态变化时动态更新托盘菜单内容（如当前窗口尺寸、对齐状态）。

#### Scenario: Menu reflects current window size
- **WHEN** 用户手动调整窗口大小后右键托盘
- **THEN** 菜单中当前匹配的尺寸预设项显示为选中状态

### Requirement: Internationalized menu labels

The system SHALL 使用 `@proj-airi/i18n` 的翻译为托盘菜单项提供国际化标签。

#### Scenario: Japanese locale menu
- **WHEN** 应用语言设置为日语
- **THEN** 托盘菜单项显示日语标签
