mod components;
mod systems;

pub use components::*;
use systems::*;

use crate::AppState;
use bevy::prelude::*;

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hotbar>()
            .add_systems(OnEnter(AppState::InGame), spawn_hotbar)
            .add_systems(OnExit(AppState::InGame), despawn_hotbar)
            .add_systems(
                Update,
                (
                    handle_hotbar_keyboard,
                    handle_hotbar_scroll,
                    update_hotbar_selection,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
