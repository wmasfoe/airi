//! Compile-time verification that all commands are properly registered
//! and their types are compatible with tauri-specta.
//!
//! These tests verify the command signatures compile correctly.
//! Runtime integration tests require a running Tauri app.

use stage_tamagotchi_tauri_lib::run;

/// Verify the app entry point compiles (all commands registered).
/// This is a compile-time check, not a runtime test.
#[test]
fn commands_compile() {
    // If this compiles, all commands are properly registered with tauri-specta.
    // We don't actually run the app in tests.
    let _ = std::any::type_name_of_val(&run);
}
