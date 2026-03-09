## Context

AIRI 使用 Electron 的 `desktopCapturer` + `session.setDisplayMediaRequestHandler` 实现屏幕捕获。`packages/electron-screen-capture` 封装了完整的捕获逻辑，包括源枚举、Mutex 管理、macOS 权限检查、音频环回（CoreAudioTap/ScreenCaptureKit/PulseAudio）。

Tauri 没有内置屏幕捕获 API，但 Rust 生态有成熟方案：
- `scap`：跨平台（ScreenCaptureKit / Windows.Graphics.Capture / PipeWire）
- `screencapturekit-rs`：macOS 专用底层绑定
- `cpal`：跨平台音频 I/O

## Goals / Non-Goals

**Goals:**
- 创建 Tauri 插件封装跨平台屏幕捕获
- 实现源枚举、帧捕获、权限管理
- 实现音频环回捕获
- 提供前端 JS API

**Non-Goals:**
- 不实现 `getDisplayMedia` Web API 的完整模拟（太复杂）
- 不处理视频编码/录制（只提供原始帧）
- 不实现远程桌面/屏幕共享协议

## Decisions

### 1. 捕获引擎：scap crate

**选择**: 使用 `scap` crate 作为主要捕获引擎。

**替代方案**:
- 直接使用 `screencapturekit-rs` + `windows-capture` + PipeWire bindings：需要维护三套代码
- FFmpeg bindings：过重

**理由**: `scap` 已封装三个平台的原生 API，API 简洁，活跃维护。

### 2. 帧传输：Tauri binary event

**选择**: 使用 Tauri 的 event system 传输帧数据。小帧用 base64 JSON event，大帧/高频用 Tauri channel API。

**替代方案**:
- SharedArrayBuffer：WKWebView 兼容性问题
- WebSocket：额外复杂度
- 写入临时文件 + 通知：延迟高

**理由**: Tauri event 是最自然的 Rust → 前端通道。Channel API 支持 binary payload，适合高频帧传输。

### 3. 音频捕获：平台独立实现

**选择**: 音频环回与视频捕获分开实现，因为 `scap` 主要聚焦视频。
- macOS：`screencapturekit-rs` 的音频流
- Windows：WASAPI loopback（`cpal` crate）
- Linux：PipeWire audio capture

**理由**: 音频环回的平台差异大，统一抽象反而增加复杂度。

## Risks / Trade-offs

- **[Risk] scap crate 稳定性**: 相对年轻的 crate → 锁定版本，准备 fallback 到直接平台 API
- **[Risk] 帧传输性能**: 高分辨率 + 高帧率的 IPC 传输可能成为瓶颈 → 支持降采样和 JPEG 压缩
- **[Trade-off] 无 getDisplayMedia 兼容**: 前端代码需要从 Web API 迁移到自定义 API → 创建兼容层
- **[Risk] Linux PipeWire 权限**: 某些 Linux 发行版的 PipeWire portal 配置不同 → 提供详细错误信息

## Open Questions

- 是否需要支持窗口级别的捕获（只捕获特定窗口），还是只需要全屏捕获？
- 音频环回的静音模式（capture without playback）在 macOS ScreenCaptureKit 上是否可靠？
- 帧数据是否需要在 Rust 侧做预处理（如人脸检测、运动检测），还是全部交给前端？
