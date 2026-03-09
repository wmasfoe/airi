/**
 * Maps eventa channel names to Tauri command/event names.
 *
 * Eventa channels follow the pattern:
 *   eventa:invoke:electron:<domain>:<action>  → Tauri command (snake_case)
 *   eventa:event:electron:<domain>:<action>   → Tauri event name (snake_case)
 *
 * Examples:
 *   eventa:invoke:electron:window:close        → window_close
 *   eventa:invoke:electron:mcp:call-tool       → mcp_call_tool
 *   eventa:event:electron:auto-updater:state-changed → auto_updater_state_changed
 */

const INVOKE_PREFIX = 'eventa:invoke:electron:'
const EVENT_PREFIX = 'eventa:event:electron:'

/**
 * Strip the eventa prefix and convert the remainder to snake_case.
 * Colons and hyphens become underscores.
 */
function toSnakeCase(segment: string): string {
  return segment.replace(/[:-]/g, '_')
}

/**
 * Map an eventa invoke channel name to a Tauri command name.
 *
 * @example channelToCommand('eventa:invoke:electron:window:close') // 'window_close'
 */
export function channelToCommand(channel: string): string {
  if (channel.startsWith(INVOKE_PREFIX)) {
    return toSnakeCase(channel.slice(INVOKE_PREFIX.length))
  }
  // Fallback: strip any known prefix and convert
  if (channel.startsWith(EVENT_PREFIX)) {
    return toSnakeCase(channel.slice(EVENT_PREFIX.length))
  }
  // Non-standard channel — best-effort conversion
  return toSnakeCase(channel)
}

/**
 * Map an eventa event channel name to a Tauri event name.
 *
 * @example channelToEvent('eventa:event:electron:window:bounds') // 'window_bounds'
 */
export function channelToEvent(channel: string): string {
  if (channel.startsWith(EVENT_PREFIX)) {
    return toSnakeCase(channel.slice(EVENT_PREFIX.length))
  }
  if (channel.startsWith(INVOKE_PREFIX)) {
    return toSnakeCase(channel.slice(INVOKE_PREFIX.length))
  }
  return toSnakeCase(channel)
}
