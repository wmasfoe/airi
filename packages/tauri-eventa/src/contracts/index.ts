/**
 * Tauri-compatible eventa contract definitions.
 *
 * These mirror the contracts in `apps/stage-tamagotchi/src/shared/eventa.ts`
 * and `packages/electron-eventa/` but without Electron type dependencies.
 * The channel names are identical so the Tauri adapter maps them correctly.
 */

import { defineEventa, defineInvokeEventa } from '@moeru/eventa'

// ── Window contracts ──────────────────────────────────────────

export interface Rectangle {
  x: number
  y: number
  width: number
  height: number
}

export interface Point {
  x: number
  y: number
}

export type ResizeDirection = 'n' | 's' | 'e' | 'w' | 'ne' | 'nw' | 'se' | 'sw'
export type VibrancyType = string | null
export type BackgroundMaterialType = string | null

export const bounds = defineEventa<Rectangle>('eventa:event:electron:window:bounds')
export const startLoopGetBounds = defineInvokeEventa('eventa:event:electron:window:start-loop-get-bounds')
export const cursorScreenPoint = defineEventa<Point>('eventa:event:electron:screen:cursor-screen-point')
export const startLoopGetCursorScreenPoint = defineInvokeEventa('eventa:event:electron:screen:start-loop-get-cursor-screen-point')

export const window = {
  getBounds: defineInvokeEventa<Rectangle>('eventa:invoke:electron:window:get-bounds'),
  setBounds: defineInvokeEventa<void, [Partial<Rectangle>]>('eventa:invoke:electron:window:set-bounds'),
  setIgnoreMouseEvents: defineInvokeEventa<void, [boolean, { forward: boolean }]>('eventa:invoke:electron:window:set-ignore-mouse-events'),
  setVibrancy: defineInvokeEventa<void, [VibrancyType]>('eventa:invoke:electron:window:set-vibrancy'),
  setBackgroundMaterial: defineInvokeEventa<void, [BackgroundMaterialType]>('eventa:invoke:electron:window:set-background-material'),
  resize: defineInvokeEventa<void, { deltaX: number, deltaY: number, direction: ResizeDirection }>('eventa:invoke:electron:window:resize'),
  close: defineInvokeEventa<void>('eventa:invoke:electron:window:close'),
}

// ── Screen contracts ──────────────────────────────────────────

export interface Display {
  id: number
  bounds: Rectangle
  workArea: Rectangle
  scaleFactor: number
}

export const screen = {
  getAllDisplays: defineInvokeEventa<Display[]>('eventa:invoke:electron:screen:get-all-displays'),
  getPrimaryDisplay: defineInvokeEventa<Display>('eventa:invoke:electron:screen:get-primary-display'),
  getCursorScreenPoint: defineInvokeEventa<Point>('eventa:invoke:electron:screen:get-cursor-screen-point'),
  dipToScreenPoint: defineInvokeEventa<Point, Point>('eventa:invoke:electron:screen:dip-to-screen-point'),
  dipToScreenRect: defineInvokeEventa<Rectangle, Rectangle>('eventa:invoke:electron:screen:dip-to-screen-rect'),
  screenToDipPoint: defineInvokeEventa<Point, Point>('eventa:invoke:electron:screen:screen-to-dip-point'),
  screenToDipRect: defineInvokeEventa<Rectangle, Rectangle>('eventa:invoke:electron:screen:screen-to-dip-rect'),
}

// ── App contracts ─────────────────────────────────────────────

export const app = {
  isMacOS: defineInvokeEventa<boolean>('eventa:invoke:electron:app:is-macos'),
  isWindows: defineInvokeEventa<boolean>('eventa:invoke:electron:app:is-windows'),
  isLinux: defineInvokeEventa<boolean>('eventa:invoke:electron:app:is-linux'),
  quit: defineInvokeEventa<void>('eventa:invoke:electron:app:quit'),
}

/**
 * Combined electron-compatible namespace for use with the Tauri adapter.
 * Provides the same API surface as `@proj-airi/electron-eventa`'s `electron` export.
 */
export const electron = {
  screen,
  window,
  app,
  // systemPreferences is Electron-specific and not needed in Tauri
}
