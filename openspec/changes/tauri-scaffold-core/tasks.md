## 1. 项目目录和基础配置

- [ ] 1.1 创建 `apps/stage-tamagotchi-tauri/` 目录结构（`src/`、`src-tauri/`、`src-tauri/src/`、`src-tauri/capabilities/`、`src-tauri/icons/`）
- [ ] 1.2 创建 `apps/stage-tamagotchi-tauri/package.json`，包名 `@proj-airi/stage-tamagotchi-tauri`，添加 `dev`、`build`、`build:mac`、`build:win`、`build:linux` 脚本
- [ ] 1.3 创建 `apps/stage-tamagotchi-tauri/src-tauri/Cargo.toml`，配置 Tauri 2.x 依赖和 workspace 插件引用，设置 `core`/`full` feature flags
- [ ] 1.4 创建 `apps/stage-tamagotchi-tauri/src-tauri/tauri.conf.json`，配置 identifier `ai.moeru.airi`、productName `AIRI`、默认窗口（透明、无边框、450x600）、CSP、bundle 配置
- [ ] 1.5 创建 `apps/stage-tamagotchi-tauri/src-tauri/capabilities/default.json`，授权 window、shell、fs、http 权限

## 2. Rust 入口和插件注册

- [ ] 2.1 创建 `apps/stage-tamagotchi-tauri/src-tauri/src/lib.rs`，使用 `tauri::Builder::default()` 初始化应用，配置日志
- [ ] 2.2 注册 `tauri-plugin-shell` 到 Builder
- [ ] 2.3 注册 `tauri-plugin-mcp` 到 Builder（条件编译 `#[cfg(feature = "plugin-mcp")]`）
- [ ] 2.4 注册 `tauri-plugin-rdev` 到 Builder（条件编译）
- [ ] 2.5 注册 `tauri-plugin-window-pass-through-on-hover` 和 `tauri-plugin-window-router-link` 到 Builder（条件编译）
- [ ] 2.6 注册 `tauri-plugin-ipc-audio-transcription-ort` 和 `tauri-plugin-ipc-audio-vad-ort` 到 Builder（`full` feature flag 条件编译）
- [ ] 2.7 实现 `health_check` Tauri command，返回应用版本
- [ ] 2.8 创建 `apps/stage-tamagotchi-tauri/src-tauri/build.rs`，调用 `tauri_build::build()`

## 3. Vite 构建管线

- [ ] 3.1 创建 `apps/stage-tamagotchi-tauri/vite.config.ts`，参考 `apps/stage-web/vite.config.ts`，配置 Vue、UnoCSS、unplugin-vue-router、vite-plugin-vue-layouts
- [ ] 3.2 创建 `apps/stage-tamagotchi-tauri/uno.config.ts`，引用根 UnoCSS 配置
- [ ] 3.3 创建 `apps/stage-tamagotchi-tauri/index.html`，包含 `<div id="app">` 和 `<script type="module" src="/src/main.ts">`
- [ ] 3.4 创建 `apps/stage-tamagotchi-tauri/tsconfig.json`，配置 TypeScript 路径别名和 Vue 支持

## 4. 前端最小入口

- [ ] 4.1 创建 `apps/stage-tamagotchi-tauri/src/main.ts`，初始化 Vue app、Pinia、Vue Router、i18n
- [ ] 4.2 创建 `apps/stage-tamagotchi-tauri/src/App.vue`，最小 shell 组件，包含 `<router-view>`
- [ ] 4.3 创建 `apps/stage-tamagotchi-tauri/src/pages/index.vue`，简单的 hello world 页面，验证 stage-ui 组件可渲染
- [ ] 4.4 验证 `@proj-airi/stage-ui`、`@proj-airi/i18n`、`@proj-airi/ui` 等 workspace 包可正确导入

## 5. Workspace 集成

- [ ] 5.1 修改根 `Cargo.toml`，将 `apps/stage-tamagotchi-tauri/src-tauri` 添加到 workspace members
- [ ] 5.2 确认 pnpm workspace 配置已包含 `apps/stage-tamagotchi-tauri`（`apps/*` glob 应自动匹配）
- [ ] 5.3 修改 `turbo.json`，添加 `@proj-airi/stage-tamagotchi-tauri#build` 任务及其依赖
- [ ] 5.4 复制应用图标到 `apps/stage-tamagotchi-tauri/src-tauri/icons/`（从现有 Electron 应用的 `build/` 目录）

## 6. 验证

- [ ] 6.1 运行 `pnpm install` 验证 workspace 依赖解析正常
- [ ] 6.2 运行 `cargo build --workspace` 验证 Cargo workspace 编译通过
- [ ] 6.3 运行 `pnpm -F @proj-airi/stage-tamagotchi-tauri dev` 验证 Tauri dev 模式启动成功
- [ ] 6.4 验证 Vue 前端在 Tauri webview 中正常渲染（透明窗口、无边框、UnoCSS 样式生效）
- [ ] 6.5 验证 `health_check` command 可从前端调用并返回正确版本
