use bevy::prelude::*;

/// Adds the Bevy UI debug overlay for dev builds.
///
/// Only compiled when the `dev_native` feature (which enables `bevy_ui_debug`)
/// is active. Press **T** at any time to toggle node-outline wireframes over
/// every UI element.
///
/// `UiDebugOptions` is already initialised by `UiRenderPlugin`; this plugin
/// only wires up the toggle system.
pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_ui_debug")]
        app.add_systems(Update, toggle_overlay);
    }
}

/// Toggles the UI debug overlay on/off when T is pressed.
#[cfg(feature = "bevy_ui_debug")]
fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::ui_render::UiDebugOptions>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        options.enabled = !options.enabled;
    }
}