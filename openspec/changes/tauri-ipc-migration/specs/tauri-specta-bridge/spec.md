## ADDED Requirements

### Requirement: Type-safe Rust-to-TypeScript bridge via tauri-specta

The system SHALL 使用 `tauri-specta` 从 Rust command 定义自动生成 TypeScript 类型绑定。生成的类型 MUST 与现有 eventa 合约中定义的 TypeScript 接口兼容。

#### Scenario: Generated types match eventa contracts
- **WHEN** `tauri-specta` 从 Rust commands 生成 TypeScript bindings
- **THEN** 生成的参数和返回值类型与 `apps/stage-tamagotchi/src/shared/eventa.ts` 中定义的接口（如 `ElectronMcpCallToolResult`、`PluginRegistrySnapshot`、`AutoUpdaterState`）结构一致

#### Scenario: Type generation runs at build time
- **WHEN** 开发者运行 `cargo tauri dev` 或 `cargo tauri build`
- **THEN** `tauri-specta` 自动生成/更新 TypeScript bindings 文件到前端 `src/` 目录

#### Scenario: Frontend uses generated types without manual casting
- **WHEN** 前端调用 Tauri command
- **THEN** TypeScript 编译器能推断出正确的参数和返回值类型，无需 `as` 类型断言

### Requirement: Shared type definitions between Rust and TypeScript

The system SHALL 在 Rust 侧定义与现有 TypeScript 接口对应的 struct/enum，使用 `serde` + `specta` derive macros。以下类型 MUST 有 Rust 对应物：

- `AutoUpdaterState`、`AutoUpdaterProgress`、`AutoUpdaterError`、`AutoUpdaterStatus`
- `ElectronMcpStdioConfigFile`、`ElectronMcpStdioServerConfig`、`ElectronMcpStdioApplyResult`
- `ElectronMcpStdioRuntimeStatus`、`ElectronMcpStdioServerRuntimeStatus`
- `ElectronMcpToolDescriptor`、`ElectronMcpCallToolPayload`、`ElectronMcpCallToolResult`
- `PluginRegistrySnapshot`、`PluginManifestSummary`、`PluginHostDebugSnapshot`
- `PluginCapabilityPayload`、`PluginCapabilityState`
- `WidgetSnapshot`、`WidgetsAddPayload`
- `RequestWindowPayload`、`RequestWindowPending`
- Window 相关：`Rectangle`（bounds）、`Display`、`Point`、`ResizeDirection`

#### Scenario: Rust struct serializes to expected JSON shape
- **WHEN** Rust 侧返回 `AutoUpdaterState { status: "idle", info: None, progress: None, error: None }`
- **THEN** 前端收到的 JSON 与现有 Electron 版本返回的格式一致

#### Scenario: Enum variants match TypeScript union types
- **WHEN** Rust 侧使用 `AutoUpdaterStatus::Checking`
- **THEN** 序列化为 `"checking"`，与 TypeScript 的 `AutoUpdaterStatus` union type 匹配
