//! Non-macOS fallback for panel windows.
//!
//! Uses standard `set_always_on_top(true)` since NSPanel is macOS-only.
//! Windows will float above normal windows but cannot float above
//! fullscreen applications on Windows/Linux.
//!
//! All panel-specific behavior is handled directly in `WindowManager`
//! methods via `#[cfg(not(target_os = "macos"))]` blocks.
