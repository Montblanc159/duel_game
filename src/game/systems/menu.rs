use super::*;

fn wait_for_input_to_start_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Enter {
            next_app_state.set(AppStates::InGame);
        }
    }
}

fn spawn_menu_audio(mut commands: Commands, menu_audio: Res<assets::MenuAudio>) {
    if let Some(audio) = menu_audio.audio.as_ref() {
        commands.spawn((
            AudioPlayer(audio.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            },
            MenuEntity,
        ));
    }
}

fn spawn_start_game_ui(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    // let dimensions = [window.width(), window.height()];

    commands
        .spawn((
            Node {
                width: Val::Px(window.width()),
                height: Val::Px(window.height()),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(25., 0., 255., 0.5)),
            MenuEntity,
        ))
        .with_child((
            Node {
                width: Val::Px(window.width()),
                ..default()
            },
            Text::new("Press Enter to start game !"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont::from_font_size(125.),
            MenuEntity,
        ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppStates::Menu),
        (spawn_menu_audio, spawn_start_game_ui),
    );

    app.add_systems(
        Update,
        wait_for_input_to_start_game.run_if(in_state(AppStates::Menu)),
    );

    app.add_systems(OnExit(AppStates::Menu), clean_system::<MenuEntity>);
}
