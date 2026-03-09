## ADDED Requirements

### Requirement: Asset path conversion for Tauri

The system SHALL 处理 Tauri 的资源协议，使静态资源（图片、字体、3D 模型、音频）能正确加载：
- 使用 `@tauri-apps/api/core.convertFileSrc()` 将文件系统路径转为 webview 可访问的 URL
- 配置 Tauri 的 `asset` 协议 scope 允许访问必要的资源目录

#### Scenario: Font files load via asset protocol
- **WHEN** CSS `@font-face` 声明引用本地字体文件
- **THEN** 字体通过 Tauri asset 协议正确加载并渲染

#### Scenario: 3D model files load
- **WHEN** Three.js loader 请求 GLTF/GLB 模型文件
- **THEN** 模型通过 asset 协议加载，无 CORS 错误

#### Scenario: Dynamic file path conversion
- **WHEN** 前端需要显示用户数据目录中的图片
- **THEN** 使用 `convertFileSrc(path)` 将绝对路径转为 `asset://localhost/...` URL

### Requirement: Content Security Policy configuration

The system SHALL 配置 Tauri 的 CSP 允许：
- 加载本地资源（`asset:` 协议）
- 连接外部 API（LLM providers、TTS services 等）
- WebSocket 连接（本地 server channel）
- `unsafe-eval`（如果 Vue 模板编译需要）
- `blob:` 和 `data:` URI（用于动态生成的内容）

#### Scenario: External API calls succeed
- **WHEN** 前端向 LLM provider API 发送请求
- **THEN** 请求不被 CSP 阻止，正常返回响应

#### Scenario: WebSocket connection to local server
- **WHEN** 前端连接本地 WebSocket server（`wss://localhost:6121`）
- **THEN** 连接不被 CSP 阻止，正常建立
