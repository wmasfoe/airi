## 1. 平台抽象层

- [ ] 1.1 创建 `packages/stage-shared/src/platform.ts`，导出 `isTauri`、`isElectron` 检测函数
- [ ] 1.2 审计 `packages/stage-ui/src/composables/` 中所有依赖 Electron API 的 composables，列出需要适配的清单
- [ ] 1.3 审计 `packages/stage-ui/src/stores/` 中所有依赖 eventa/Electron 的 stores
- [ ] 1.4 为每个平台相关 composable 创建 adapter 接口（如 `WindowAdapter`、`ScreenAdapter`）
- [ ] 1.5 实现 Tauri adapter（调用 `@tauri-apps/api`）
- [ ] 1.6 实现 Electron adapter（包装现有 eventa 调用）
- [ ] 1.7 在 composables 中集成平台选择逻辑

## 2. Electron API 依赖移除

- [ ] 2.1 移除 renderer 层对 `contextBridge` / `preload` 的直接依赖
- [ ] 2.2 将 `window.electron.*` 调用替换为平台抽象层调用
- [ ] 2.3 处理 `@proj-airi/electron-eventa` 在 Tauri 环境下的替代导入
- [ ] 2.4 处理 `@proj-airi/electron-vueuse` 在 Tauri 环境下的替代

## 3. 资源和 CSP 配置

- [ ] 3.1 配置 Tauri 的 asset protocol scope（允许访问 public 目录和 userData）
- [ ] 3.2 配置 CSP：允许 `asset:`、外部 API、WebSocket、`blob:`、`data:`
- [ ] 3.3 创建 `convertFileSrc` 工具函数的平台抽象（Tauri 用 `@tauri-apps/api`，Electron 用 `file://`）
- [ ] 3.4 验证字体文件加载
- [ ] 3.5 验证 3D 模型文件加载

## 4. WebView 兼容性验证

- [ ] 4.1 测试 Three.js WebGL2 渲染在 macOS WKWebView 中的表现
- [ ] 4.2 测试 Three.js WebGPU 渲染在 macOS 26+ WKWebView 中的表现
- [ ] 4.3 测试 MediaPipe 在 WKWebView 中的兼容性
- [ ] 4.4 测试 Three.js 在 Windows WebView2 中的表现
- [ ] 4.5 测试 Three.js 在 Linux WebKitGTK 中的表现
- [ ] 4.6 记录兼容性问题并实现 fallback

## 5. CSS 兼容性

- [ ] 5.1 检查 UnoCSS 生成的 CSS 在 WKWebView 中的渲染
- [ ] 5.2 添加必要的 `-webkit-` 前缀（通过 UnoCSS/PostCSS 配置）
- [ ] 5.3 处理 scrollbar 样式差异
- [ ] 5.4 处理 backdrop-filter 兼容性

## 6. 验证

- [ ] 6.1 在 macOS Tauri 应用中验证完整 UI 渲染
- [ ] 6.2 在 Windows Tauri 应用中验证完整 UI 渲染
- [ ] 6.3 验证平台抽象层在两个平台上都能正确选择 adapter
- [ ] 6.4 运行 `pnpm typecheck` 确保无类型错误
- [ ] 6.5 运行 `pnpm lint:fix` 确保代码风格一致
