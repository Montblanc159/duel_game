use bevy::prelude::*;
use rand::{distr::StandardUniform, prelude::*};

mod components;
mod events;
mod resources;
mod settings;
mod states;
mod systems;
mod ui_components;
mod ui_defaults;

use components::*;
use events::*;
use resources::*;
use settings::*;
use states::*;
use ui_components::*;
use ui_defaults::*;

// Game
// ================================================================

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, resources::plugin, states::plugin));

    app.add_plugins((
        systems::loading::plugin,
        systems::menu::plugin,
        systems::in_game::plugin,
    ));
    app.add_plugins((
        // Chronological order
        systems::in_game::preparing::plugin,
        systems::in_game::countdown::plugin,
        systems::in_game::betting::plugin,
        systems::in_game::fighting::plugin,
        systems::in_game::rounding_up::plugin,
        systems::in_game::game_over::plugin,
    ));
}
