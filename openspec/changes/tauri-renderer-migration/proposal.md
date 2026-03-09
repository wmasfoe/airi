## Why

AIRI 的 Vue renderer 层（`packages/stage-ui`、`packages/stage-ui-three`、`packages/stage-shared`）是纯 Web 代码，理论上可以直接在 Tauri 的 webview 中运行。但实际上，renderer 中有大量代码通过 eventa IPC 调用 Electron main process API，且依赖 Electron 特有的 `contextBridge`/`preload` 安全模型。

此外，Tauri 在 macOS 上使用 WKWebView（Safari 内核），在 Windows 上使用 WebView2（Chromium 内核），在 Linux 上使用 WebKitGTK。需要验证 Three.js、MediaPipe（WebGPU）、以及其他 Web API 在这些 webview 中的兼容性。

本 change 聚焦于确保 Vue 前端在 Tauri webview 中正确运行，处理平台差异。

## What Changes

- 移除 renderer 层对 Electron `contextBridge`/`preload` 的依赖，改用 Tauri 的 `@tauri-apps/api`
- 创建平台抽象层，使 composables/stores 能同时支持 Electron 和 Tauri 后端
- 验证并适配 Three.js 在 WKWebView/WebView2/WebKitGTK 中的渲染
- 验证 MediaPipe WebGPU 在 WKWebView（Safari 26+）中的兼容性
- 处理 CSS/样式在不同 webview 引擎中的差异
- 适配文件路径和资源加载（Tauri 的 `convertFileSrc`/`asset` 协议）
- 处理 CSP（Content Security Policy）差异

## Capabilities

### New Capabilities

- `platform-abstraction-layer`: 前端平台抽象层，使 composables/stores 能透明切换 Electron/Tauri 后端
- `webview-compatibility`: Three.js、MediaPipe、WebGPU 在 WKWebView/WebView2/WebKitGTK 中的兼容性验证和适配
- `tauri-asset-protocol`: Tauri 资源协议适配，文件路径转换，CSP 配置

### Modified Capabilities

（无现有 specs 需要修改）

## Impact

- 修改 `packages/stage-ui/src/composables/` 中依赖 Electron API 的 composables
- 修改 `packages/stage-ui/src/stores/` 中依赖 IPC 的 stores
- 可能需要修改 `packages/stage-ui-three/` 中的 Three.js 初始化代码
- 新增平台检测和抽象层代码
- 影响所有使用 `electron` 或 `ipcRenderer` 的前端代码
