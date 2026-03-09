## Why

当前 Electron 应用使用自研的 `@moeru/eventa` IPC 框架（通过 `packages/electron-eventa` 适配），在 `ipcMain`/`ipcRenderer` 上构建了类型安全的 RPC 系统。整个 renderer 层的 Vue composables、stores 都深度依赖 eventa 的 `defineInvokeHandler`、`createContext` 等 API。迁移到 Tauri 后，底层 IPC 机制完全不同（Tauri 使用 `invoke` commands + `listen`/`emit` events），eventa 的 Electron adapter 无法直接使用。

这是整个迁移中工作量最大的部分，因为 IPC 是连接 Rust 后端和 Vue 前端的核心桥梁，几乎所有功能都依赖它。

## What Changes

- **BREAKING** 创建 `@moeru/eventa` 的 Tauri adapter（`packages/tauri-eventa` 或 `@moeru/eventa/adapters/tauri`）
- 将现有 eventa 合约（`apps/stage-tamagotchi/src/shared/eventa`）迁移为 Tauri invoke commands
- 使用 `tauri-specta` 实现 Rust → TypeScript 的类型安全桥接
- 迁移 renderer 侧的 eventa composables 到 Tauri `invoke`/`listen` API
- 迁移以下 IPC 合约：
  - `electron.window`（getBounds、setBounds、setIgnoreMouseEvents、setVibrancy、resize、close）
  - `electron.screen`（getAllDisplays、getPrimaryDisplay、getCursorScreenPoint、DIP 转换）
  - `electron.systemPreferences`（媒体访问权限检查）
  - `electron.app`（生命周期、路径）
  - `electron-updater` 合约（getState、checkForUpdates、downloadUpdate、quitAndInstall）
  - MCP 服务器管理合约
  - 插件系统合约
  - Server Channel 合约

## Capabilities

### New Capabilities

- `eventa-tauri-adapter`: eventa 框架的 Tauri adapter 实现，将 eventa 的 defineInvokeHandler/createContext 映射到 Tauri invoke/listen
- `tauri-specta-bridge`: 使用 tauri-specta 生成类型安全的 TypeScript bindings，替代手动 IPC 类型定义
- `ipc-contract-migration`: 现有 IPC 合约（window、screen、app、updater、mcp、plugin、server-channel）的逐一迁移规范

### Modified Capabilities

（无现有 specs 需要修改）

## Impact

- 新增 `packages/tauri-eventa/`（或修改 `@moeru/eventa` 添加 Tauri adapter）
- 修改 `apps/stage-tamagotchi-tauri/src-tauri/src/` 添加 Tauri commands
- 修改 renderer 侧所有使用 eventa 的 composables 和 stores
- 依赖：`tauri-specta`、`specta`、`@tauri-apps/api`
- 影响范围广：几乎所有 main↔renderer 通信都需要适配
