mod menu;
mod state;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_loopless_state(state::Main::Menu)
        .add_enter_system(state::Main::Menu, menu::setup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(state::Main::Menu)
                .with_system(menu::update)
                .into(),
        )
        .add_exit_system(state::Main::Menu, menu::cleanup)
        .run();
}
