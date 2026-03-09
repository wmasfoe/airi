## 1. tauri-nspanel 集成

- [ ] 1.1 添加 `tauri-nspanel` v2.1 git 依赖到 `apps/stage-tamagotchi-tauri/src-tauri/Cargo.toml`
- [ ] 1.2 在 `lib.rs` 中注册 `tauri_nspanel::init()` 插件
- [ ] 1.3 定义 `AiriMainPanel` panel class（`can_become_key_window: true`、`is_floating_panel: true`）
- [ ] 1.4 定义 `AiriCaptionPanel` 和 `AiriWidgetsPanel` panel classes
- [ ] 1.5 验证 Panel 窗口在 macOS 上可浮在全屏应用上方

## 2. WindowManager 核心

- [ ] 2.1 创建 `src-tauri/src/windows/mod.rs`，定义 `WindowManager` struct
- [ ] 2.2 实现 `WindowManager::new()` 初始化
- [ ] 2.3 实现 `create_main_window()`：使用 PanelBuilder 创建 NSPanel，透明、无边框、450x600、PanelLevel::ScreenSaver
- [ ] 2.4 实现 `create_settings_window()`：标准窗口，800x600，单例模式
- [ ] 2.5 实现 `create_chat_window()`：标准窗口，400x600，单例模式
- [ ] 2.6 实现 `create_devtools_window(route)`：标准窗口，1200x800，支持多开
- [ ] 2.7 实现 `create_about_window()`：标准窗口，400x300，单例模式
- [ ] 2.8 实现 `create_notice_window()`：标准窗口，400x200
- [ ] 2.9 实现 `create_caption_window()`：NSPanel，透明，floating level 2
- [ ] 2.10 实现 `create_widgets_window()`：NSPanel，透明，floating level 1，点击穿透
- [ ] 2.11 实现 `create_dashboard_window()` 和 `create_inlay_window()`
- [ ] 2.12 实现 `create_beat_sync_window()`

## 3. 窗口效果

- [ ] 3.1 实现透明窗口配置函数（Tauri 的 `transparent: true`、`decorations: false`）
- [ ] 3.2 实现 macOS vibrancy 效果（通过 `tauri-nspanel` 或 `objc2` 设置 `NSVisualEffectView`）
- [ ] 3.3 实现 Windows acrylic 效果（Tauri 的 window effects API）
- [ ] 3.4 集成 `tauri-plugin-window-pass-through-on-hover` 到 widgets 窗口
- [ ] 3.5 实现窗口拖拽（CSS `data-tauri-drag-region` 或 `window.start_dragging()` command）
- [ ] 3.6 实现 macOS 隐藏红绿灯按钮（通过 `tauri-nspanel` 或 `objc2`）

## 4. 窗口持久化

- [ ] 4.1 添加 `tauri-plugin-window-state` 依赖并注册
- [ ] 4.2 为标准窗口（settings、chat、devtools）启用自动 bounds 持久化
- [ ] 4.3 为 Panel 窗口实现自定义 JSON 持久化（读写 `{userData}/window-state.json`）
- [ ] 4.4 实现多显示器感知的位置恢复（检查保存位置是否在当前显示器范围内）

## 5. 跨平台 Fallback

- [ ] 5.1 实现 `#[cfg(not(target_os = "macos"))]` fallback：使用标准 `set_always_on_top(true)` 替代 NSPanel
- [ ] 5.2 确保 Windows/Linux 上所有窗口功能正常（无 NSPanel 依赖的代码路径）
- [ ] 5.3 测试 Linux Wayland 和 X11 上的 always-on-top 行为

## 6. 验证

- [ ] 6.1 验证 macOS 上主窗口浮在全屏应用上方
- [ ] 6.2 验证 macOS 上窗口在所有 Spaces 可见
- [ ] 6.3 验证 Panel 层级顺序（main > caption > widgets）
- [ ] 6.4 验证 widgets 窗口点击穿透正常工作
- [ ] 6.5 验证窗口位置在重启后正确恢复
- [ ] 6.6 验证 Windows 上 always-on-top fallback 正常工作
