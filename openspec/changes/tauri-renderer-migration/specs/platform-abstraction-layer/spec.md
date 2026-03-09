## ADDED Requirements

### Requirement: Platform detection and adapter selection

The system SHALL 提供运行时和构建时的平台检测机制，使前端代码能透明切换 Electron 和 Tauri 后端：
- 构建时：通过 `import.meta.env.TAURI` 环境变量（Tauri 自动注入）
- 运行时：通过 `window.__TAURI__` 全局对象检测

#### Scenario: Tauri build uses Tauri adapter
- **WHEN** 前端代码在 Tauri 应用中构建
- **THEN** `import.meta.env.TAURI` 为 truthy，构建时 tree-shake 掉 Electron adapter 代码

#### Scenario: Electron build uses Electron adapter
- **WHEN** 前端代码在 Electron 应用中构建
- **THEN** `import.meta.env.TAURI` 为 falsy，使用现有 Electron adapter

### Requirement: Platform-agnostic composable wrappers

The system SHALL 为 `packages/stage-ui` 中依赖平台 API 的 composables 提供平台无关的包装层。每个包装 composable MUST：
- 在内部根据平台选择正确的实现
- 对外暴露相同的 API 签名
- 在不支持的平台上提供 graceful fallback（返回默认值或 noop）

#### Scenario: useWindowBounds works on both platforms
- **WHEN** `useWindowBounds()` composable 在 Tauri 应用中调用
- **THEN** 通过 Tauri invoke 获取窗口 bounds，返回与 Electron 版本相同的 reactive 数据结构

#### Scenario: Unsupported feature fallback
- **WHEN** 某个 Electron-only 功能（如 `setVibrancy`）在不支持的平台上调用
- **THEN** 函数静默返回，不抛出错误，控制台输出 warning
