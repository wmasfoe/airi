## ADDED Requirements

### Requirement: System audio loopback capture

The system SHALL 提供系统音频环回捕获功能，替代 Electron 的 loopback audio 特性：
- macOS：使用 ScreenCaptureKit 的音频捕获（或 CoreAudioTap）
- Linux：使用 PipeWire 的音频捕获
- Windows：使用 WASAPI loopback capture

#### Scenario: Capture system audio on macOS
- **WHEN** 前端调用 `audio_loopback_start()` on macOS
- **THEN** 系统音频开始被捕获，音频数据通过 Tauri event 推送（PCM 格式）

#### Scenario: Stop audio capture
- **WHEN** 前端调用 `audio_loopback_stop()`
- **THEN** 音频捕获停止，释放音频设备资源

#### Scenario: Audio capture with mute option
- **WHEN** 前端调用 `audio_loopback_start({ muteSystemAudio: true })`
- **THEN** 系统音频被捕获但不从扬声器输出（静音环回）
