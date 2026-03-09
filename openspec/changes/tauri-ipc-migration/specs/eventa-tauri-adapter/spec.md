## ADDED Requirements

### Requirement: Tauri adapter for eventa framework

The system SHALL provide a Tauri adapter for `@moeru/eventa`（位于 `@moeru/eventa/adapters/tauri` 或 `packages/tauri-eventa`），使 `defineInvokeEventa` 和 `defineEventa` 定义的合约能通过 Tauri 的 `invoke` 和 `listen`/`emit` 机制通信。

Adapter MUST 实现以下核心功能：
- `createContext(webviewWindow)`: 创建 Tauri 侧的 eventa 上下文，绑定到指定 webview window
- `defineInvokeHandler(eventa, handler)`: 在 Rust 侧注册 invoke command handler
- 前端侧的 `invoke(eventa, payload)`: 通过 `@tauri-apps/api/core.invoke()` 调用 Rust command
- 前端侧的 `listen(eventa, callback)`: 通过 `@tauri-apps/api/event.listen()` 监听 Rust 事件
- 前端侧的 `emit(eventa, payload)`: 通过 `@tauri-apps/api/event.emit()` 发送事件到 Rust

#### Scenario: Invoke eventa from frontend to Rust
- **WHEN** 前端调用 `invoke(electronWindowClose)` 
- **THEN** Tauri adapter 将 eventa channel name 映射为 Tauri command name，通过 `@tauri-apps/api/core.invoke()` 调用对应的 Rust handler

#### Scenario: Listen to Rust events in frontend
- **WHEN** Rust 侧通过 `app.emit("eventa:event:...", payload)` 发送事件
- **THEN** 前端通过 `listen(someEventa, callback)` 注册的回调被触发，payload 类型正确

#### Scenario: Adapter coexists with Electron adapter
- **WHEN** 项目同时包含 `@moeru/eventa/adapters/electron` 和 `@moeru/eventa/adapters/tauri`
- **THEN** 两个 adapter 互不干扰，可通过构建时条件或运行时检测选择使用哪个

### Requirement: Channel name to Tauri command mapping

The system SHALL 提供一个确定性的映射规则，将 eventa channel name（如 `eventa:invoke:electron:window:close`）转换为合法的 Tauri command name（Rust 函数名，snake_case，无冒号）。

映射规则 MUST：
- 将 `eventa:invoke:electron:` 前缀替换为空
- 将 `-` 和 `:` 替换为 `_`
- 结果为合法的 Rust 标识符（snake_case）

例如：`eventa:invoke:electron:window:close` → `window_close`

#### Scenario: Deterministic mapping
- **WHEN** 两个不同的 eventa channel name 被映射
- **THEN** 它们产生不同的 Tauri command name，不会冲突

#### Scenario: Round-trip consistency
- **WHEN** 前端使用 eventa 合约调用 invoke
- **THEN** adapter 自动将 channel name 映射为正确的 Tauri command name，无需手动指定
