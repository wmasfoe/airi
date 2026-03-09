import { describe, expect, it } from 'vitest'

import { channelToCommand, channelToEvent } from './mapping'

describe('channelToCommand', () => {
  it('maps invoke channel to snake_case command', () => {
    expect(channelToCommand('eventa:invoke:electron:window:close')).toBe('window_close')
  })

  it('maps hyphenated segments', () => {
    expect(channelToCommand('eventa:invoke:electron:mcp:call-tool')).toBe('mcp_call_tool')
  })

  it('maps deeply nested channels', () => {
    expect(channelToCommand('eventa:invoke:electron:windows:settings:open')).toBe('windows_settings_open')
  })

  it('maps event channels as fallback', () => {
    expect(channelToCommand('eventa:event:electron:auto-updater:state-changed')).toBe('auto_updater_state_changed')
  })

  it('handles non-standard channels gracefully', () => {
    expect(channelToCommand('custom:channel:name')).toBe('custom_channel_name')
  })
})

describe('channelToEvent', () => {
  it('maps event channel to snake_case event name', () => {
    expect(channelToEvent('eventa:event:electron:window:bounds')).toBe('window_bounds')
  })

  it('maps hyphenated event channels', () => {
    expect(channelToEvent('eventa:event:electron:screen:cursor-screen-point')).toBe('screen_cursor_screen_point')
  })

  it('maps invoke channels as fallback', () => {
    expect(channelToEvent('eventa:invoke:electron:window:close')).toBe('window_close')
  })
})
