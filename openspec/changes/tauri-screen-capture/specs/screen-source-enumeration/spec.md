## ADDED Requirements

### Requirement: Cross-platform screen and window source enumeration

The system SHALL 提供跨平台的屏幕和窗口源枚举功能，替代 Electron 的 `desktopCapturer.getSources()`。使用 `scap` crate 实现：
- macOS：ScreenCaptureKit
- Windows：Windows.Graphics.Capture
- Linux：PipeWire

返回的源信息 MUST 包含：源 ID、名称、类型（screen/window）、缩略图（可选）。

#### Scenario: List all screens
- **WHEN** 前端调用 `screen_capture_get_sources({ types: ["screen"] })`
- **THEN** 返回所有可用显示器的源列表，每个包含 ID、名称、分辨率信息

#### Scenario: List all windows
- **WHEN** 前端调用 `screen_capture_get_sources({ types: ["window"] })`
- **THEN** 返回所有可见窗口的源列表，每个包含 ID、窗口标题、所属应用名

#### Scenario: Source with thumbnail
- **WHEN** 前端请求源列表并指定 `thumbnailSize: { width: 320, height: 240 }`
- **THEN** 每个源包含指定尺寸的缩略图（base64 编码的 PNG）
