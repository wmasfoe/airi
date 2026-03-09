## ADDED Requirements

### Requirement: Plugin system Tauri adapter

The system SHALL 提供插件系统的 Tauri 适配，使现有的 `@proj-airi/plugin-sdk` 插件能在 Tauri 应用中运行。插件宿主 MUST 支持：
- Manifest v1 格式的插件发现（扫描 `{userData}/plugins/v1/` 目录）
- 插件启用/禁用状态持久化
- 插件加载/卸载生命周期
- 能力声明和查询

#### Scenario: List installed plugins
- **WHEN** 前端调用 `plugin_list`
- **THEN** 返回 `PluginRegistrySnapshot`，包含所有已发现插件的名称、路径、启用状态

#### Scenario: Enable and load plugin
- **WHEN** 前端调用 `plugin_set_enabled({ name: "my-plugin", enabled: true })`
- **THEN** 插件状态更新为启用，自动加载插件，返回更新后的 registry snapshot

#### Scenario: Plugin capability announcement
- **WHEN** 插件声明了 `providers` 能力
- **THEN** 前端可通过 `plugin_list_providers` 查询到该插件提供的 provider 列表

### Requirement: Plugin host implementation strategy

The system SHALL 在 Rust 侧实现插件宿主的核心逻辑（manifest 解析、文件系统扫描、状态持久化），插件的 JavaScript 运行时通过 webview 执行。

#### Scenario: Manifest parsing
- **WHEN** 插件目录包含有效的 manifest.json
- **THEN** Rust 侧正确解析 manifest 并注册插件

#### Scenario: Invalid manifest handling
- **WHEN** 插件目录包含无效的 manifest.json
- **THEN** 该插件被跳过，错误信息记录到日志，不影响其他插件
