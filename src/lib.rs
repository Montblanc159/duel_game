use bevy::prelude::*;

mod camera;
mod game;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(camera::plugin);
        app.add_plugins(game::plugin);

        // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
    }
}
