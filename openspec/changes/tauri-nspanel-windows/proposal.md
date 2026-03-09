## Why

AIRI 的核心体验是一个始终浮在其他应用上方的角色窗口（桌面宠物），这依赖 Electron 的 `type: 'panel'`（macOS NSPanel）和 `setAlwaysOnTop(true, 'screen-saver', level)` 实现。项目有 9 个不同类型的窗口（main、caption、widgets、settings、chat、devtools、about、notice、beat-sync），每个都有独特的配置（透明度、置顶层级、点击穿透等）。

`tauri-nspanel` v2.1 现在提供了完整的 NSPanel PanelBuilder API，已被 BongoCat（桌面宠物，同类型应用）等项目验证。项目自身也有 `tauri-plugin-window-pass-through-on-hover` 插件。本 change 将这些能力整合，实现完整的多窗口管理。

## What Changes

- 集成 `tauri-nspanel` v2.1，将主窗口转换为 macOS NSPanel（浮动、全屏覆盖、所有工作区可见）
- 实现 9 个窗口的 Tauri 配置：
  - Main：Panel 类型，透明，always-on-top（screen-saver 级别），所有工作区可见
  - Caption：Panel 类型，透明，always-on-top（level 2）
  - Widgets：Panel 类型，透明，always-on-top（level 1），点击穿透
  - Settings/Chat/Dashboard/About/Notice/Devtools：标准窗口
  - Beat-Sync：特殊窗口
- 集成现有 `tauri-plugin-window-pass-through-on-hover` 实现点击穿透
- 实现窗口位置/尺寸持久化（`tauri-plugin-window-state` 或自定义）
- 实现 macOS vibrancy（`setVibrancy('hud')`）和 Windows acrylic 效果
- 实现窗口拖拽（替代 `electron-click-drag-plugin`）
- Windows/Linux 平台的 always-on-top 替代方案

## Capabilities

### New Capabilities

- `nspanel-integration`: macOS NSPanel 集成，PanelBuilder 配置，浮动层级管理，全屏覆盖行为
- `multi-window-manager`: 9 个窗口的创建、配置、生命周期管理，窗口间通信
- `window-effects`: 透明度、vibrancy/acrylic 模糊效果、点击穿透、无边框拖拽的跨平台实现
- `window-persistence`: 窗口位置/尺寸的持久化和恢复

### Modified Capabilities

（无现有 specs 需要修改）

## Impact

- 修改 `apps/stage-tamagotchi-tauri/src-tauri/Cargo.toml` 添加 tauri-nspanel、window-state 依赖
- 新增窗口管理 Rust 模块 `apps/stage-tamagotchi-tauri/src-tauri/src/windows/`
- 复用 `crates/tauri-plugin-window-pass-through-on-hover`
- 依赖：`tauri-nspanel@v2.1`、`tauri-plugin-window-state`、`objc2`（macOS）
- 平台差异：macOS 使用 NSPanel，Windows/Linux 使用标准 always-on-top 窗口
