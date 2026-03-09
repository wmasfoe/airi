/**
 * Eventa context for the Tauri renderer.
 *
 * Creates a singleton eventa context backed by the Tauri IPC adapter,
 * with event subscriptions for continuous streams (mouse tracking, window bounds, etc.).
 */
import { createContext } from '@proj-airi/tauri-eventa/adapter'
import { bounds, cursorScreenPoint } from '@proj-airi/tauri-eventa/contracts'

let _instance: ReturnType<typeof createContext> | undefined

/**
 * Get or create the singleton Tauri eventa context.
 * Subscribes to Tauri events and bridges them into the eventa context.
 */
export function useTauriEventa() {
  if (!_instance) {
    _instance = createContext({
      eventSubscriptions: {
        // Continuous event streams from Rust → frontend
        cursor_screen_point: cursorScreenPoint.type ?? 'eventa:event:electron:screen:cursor-screen-point',
        window_bounds: bounds.type ?? 'eventa:event:electron:window:bounds',
        // Widget events from Rust → frontend
        widgets_render: 'eventa:event:electron:windows:widgets:render',
        widgets_remove_event: 'eventa:event:electron:windows:widgets:remove',
        widgets_clear_event: 'eventa:event:electron:windows:widgets:clear',
        widgets_update_event: 'eventa:event:electron:windows:widgets:update',
        // Caption overlay
        caption_is_following_window_changed: 'eventa:event:electron:windows:caption-overlay:is-following-window-changed',
      },
    })
  }
  return _instance
}
