mod menu;
mod state;

use bevy::prelude::*;
use bevy_editor_pls::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_state(state::Main::Menu)
        .add_system_set(SystemSet::on_enter(state::Main::Menu).with_system(menu::setup))
        .run();
}
