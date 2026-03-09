## ADDED Requirements

### Requirement: Vite build pipeline for Tauri

The system SHALL configure Vite to build the Vue frontend for Tauri's webview. The Vite config MUST:
- Use `@vitejs/plugin-vue` for Vue SFC compilation
- Configure `unplugin-vue-router` for file-based routing
- Configure `vite-plugin-vue-layouts` for layout support
- Set the build output to Tauri's expected `dist/` directory
- Resolve `@proj-airi/*` workspace package aliases correctly

#### Scenario: Dev server starts with HMR
- **WHEN** developer runs `cargo tauri dev`
- **THEN** Vite dev server starts, Tauri opens the webview pointing to the dev server URL, and HMR updates reflect in the webview within 2 seconds

#### Scenario: Production build outputs static assets
- **WHEN** developer runs `cargo tauri build`
- **THEN** Vite produces optimized static assets in the dist directory that Tauri bundles into the application

### Requirement: Shared package resolution

The Vite config SHALL resolve all shared workspace packages used by the renderer:
- `@proj-airi/stage-ui` (core business components/composables/stores)
- `@proj-airi/stage-ui-three` (Three.js bindings)
- `@proj-airi/stage-shared` (shared logic)
- `@proj-airi/ui` (primitives)
- `@proj-airi/i18n` (translations)
- `@proj-airi/stage-pages` (shared page bases)
- `@proj-airi/stage-layouts` (layout components)

#### Scenario: Stage-ui components render correctly
- **WHEN** the Tauri app loads a page that uses `@proj-airi/stage-ui` components
- **THEN** the components render without import resolution errors

#### Scenario: Three.js scene renders
- **WHEN** the Tauri app loads a page with `@proj-airi/stage-ui-three` components
- **THEN** the Three.js canvas renders in the webview without WebGL/WebGPU errors

### Requirement: UnoCSS integration

The Vite config SHALL include UnoCSS with the same configuration as the existing Electron app, referencing the root `uno.config.ts` or a compatible local config.

#### Scenario: UnoCSS utility classes work
- **WHEN** the Vue frontend uses UnoCSS utility classes (e.g., `flex`, `bg-white/50`)
- **THEN** the styles are correctly applied in the Tauri webview

### Requirement: Asset path handling

The system SHALL handle asset paths correctly for Tauri's asset protocol. Static assets (images, fonts, models) MUST be accessible via Tauri's `asset://` protocol or relative paths.

#### Scenario: Font files load correctly
- **WHEN** the application loads and font-face declarations reference local font files
- **THEN** the fonts render correctly in the webview without CORS or path errors

#### Scenario: 3D model assets load
- **WHEN** a Three.js scene loads a GLTF/GLB model from the assets directory
- **THEN** the model loads successfully via the appropriate Tauri asset protocol
