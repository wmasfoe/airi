## Context

AIRI 桌面端有 12 个窗口，其中 main、caption、widgets、inlay 使用 Electron 的 `type: 'panel'`（macOS NSPanel）实现浮动覆盖行为。这是 AIRI 作为桌面宠物的核心体验——角色窗口始终可见。

现有窗口配置分散在 `apps/stage-tamagotchi/src/main/windows/` 下各子目录中，每个窗口有独立的创建逻辑。共享工具函数在 `windows/shared/` 中（`transparentWindowConfig`、`blurryWindowConfig`、`resizeWindowByDelta` 等）。

`tauri-nspanel` v2.1 提供了 `PanelBuilder` API，已被 BongoCat（桌面宠物）等同类项目验证。项目自有的 `tauri-plugin-window-pass-through-on-hover` 提供点击穿透能力。

## Goals / Non-Goals

**Goals:**
- 使用 `tauri-nspanel` 实现 macOS Panel 窗口（main、caption、widgets、inlay）
- 实现 12 个窗口的创建、配置、生命周期管理
- 实现透明、vibrancy、点击穿透、拖拽等窗口效果
- 实现窗口位置/尺寸持久化
- Windows/Linux 上提供合理的 fallback

**Non-Goals:**
- 不实现窗口间的 IPC 通信（属于 `tauri-ipc-migration`）
- 不实现托盘菜单中的窗口控制（属于 `tauri-system-integration`）
- 不处理 renderer 层的窗口相关 composables（属于 `tauri-renderer-migration`）

## Decisions

### 1. NSPanel 集成方式：tauri-nspanel PanelBuilder

**选择**: 使用 `tauri-nspanel` v2.1 的 `PanelBuilder` API 创建 Panel 窗口。

```rust
tauri_panel! {
    panel!(AiriMainPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })
}
```

**替代方案**:
- 直接使用 `objc2` 手动 swizzle NSWindow → NSPanel：复杂且容易出错
- 使用 `tauri-plugin-spotlight`：功能不如 `tauri-nspanel` 完整

**理由**: `tauri-nspanel` 是社区标准方案，API 稳定，有大量生产项目验证。

### 2. 窗口管理架构：集中式 WindowManager

**选择**: 创建一个 `WindowManager` struct 持有所有窗口的引用，提供统一的创建/获取/销毁接口。

```rust
pub struct WindowManager {
    main: Option<WebviewPanel>,      // NSPanel
    caption: Option<WebviewPanel>,   // NSPanel
    widgets: Option<WebviewPanel>,   // NSPanel
    settings: Option<WebviewWindow>, // 标准窗口
    chat: Option<WebviewWindow>,
    // ...
}
```

**替代方案**:
- 每个窗口独立管理（类似现有 Electron 架构）：分散，难以协调
- 使用 Tauri 的 `app.get_webview_window()` 按 label 查找：可行但缺少类型安全

**理由**: 集中管理便于窗口间协调（如 caption 跟随 main），也便于生命周期管理。

### 3. 窗口持久化：tauri-plugin-window-state + 自定义扩展

**选择**: 使用 `tauri-plugin-window-state` 处理标准窗口的 bounds 持久化，Panel 窗口使用自定义持久化（因为 Panel 不是标准 WebviewWindow）。

**替代方案**:
- 全部自定义：重复造轮子
- 全部用 plugin：Panel 窗口可能不兼容

**理由**: 混合方案最务实。标准窗口用 plugin 省事，Panel 窗口用自定义 JSON 配置。

### 4. 跨平台策略：条件编译 + trait 抽象

**选择**: 定义 `WindowBackend` trait，macOS 实现使用 NSPanel，Windows/Linux 实现使用标准 always-on-top 窗口。

```rust
#[cfg(target_os = "macos")]
mod macos { /* NSPanel implementation */ }

#[cfg(not(target_os = "macos"))]
mod fallback { /* Standard always-on-top */ }
```

**理由**: 保持平台特定代码隔离，主逻辑不受平台差异影响。

### 5. 窗口 URL 路由：统一前端 + path 路由

**选择**: 所有窗口加载同一个前端构建产物，通过 URL path 区分页面（如 `/settings`、`/devtools/memory`）。Vue Router 根据 path 渲染对应组件。

**替代方案**:
- 每个窗口独立 HTML 入口：构建复杂，资源重复
- 使用 hash 路由：不如 path 路由清晰

**理由**: 与现有 Electron 架构一致，Vue Router 已支持 path 路由。

## Risks / Trade-offs

- **[Risk] tauri-nspanel 版本兼容**: v2.1 是 git 依赖，非 crates.io 发布 → 锁定 commit hash
- **[Risk] Panel 窗口的 webview 行为差异**: NSPanel 的 webview 可能在某些场景下行为不同（如键盘焦点） → 参考 BongoCat 的实现验证
- **[Trade-off] Windows/Linux 无全屏覆盖**: 这些平台上 AIRI 窗口无法浮在全屏应用上方 → 可接受，macOS 是主要目标平台
- **[Risk] 多窗口内存占用**: 12 个 webview 窗口的内存占用 → 延迟创建非核心窗口，按需加载
- **[Risk] 点击穿透 + 事件转发的平台差异**: macOS 和 Windows 的实现机制不同 → 复用现有 `tauri-plugin-window-pass-through-on-hover` 已处理

## Open Questions

- `tauri-nspanel` 的 `PanelLevel` 是否支持自定义数值层级（如 Electron 的 level 1/2 区分），还是只有预设层级？
- Panel 窗口是否支持 `tauri-plugin-window-state` 的自动持久化，还是必须自定义？
- caption 窗口跟随主窗口移动的实现：是通过 Rust 侧监听主窗口 move 事件同步，还是前端侧通过 IPC 协调？
