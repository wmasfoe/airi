## Context

AIRI 的 Vue renderer 层主要在 `packages/stage-ui`、`packages/stage-ui-three`、`packages/stage-shared` 中，是纯 Web 代码。但 renderer 中有大量代码通过 eventa IPC 调用 Electron main process API，且依赖 Electron 的 `contextBridge`/`preload` 安全模型。

Tauri 在不同平台使用不同 webview 引擎：macOS WKWebView（Safari）、Windows WebView2（Chromium）、Linux WebKitGTK。需要处理 API 兼容性差异。

## Goals / Non-Goals

**Goals:**
- 创建平台抽象层，使 composables/stores 能同时支持 Electron 和 Tauri
- 验证 Three.js 和 MediaPipe 在各 webview 中的兼容性
- 处理资源加载和 CSP 配置
- 处理 CSS 渲染差异

**Non-Goals:**
- 不重写 composables/stores 的业务逻辑
- 不修改 `packages/stage-ui` 的组件 API
- 不处理 IPC 合约迁移（属于 `tauri-ipc-migration`）

## Decisions

### 1. 平台抽象：条件导入 + Vite define

**选择**: 使用 Vite 的 `define` 配置注入平台标识，结合条件动态 import 选择 adapter。

```ts
// packages/stage-shared/src/platform.ts
export const isTauri = !!import.meta.env.TAURI
export const isElectron = !isTauri && !!window.electron
```

Composables 内部使用条件导入：
```ts
const adapter = isTauri
  ? await import('./adapters/tauri')
  : await import('./adapters/electron')
```

**替代方案**:
- 编译时完全替换（alias）：不够灵活，难以测试
- 运行时 DI：增加复杂度

**理由**: 构建时 tree-shaking 移除不需要的 adapter，运行时零开销。

### 2. WebGPU 策略：渐进增强

**选择**: Three.js 默认使用 WebGL2，在检测到 WebGPU 支持时自动升级。MediaPipe 在不支持 WebGPU 的环境中禁用。

**理由**: WebGL2 在所有 webview 中都支持，WebGPU 是增强而非必需。

### 3. 资源加载：Vite public 目录 + Tauri asset scope

**选择**: 静态资源放在 Vite 的 `public/` 目录，Tauri 自动将其打包。动态资源（用户数据）通过 `convertFileSrc()` 转换路径。

**理由**: 与现有 Vite 构建流程一致，无需额外配置。

## Risks / Trade-offs

- **[Risk] WKWebView CSS 差异**: Safari 的 CSS 渲染与 Chromium 有差异 → 使用 autoprefixer + 兼容性测试
- **[Risk] MediaPipe WKWebView 兼容性**: 未经验证 → 需要实际测试，准备 fallback
- **[Trade-off] 双 adapter 代码量**: 每个平台相关 composable 需要两份实现 → 抽象层尽量薄

## Open Questions

- `packages/stage-ui` 中有多少 composables 直接依赖 Electron API？需要逐一审计
- WKWebView 是否支持 `OffscreenCanvas`（Three.js worker rendering 需要）？
- Tauri 的 CSP 配置是否支持 `unsafe-eval`（Vue 运行时模板编译可能需要）？
