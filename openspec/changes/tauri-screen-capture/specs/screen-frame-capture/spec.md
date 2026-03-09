## ADDED Requirements

### Requirement: Screen frame capture and streaming

The system SHALL 提供屏幕帧捕获功能，将捕获的帧通过 IPC 传递给 webview。支持两种模式：
- 单帧捕获：获取当前屏幕截图
- 连续捕获：以指定帧率持续捕获并通过 Tauri event 推送

#### Scenario: Single frame capture
- **WHEN** 前端调用 `screen_capture_frame({ sourceId: "screen-0" })`
- **THEN** 返回当前屏幕的一帧图像（base64 PNG 或 raw RGBA buffer）

#### Scenario: Start continuous capture
- **WHEN** 前端调用 `screen_capture_start({ sourceId: "screen-0", fps: 15 })`
- **THEN** Rust 侧开始以 15fps 捕获屏幕帧，通过 Tauri event `screen_capture_frame` 推送

#### Scenario: Stop continuous capture
- **WHEN** 前端调用 `screen_capture_stop()`
- **THEN** 捕获停止，不再推送帧事件，释放捕获资源

### Requirement: Frame format and performance

The system SHALL 支持多种帧传输格式以平衡质量和性能：
- `raw`: RGBA buffer（最快，适合本地处理）
- `png`: PNG 编码（适合显示）
- `jpeg`: JPEG 编码（最小体积，适合网络传输）

#### Scenario: High-performance local capture
- **WHEN** 前端请求 `raw` 格式的连续捕获
- **THEN** 帧数据以 RGBA buffer 通过 Tauri binary event 传输，延迟 < 50ms
