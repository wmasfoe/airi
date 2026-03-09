## ADDED Requirements

### Requirement: Window registry and lifecycle management

The system SHALL 维护一个窗口注册表，管理以下 12 个窗口的创建、显示、隐藏、销毁：

| 窗口 | 类型 | 透明 | Always-on-top | 特殊行为 |
|------|------|------|---------------|---------|
| main | Panel | 是 | screen-saver level 1 | 所有工作区可见 |
| caption | Panel | 是 | floating level 2 | 跟随主窗口 |
| widgets | Panel | 是 | floating level 1 | 点击穿透 |
| settings | 标准 | 否 | 否 | 单例，可复用 |
| chat | 标准 | 否 | 否 | 单例，可复用 |
| devtools | 标准 | 否 | 否 | 可多开 |
| about | 标准 | 否 | 否 | 单例，可复用 |
| notice | 标准 | 否 | 否 | 请求/响应模式 |
| dashboard | 标准 | 否 | 否 | 单例 |
| inlay | Panel | 是 | floating | 嵌入式覆盖 |
| beat-sync | 标准 | 否 | 否 | 特殊用途 |

#### Scenario: Create and show settings window
- **WHEN** 用户请求打开设置窗口
- **THEN** 如果设置窗口不存在则创建，如果已存在则聚焦显示

#### Scenario: Destroy window on close
- **WHEN** 用户关闭一个非主窗口
- **THEN** 窗口从注册表移除并销毁，释放资源

#### Scenario: Main window never destroyed
- **WHEN** 用户点击主窗口的关闭按钮
- **THEN** 主窗口隐藏而非销毁（应用继续在托盘运行）

### Requirement: Window creation with correct webview URL

The system SHALL 为每个窗口创建时加载正确的 webview URL。所有窗口共享同一个 Vite 构建产物，通过不同的 URL path/hash 路由到不同页面。

#### Scenario: Settings window loads settings route
- **WHEN** 设置窗口创建
- **THEN** webview 加载 `tauri://localhost/settings`（dev 模式）或对应的本地文件路径

#### Scenario: Devtools window loads with route parameter
- **WHEN** devtools 窗口以 `{ route: "/memory" }` 参数创建
- **THEN** webview 加载 `tauri://localhost/devtools/memory`

### Requirement: Window size and position defaults

The system SHALL 为每个窗口提供合理的默认尺寸和位置：
- Main：450x600，居中
- Settings：800x600，居中
- Chat：400x600，居中
- Devtools：1200x800，居中
- About：400x300，居中
- Notice：400x200，居中

#### Scenario: First launch uses defaults
- **WHEN** 应用首次启动，无持久化的窗口配置
- **THEN** 主窗口以 450x600 尺寸在屏幕中央显示
