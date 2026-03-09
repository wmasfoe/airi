## Why

AIRI 使用 Electron 的 `desktopCapturer` API 和 `session.setDisplayMediaRequestHandler` 实现屏幕捕获，包括屏幕/窗口枚举、视频流获取、音频环回（macOS CoreAudioTap/ScreenCaptureKit、Linux PulseAudio）。这是 Electron 独有的 API，Tauri 没有内置等价物。

现在 Rust 生态有了成熟的跨平台屏幕捕获方案：`scap` crate 封装了 macOS ScreenCaptureKit、Windows Graphics Capture、Linux PipeWire。`screencapturekit-rs` 提供了 macOS 专用的更底层绑定。这些 crate 使得在 Tauri 中实现屏幕捕获成为可能，但需要自定义插件将捕获的帧桥接到 webview。

## What Changes

- 创建 `tauri-plugin-screen-capture` Tauri 插件，封装 `scap` crate
- 实现屏幕/窗口源枚举（替代 `desktopCapturer.getSources()`）
- 实现屏幕捕获帧获取，通过 IPC 传递给 renderer
- 实现 macOS 屏幕录制权限检查和请求（替代 `systemPreferences.getMediaAccessStatus('screen')`）
- 实现音频环回捕获（macOS ScreenCaptureKit audio、Linux PipeWire audio）
- 实现 Mutex 基础的源管理和超时处理（复用现有 `packages/electron-screen-capture` 的逻辑）
- 提供 renderer 侧的 JS API，尽量兼容现有 `getDisplayMedia` 使用模式

## Capabilities

### New Capabilities

- `screen-source-enumeration`: 跨平台屏幕/窗口源枚举，源信息序列化，缩略图生成
- `screen-frame-capture`: 屏幕帧捕获和传输，帧格式转换，性能优化（共享内存/零拷贝）
- `audio-loopback-capture`: 系统音频环回捕获，macOS CoreAudioTap/ScreenCaptureKit、Linux PipeWire 适配
- `capture-permissions`: 跨平台屏幕录制权限检查和请求（macOS TCC、Windows/Linux 权限模型）

### Modified Capabilities

（无现有 specs 需要修改）

## Impact

- 新增 `crates/tauri-plugin-screen-capture/`（或扩展现有 `packages/electron-screen-capture`）
- 依赖：`scap`、`screencapturekit`（macOS）、`windows-capture`（Windows）
- 修改 renderer 侧屏幕捕获相关 composables
- 平台特定代码量较大（权限、音频环回）
- 现有 `packages/electron-screen-capture` 保留给 Electron 版本使用
