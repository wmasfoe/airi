/**
 * Tauri adapter for @moeru/eventa.
 *
 * Creates an EventContext that bridges eventa's invoke/event system
 * to Tauri's `invoke()` and `listen()`/`emit()` APIs.
 *
 * Usage (renderer side):
 * ```ts
 * import { createContext } from '@proj-airi/tauri-eventa/adapter'
 * import { defineInvoke } from '@moeru/eventa'
 * import { electronWindowClose } from './contracts'
 *
 * const { context, dispose } = createContext()
 * const closeWindow = defineInvoke(context, electronWindowClose)
 * await closeWindow()
 * ```
 */
import type { EventCallback, UnlistenFn } from '@tauri-apps/api/event'

import {
  and,
  createContext as createEventaContext,
  defineInboundEventa,
  matchBy,
} from '@moeru/eventa'
import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { emit as tauriEmit, listen as tauriListen } from '@tauri-apps/api/event'

import { channelToCommand } from './mapping'

// eventa internal flow direction enum value for outbound
const OUTBOUND = 1

/**
 * Checks whether an eventa event type string looks like an invoke
 * send event (ends with `-send`).
 */
function isInvokeSendType(type: string): boolean {
  return type.endsWith('-send')
}

/**
 * Derive the receive event type from a send event type.
 * `some-channel-send` → `some-channel-receive`
 */
function toReceiveType(sendType: string): string {
  return `${sendType.slice(0, -'-send'.length)}-receive`
}

/**
 * Derive the receive-error event type from a send event type.
 * `some-channel-send` → `some-channel-receive-error`
 */
function toReceiveErrorType(sendType: string): string {
  return `${sendType.slice(0, -'-send'.length)}-receive-error`
}

/**
 * Extract the original eventa channel tag from a send event type.
 * The tag is everything before `-send`.
 */
function extractTag(sendType: string): string {
  return sendType.slice(0, -'-send'.length)
}

export interface CreateContextOptions {
  /**
   * Optional list of Tauri event names to subscribe to automatically.
   * Each entry maps a Tauri event name to the eventa channel tag
   * that should receive the payload.
   *
   * @example
   * ```ts
   * createContext({
   *   eventSubscriptions: {
   *     'window_bounds': 'eventa:event:electron:window:bounds',
   *     'cursor_screen_point': 'eventa:event:electron:screen:cursor-screen-point',
   *   }
   * })
   * ```
   */
  eventSubscriptions?: Record<string, string>
}

/**
 * Create an eventa context backed by Tauri IPC.
 *
 * - Outbound invoke events are intercepted and routed to `@tauri-apps/api/core.invoke()`
 * - Tauri events can be subscribed to and emitted into the context
 */
export function createContext(options?: CreateContextOptions) {
  const ctx = createEventaContext()
  const unlisteners: UnlistenFn[] = []
  const pendingSetup: Promise<void>[] = []

  // Intercept all outbound events from the context.
  // When an invoke send event is detected, call the corresponding
  // Tauri command and emit the response (or error) back.
  ctx.on(
    and(
      matchBy((e: any) => e._flowDirection === OUTBOUND || !e._flowDirection),
      matchBy('*'),
    ),
    async (event: any) => {
      const type: string = event.type
      if (!isInvokeSendType(type))
        return

      const tag = extractTag(type)
      const commandName = channelToCommand(tag)
      const receiveType = toReceiveType(type)
      const receiveErrorType = toReceiveErrorType(type)

      try {
        // event.body is the payload passed to the invoke function
        const result = await tauriInvoke(commandName, event.body ?? {})
        ctx.emit(defineInboundEventa(receiveType), result)
      }
      catch (error) {
        ctx.emit(defineInboundEventa(receiveErrorType), { error })
      }
    },
  )

  // Subscribe to Tauri events and bridge them into the eventa context
  if (options?.eventSubscriptions) {
    for (const [tauriEventName, eventaChannel] of Object.entries(options.eventSubscriptions)) {
      const setup = tauriListen(tauriEventName, ((ev: { payload: unknown }) => {
        ctx.emit(defineInboundEventa(eventaChannel), ev.payload)
      }) as EventCallback<unknown>).then((unlisten) => {
        unlisteners.push(unlisten)
      })
      pendingSetup.push(setup)
    }
  }

  return {
    context: ctx,
    /**
     * Wait for all event subscriptions to be established.
     * Call this before using event-based features.
     */
    ready: () => Promise.all(pendingSetup).then(() => {}),
    dispose: () => {
      for (const unlisten of unlisteners) {
        unlisten()
      }
      unlisteners.length = 0
    },
  }
}

/**
 * Listen to a Tauri event and bridge it into an existing eventa context.
 * Returns an unlisten function.
 */
export async function listenTauriEvent(
  ctx: ReturnType<typeof createEventaContext>,
  tauriEventName: string,
  eventaChannel: string,
): Promise<UnlistenFn> {
  return tauriListen(tauriEventName, ((ev: { payload: unknown }) => {
    ctx.emit(defineInboundEventa(eventaChannel), ev.payload)
  }) as EventCallback<unknown>)
}

/**
 * Emit a Tauri event from the frontend to Rust.
 */
export async function emitTauriEvent(eventName: string, payload?: unknown): Promise<void> {
  await tauriEmit(eventName, payload)
}
