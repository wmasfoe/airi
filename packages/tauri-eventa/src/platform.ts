/**
 * Platform detection utilities for choosing between Electron and Tauri adapters.
 *
 * Tauri injects `import.meta.env.TAURI` at build time via Vite,
 * allowing tree-shaking of the unused adapter.
 */

/**
 * Returns `true` when running inside a Tauri webview.
 * Uses the build-time env var injected by Tauri's Vite plugin.
 */
export function isTauri(): boolean {
  try {
    // Tauri v2 injects this at build time
    return !!(import.meta as any).env?.TAURI
  }
  catch {
    return false
  }
}

/**
 * Returns `true` when running inside an Electron renderer.
 */
export function isElectron(): boolean {
  return typeof window !== 'undefined'
    && typeof (window as any).electron?.ipcRenderer !== 'undefined'
}
