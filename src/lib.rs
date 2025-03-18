use bevy::{asset::load_internal_binary_asset, prelude::*};

mod camera;
mod game;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);

        // Change default font
        load_internal_binary_asset!(
            app,
            TextFont::default().font,
            "../assets/fonts/yoster.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app.add_plugins(camera::plugin);
        app.add_plugins(game::plugin);

        // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
    }
}
