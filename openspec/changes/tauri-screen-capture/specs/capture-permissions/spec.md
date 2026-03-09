## ADDED Requirements

### Requirement: Screen capture permission management

The system SHALL 提供跨平台的屏幕录制权限检查和请求功能：
- macOS：检查和请求 TCC（Transparency, Consent, and Control）屏幕录制权限
- Windows：检查 Windows Graphics Capture 权限
- Linux：检查 PipeWire portal 权限

#### Scenario: Check permission on macOS
- **WHEN** 前端调用 `screen_capture_check_permission()`
- **THEN** 返回 `"granted"` | `"denied"` | `"not-determined"` 状态

#### Scenario: Request permission on macOS
- **WHEN** 前端调用 `screen_capture_request_permission()` 且当前状态为 `"not-determined"`
- **THEN** 系统弹出权限请求对话框，用户操作后返回新状态

#### Scenario: Permission denied handling
- **WHEN** 前端调用 `screen_capture_get_sources()` 但权限为 `"denied"`
- **THEN** 返回错误信息指示需要在系统设置中手动授权，并提供打开系统设置的 command
