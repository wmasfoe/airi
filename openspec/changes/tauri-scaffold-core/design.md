## Context

当前桌面端 `apps/stage-tamagotchi` 使用 Electron + electron-vite 构建。项目在 `crates/` 下已有 6 个 Tauri v2 插件，说明此前做过 Tauri 探索后迁移到了 Electron。现在 Tauri 生态补齐了关键短板（NSPanel、WebGPU），需要搭建一个与 Electron 版本并行的 Tauri 应用骨架。

关键约束：
- 现有 Electron 应用不能受影响，两者并行存在
- Vue renderer 代码（`packages/stage-ui` 等）应最大程度复用
- 现有 `crates/` 插件应直接集成，不重复开发
- Cargo workspace 已存在，需要扩展而非重建

## Goals / Non-Goals

**Goals:**
- 创建可编译、可运行的 Tauri v2 应用骨架
- 集成现有 6 个 Tauri 插件
- 配置 Vite 构建管线，使 Vue 前端在 Tauri webview 中正常渲染
- 配置 pnpm workspace 和 turbo pipeline
- 验证基础窗口（透明、无边框）可正常工作

**Non-Goals:**
- 不实现 IPC 迁移（属于 `tauri-ipc-migration` change）
- 不实现 NSPanel/多窗口管理（属于 `tauri-nspanel-windows` change）
- 不实现屏幕捕获（属于 `tauri-screen-capture` change）
- 不实现系统托盘/自动更新（属于 `tauri-system-integration` change）
- 不处理 renderer 层的 Electron API 依赖（属于 `tauri-renderer-migration` change）
- 不做 Electron 应用的任何修改

## Decisions

### 1. 应用目录结构：`apps/stage-tamagotchi-tauri/`

**选择**: 创建独立目录 `apps/stage-tamagotchi-tauri/`，不修改现有 `apps/stage-tamagotchi/`。

**替代方案**:
- 在现有目录添加 `src-tauri/`：会污染 Electron 项目，增加构建复杂度
- 使用完全独立的仓库：失去 monorepo 的共享包优势

**理由**: 并行存在最安全，且 monorepo 内共享包可直接引用。目录结构：

```
apps/stage-tamagotchi-tauri/
├── package.json
├── vite.config.ts
├── index.html
├── src/                    # 最小入口，引用 packages/stage-ui 等
│   ├── main.ts
│   ├── App.vue
│   └── router.ts
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   ├── src/
│   │   └── lib.rs
│   └── icons/
└── uno.config.ts
```

### 2. Vite 配置：独立 vite.config.ts，参考 stage-web

**选择**: 基于 `apps/stage-web/vite.config.ts` 创建新的 Vite 配置，而非复制 Electron 的 `electron.vite.config.ts`。

**替代方案**:
- 复制 electron-vite 配置并修改：electron-vite 有 Electron 特有的 main/preload/renderer 三入口，不适用于 Tauri
- 使用 `@tauri-apps/cli` 默认模板：太简单，缺少项目需要的插件配置

**理由**: `stage-web` 的 Vite 配置已经是纯 Web 构建，与 Tauri 的需求最接近。需要添加的差异：
- `@tauri-apps/api` 的 resolve 配置
- Tauri 的 dev server 端口配置（`tauri.conf.json` 中的 `devUrl`）
- 移除 Electron 特有的 preload/main 入口

### 3. Rust 入口：使用 `lib.rs` + `#[cfg_attr(mobile, tauri::mobile_entry_point)]`

**选择**: 使用 Tauri v2 推荐的 `lib.rs` 模式而非 `main.rs`。

**替代方案**:
- 使用 `main.rs`：不支持未来的移动端扩展

**理由**: `lib.rs` 模式是 Tauri v2 的标准做法，支持桌面和移动端共用入口。

### 4. 插件注册：feature flags 分层

**选择**: 将插件分为 `core`（默认启用）和 `full`（可选）两层。

```toml
[features]
default = ["core"]
core = ["plugin-mcp", "plugin-rdev", "plugin-window-pass-through", "plugin-window-router-link"]
full = ["core", "plugin-audio-transcription", "plugin-audio-vad"]
```

**替代方案**:
- 全部默认启用：ONNX Runtime 编译耗时长，影响开发体验
- 每个插件单独 feature：过于细碎，管理成本高

**理由**: audio-transcription-ort 和 audio-vad-ort 依赖 ONNX Runtime，编译时间显著。开发阶段用 `core` 快速迭代，发布用 `full`。

### 5. 前端入口：最小 shell + 动态导入 stage-ui

**选择**: 前端入口只包含最小的 App.vue + router，通过 workspace 依赖引用 `packages/stage-ui` 的组件和 stores。

**替代方案**:
- 复制 stage-tamagotchi 的 renderer 代码：大量重复，维护困难
- 直接引用 stage-web 的入口：stage-web 有自己的路由和页面结构，不完全匹配

**理由**: 保持入口最小化，业务逻辑全部在 `packages/` 中。初始阶段只需要验证渲染能力，不需要完整的页面路由。

## Risks / Trade-offs

- **[Risk] Vite 插件兼容性**: 部分 Vite 插件可能与 Tauri 的 dev server 模式不兼容 → 逐一测试，必要时替换
- **[Risk] WKWebView CSS 差异**: macOS 上 WKWebView 的 CSS 渲染可能与 Chromium 有细微差异 → 在 `tauri-renderer-migration` change 中处理
- **[Risk] Cargo 编译时间**: 首次编译 Tauri + 所有插件可能需要 5-10 分钟 → feature flags 缓解开发阶段编译时间
- **[Trade-off] 并行维护成本**: 同时维护 Electron 和 Tauri 两个应用增加维护负担 → 迁移完成后移除 Electron 版本
- **[Risk] 根 Cargo.toml workspace 冲突**: 添加新 member 可能影响现有 crates 的编译 → 使用 `resolver = "2"` 已有的配置，增量添加

## Open Questions

- 是否需要为 Tauri 版本使用不同的 app identifier（如 `ai.moeru.airi.tauri`）以便与 Electron 版本共存于同一系统？
- `injeca` DI 框架在 Tauri 的 Rust 侧是否需要对应物，还是仅在前端 JS 侧使用？
- 初始阶段是否需要配置代码签名和 notarization，还是留到后续 change？
