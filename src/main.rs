use bevy::prelude::*;

mod hit_a_key;

use crate::hit_a_key::HitAKeyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HitAKeyPlugin)
        .run();
}
