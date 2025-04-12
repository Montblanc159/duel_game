use bevy::{asset::load_internal_binary_asset, prelude::*, window::WindowResolution};

mod camera;
mod game;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: (Some(Window {
                        resolution: WindowResolution::new(1440., 1080.),
                        title: "Wiz".into(),
                        ..default()
                    })),
                    ..default()
                }),
        );

        // Change default font
        load_internal_binary_asset!(
            app,
            TextFont::default().font,
            "../assets/fonts/game-font.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app.add_plugins(camera::plugin);
        app.add_plugins(game::plugin);

        // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
    }
}
