## ADDED Requirements

### Requirement: pnpm workspace integration

The system SHALL add `apps/stage-tamagotchi-tauri` to the pnpm workspace configuration so that workspace package resolution works for the Tauri app's frontend dependencies.

#### Scenario: Workspace packages resolve
- **WHEN** `pnpm install` runs at the monorepo root
- **THEN** the Tauri app's `node_modules` correctly resolves all `workspace:^` dependencies

#### Scenario: Package name follows convention
- **WHEN** the Tauri app's `package.json` is created
- **THEN** the package name is `@proj-airi/stage-tamagotchi-tauri` following the existing naming convention

### Requirement: Turbo pipeline configuration

The system SHALL add Tauri-specific build tasks to `turbo.json`:
- `@proj-airi/stage-tamagotchi-tauri#build`: builds the Tauri application
- `@proj-airi/stage-tamagotchi-tauri#dev`: starts the Tauri dev server
- Build task MUST depend on shared package builds (`@proj-airi/stage-ui#build`, etc.)

#### Scenario: Turbo build runs Tauri build
- **WHEN** developer runs `pnpm -F @proj-airi/stage-tamagotchi-tauri build`
- **THEN** turbo executes dependent package builds first, then runs the Tauri build

#### Scenario: Turbo caches Tauri builds
- **WHEN** developer runs the build twice without changes
- **THEN** the second run uses turbo cache and completes in under 5 seconds

### Requirement: Cargo workspace integration

The system SHALL add `apps/stage-tamagotchi-tauri/src-tauri` to the root `Cargo.toml` workspace members list.

#### Scenario: Cargo workspace includes Tauri app
- **WHEN** `cargo build --workspace` runs at the monorepo root
- **THEN** the Tauri app compiles as part of the workspace alongside existing crates

#### Scenario: Shared workspace dependencies
- **WHEN** the Tauri app's `Cargo.toml` references workspace plugins
- **THEN** the dependencies resolve correctly via the workspace dependency table

### Requirement: Cross-platform build scripts

The system SHALL provide npm scripts in the Tauri app's `package.json` for building on each platform:
- `build:mac`: macOS build (`.dmg`)
- `build:win`: Windows build (`.msi`/`.exe`)
- `build:linux`: Linux build (`.deb`/`.AppImage`)
- `dev`: development mode with HMR

#### Scenario: macOS build produces DMG
- **WHEN** developer runs `pnpm -F @proj-airi/stage-tamagotchi-tauri build:mac` on macOS
- **THEN** a `.dmg` installer is produced in the `src-tauri/target/release/bundle/` directory

#### Scenario: Dev mode starts successfully
- **WHEN** developer runs `pnpm -F @proj-airi/stage-tamagotchi-tauri dev`
- **THEN** the Tauri dev server starts with Vite HMR and the application window opens
