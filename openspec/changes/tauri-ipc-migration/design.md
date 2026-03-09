## Context

AIRI 的 Electron 应用使用 `@moeru/eventa` 框架在 main process 和 renderer 之间通信。eventa 提供两种原语：
- `defineInvokeEventa<TReturn, TPayload>(channel)`: 请求/响应模式（类似 RPC）
- `defineEventa<TPayload>(channel)`: 单向事件流

当前有约 50+ 个 IPC 合约定义在 `apps/stage-tamagotchi/src/shared/eventa.ts` 和 `packages/electron-eventa/` 中，覆盖窗口管理、屏幕信息、自动更新、MCP、插件系统、widgets、i18n、notice 等功能。

Tauri 的 IPC 机制完全不同：
- `invoke(command, args)`: 前端调用 Rust command（类似 eventa invoke）
- `listen(event, callback)` / `emit(event, payload)`: 事件系统（类似 eventa event）

## Goals / Non-Goals

**Goals:**
- 创建 eventa 的 Tauri adapter，使现有合约定义可复用
- 使用 tauri-specta 实现 Rust ↔ TypeScript 类型安全
- 逐一迁移所有 IPC 合约到 Tauri commands/events
- 保持前端 composables/stores 的 API 不变或最小变更

**Non-Goals:**
- 不重写 eventa 框架本身
- 不修改 Electron 版本的 IPC 实现
- 不实现 MCP/Plugin/ServerChannel 的业务逻辑（只迁移 IPC 层，业务逻辑在其他 change 中）

## Decisions

### 1. Adapter 策略：薄适配层 + 平台检测

**选择**: 创建 `@moeru/eventa/adapters/tauri`，提供与 Electron adapter 相同的 API surface。前端代码通过构建时环境变量（`import.meta.env.TAURI`）选择 adapter。

**替代方案**:
- 完全重写前端调用为直接 `@tauri-apps/api` 调用：改动量巨大，且失去 eventa 的类型安全和抽象
- 运行时检测 `window.__TAURI__`：可行但不如构建时干净

**理由**: Tauri 注入 `import.meta.env.TAURI` 环境变量，Vite 可在构建时 tree-shake 掉不需要的 adapter。这样同一份前端代码可同时支持 Electron 和 Tauri。

### 2. Command 命名：自动映射 + 手动注册

**选择**: Tauri adapter 自动将 eventa channel name 映射为 snake_case command name，但 Rust 侧仍需手动注册每个 command。

映射规则：
```
eventa:invoke:electron:window:close → window_close
eventa:invoke:electron:mcp:call-tool → mcp_call_tool
eventa:event:electron:auto-updater:state-changed → auto_updater_state_changed
```

**替代方案**:
- 使用 Tauri 的 `invoke("raw_channel_name")` 直接传递 eventa channel name：Tauri command name 必须是合法 Rust 标识符
- 代码生成：过度工程化，合约数量可控

**理由**: 映射规则简单确定，Rust 侧用 `#[tauri::command]` 宏注册时使用映射后的名称。

### 3. 类型桥接：tauri-specta 生成 + 手动对齐

**选择**: Rust 侧用 `#[derive(Serialize, Deserialize, specta::Type)]` 定义类型，tauri-specta 生成 TypeScript bindings。生成的类型与现有 eventa.ts 中的接口手动对齐验证。

**替代方案**:
- 只用 serde 不用 specta：失去自动类型生成，需要手动维护两份类型
- 从 TypeScript 生成 Rust 类型：工具链不成熟

**理由**: tauri-specta 是 Tauri 生态的标准方案，与 `specta` crate 配合可自动生成准确的 TypeScript 类型。

### 4. 事件流：Tauri event system

**选择**: 将 eventa 的 `defineEventa`（单向事件）映射到 Tauri 的 `app.emit()` / `listen()`。持续事件流（如鼠标位置追踪）通过 Rust 侧 tokio task 定期 emit。

**替代方案**:
- WebSocket 通道：过度复杂
- SharedArrayBuffer：浏览器兼容性问题

**理由**: Tauri 的 event system 天然支持 Rust → 前端的事件推送，性能足够（鼠标追踪 50ms 间隔）。

### 5. 迁移顺序：按功能域分批

**选择**: 按以下顺序迁移，每批可独立验证：
1. Window 管理（getBounds、setBounds、close 等）— 最基础
2. Screen 信息（displays、cursor）— 窗口管理依赖
3. App 生命周期（quit、open windows）— 基础功能
4. i18n（set/get locale）— 简单，快速验证
5. Widgets（add/remove/update）— 中等复杂度
6. Notice（request/response 模式）— 中等复杂度
7. Caption overlay — 依赖窗口管理
8. MCP/Plugin/ServerChannel — 业务逻辑复杂，最后迁移

## Risks / Trade-offs

- **[Risk] eventa adapter 抽象泄漏**: Tauri 的 invoke 是异步的但不支持 streaming，而 eventa 的某些用法可能假设同步行为 → 审查所有 invoke 调用确保异步兼容
- **[Risk] 事件序列化开销**: 高频事件（鼠标位置 50ms）的 JSON 序列化可能有性能影响 → 使用 Tauri 的 binary event payload 或降低频率
- **[Trade-off] 双 adapter 维护**: 同时维护 Electron 和 Tauri adapter 增加复杂度 → 迁移完成后移除 Electron adapter
- **[Risk] tauri-specta 版本兼容**: specta 和 tauri-specta 版本需要与 Tauri 2.x 兼容 → 锁定已验证的版本组合
- **[Risk] Window-scoped events**: Tauri 的 event 可以是 app-global 或 window-scoped，需要正确选择 → 参考 eventa 的 context 机制决定 scope

## Open Questions

- eventa 的 `createContext` 是否需要在 Tauri adapter 中实现 window-scoped context，还是所有 commands 都是 app-global？
- 是否需要为 Tauri adapter 创建独立的 npm 包（`@proj-airi/tauri-eventa`），还是作为 `@moeru/eventa` 的内置 adapter？
- 高频事件（鼠标追踪）是否应该使用 Tauri 的 channel API 而非普通 event？
