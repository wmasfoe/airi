## ADDED Requirements

### Requirement: Three.js rendering compatibility

The system SHALL 确保 Three.js 在所有 Tauri 支持的 webview 中正常渲染：
- macOS WKWebView：WebGL2 + WebGPU（Safari 26+）
- Windows WebView2：WebGL2 + WebGPU
- Linux WebKitGTK：WebGL2（WebGPU 支持有限）

#### Scenario: Three.js scene renders on macOS
- **WHEN** 包含 Three.js 场景的页面在 macOS Tauri 应用中加载
- **THEN** 3D 场景正常渲染，无 WebGL/WebGPU 错误

#### Scenario: WebGPU fallback to WebGL
- **WHEN** Three.js 请求 WebGPU 但 webview 不支持
- **THEN** 自动 fallback 到 WebGL2 渲染器，功能正常但可能性能降低

### Requirement: MediaPipe WebGPU compatibility

The system SHALL 验证 MediaPipe（用于手势/面部追踪）在 Tauri webview 中的兼容性。如果 MediaPipe 在 WKWebView 中不兼容，MUST 提供替代方案或禁用相关功能。

#### Scenario: MediaPipe works on macOS Safari 26+
- **WHEN** MediaPipe 在 macOS 26+ 的 WKWebView 中初始化
- **THEN** 面部/手势追踪功能正常工作

#### Scenario: MediaPipe fallback on incompatible webview
- **WHEN** MediaPipe 在不支持 WebGPU 的 webview 中初始化
- **THEN** 系统检测到不兼容，禁用 MediaPipe 功能并显示提示信息

### Requirement: CSS and layout consistency

The system SHALL 处理不同 webview 引擎的 CSS 渲染差异：
- Safari/WKWebView 的 CSS 前缀（`-webkit-`）
- scrollbar 样式差异
- backdrop-filter 支持差异

#### Scenario: UnoCSS styles render consistently
- **WHEN** 使用 UnoCSS utility classes 的组件在 WKWebView 中渲染
- **THEN** 样式与 Chromium（Electron）中的渲染视觉一致（允许细微差异）
