## Why

当前桌面端 `apps/stage-tamagotchi` 基于 Electron 构建，打包体积约 150MB+，内存占用高，启动慢。项目在 `crates/` 下已有 6 个 Tauri v2 插件（MCP、rdev、window-pass-through、window-router-link、audio-transcription-ort、audio-vad-ort），说明团队此前已做过 Tauri 探索。随着 `tauri-nspanel` v2.1 解决了 macOS Panel 窗口问题、Safari 26 支持 WebGPU、`scap` crate 提供跨平台屏幕捕获，此前阻碍迁移的三大技术障碍已基本消除。现在是重新启动 Tauri 迁移的合适时机。

本 change 聚焦于搭建 Tauri v2 应用骨架——最小可运行的桌面应用框架，为后续 IPC 迁移、窗口管理、屏幕捕获等 change 提供基础。

## What Changes

- **BREAKING** 新增 Tauri v2 应用入口 `apps/stage-tamagotchi-tauri/`，与现有 Electron 版本并行存在
- 配置 Tauri v2 的 `tauri.conf.json`、`Cargo.toml`、capability permissions
- 集成现有 `crates/` 下的 Tauri 插件（tauri-plugin-mcp、tauri-plugin-rdev 等）到新应用
- 配置 Vite 构建管线，复用现有 Vue renderer 代码（`packages/stage-ui`、`packages/stage-ui-three` 等）
- 设置基础窗口（单窗口，透明、无边框），验证 Vue 前端可正常渲染
- 配置 `injeca` DI 的 Tauri 适配层
- 添加 pnpm workspace 配置和 turbo 构建任务
- 设置跨平台构建配置（macOS、Windows、Linux）

## Capabilities

### New Capabilities

- `tauri-app-entry`: Tauri v2 应用入口点配置，包括 `tauri.conf.json`、Rust main 入口、capability permissions、安全策略
- `tauri-vite-build`: Vite 构建管线适配 Tauri，包括 dev/build 命令、HMR 配置、资源路径处理
- `tauri-plugin-registry`: 现有 crates/ 插件的注册和初始化机制，插件依赖管理
- `tauri-workspace-config`: pnpm workspace、turbo pipeline、跨平台构建脚本配置

### Modified Capabilities

（无现有 specs 需要修改）

## Impact

- 新增 `apps/stage-tamagotchi-tauri/` 目录（Tauri 应用）
- 新增 `apps/stage-tamagotchi-tauri/src-tauri/` 目录（Rust 后端）
- 修改根 `Cargo.toml` workspace members
- 修改根 `pnpm-workspace.yaml` 或 `package.json` workspaces
- 修改 `turbo.json` 添加新的构建任务
- 依赖：`tauri@2.x`、`tauri-build`、`tauri-plugin-shell`、现有 crates 插件
- 现有 Electron 应用不受影响，两者并行存在直到迁移完成
