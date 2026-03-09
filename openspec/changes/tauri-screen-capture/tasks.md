## 1. 插件骨架

- [ ] 1.1 创建 `crates/tauri-plugin-screen-capture/`，配置 `Cargo.toml`（依赖 `tauri`、`scap`、`serde`、`specta`）
- [ ] 1.2 创建 `build.rs` 和 `permissions/` 目录
- [ ] 1.3 实现插件初始化 `init()` 函数
- [ ] 1.4 在 `apps/stage-tamagotchi-tauri/src-tauri/Cargo.toml` 中添加插件依赖

## 2. 权限管理

- [ ] 2.1 实现 `screen_capture_check_permission` command（macOS TCC 检查）
- [ ] 2.2 实现 `screen_capture_request_permission` command（macOS 权限请求）
- [ ] 2.3 实现 Windows/Linux 权限检查 fallback
- [ ] 2.4 实现 `screen_capture_open_system_settings` command（打开系统权限设置页）

## 3. 源枚举

- [ ] 3.1 实现 `screen_capture_get_sources` command，使用 `scap` 枚举屏幕和窗口
- [ ] 3.2 实现源信息序列化（ID、名称、类型、分辨率）
- [ ] 3.3 实现缩略图生成（可选，降采样 + PNG 编码 + base64）

## 4. 帧捕获

- [ ] 4.1 实现 `screen_capture_frame` command（单帧捕获）
- [ ] 4.2 实现 `screen_capture_start` command（连续捕获，tokio task + Tauri event 推送）
- [ ] 4.3 实现 `screen_capture_stop` command（停止捕获，取消 tokio task）
- [ ] 4.4 实现帧格式转换（raw RGBA、PNG、JPEG）
- [ ] 4.5 实现帧降采样（可选分辨率缩放）

## 5. 音频环回

- [ ] 5.1 实现 macOS 音频环回捕获（ScreenCaptureKit audio stream）
- [ ] 5.2 实现 Windows 音频环回捕获（WASAPI loopback 或 `cpal`）
- [ ] 5.3 实现 Linux 音频环回捕获（PipeWire）
- [ ] 5.4 实现 `audio_loopback_start` / `audio_loopback_stop` commands
- [ ] 5.5 实现音频数据通过 Tauri event 推送（PCM 格式）

## 6. 前端 JS Bindings

- [ ] 6.1 创建 `packages/tauri-plugin-screen-capture/` TypeScript 包
- [ ] 6.2 实现 `checkPermission()`、`requestPermission()` 函数
- [ ] 6.3 实现 `getSources(options)` 函数
- [ ] 6.4 实现 `captureFrame(sourceId)` 和 `startCapture(options)` / `stopCapture()` 函数
- [ ] 6.5 实现 `startAudioLoopback()` / `stopAudioLoopback()` 函数

## 7. 验证

- [ ] 7.1 验证 macOS 上屏幕源枚举返回正确的显示器列表
- [ ] 7.2 验证单帧捕获返回有效图像数据
- [ ] 7.3 验证连续捕获的帧率和延迟
- [ ] 7.4 验证权限检查和请求流程
- [ ] 7.5 验证音频环回捕获输出有效 PCM 数据
