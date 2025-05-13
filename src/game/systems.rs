use super::*;

pub mod in_game;
pub mod loading;
pub mod menu;

// Global systems go here

fn clean_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive()
    }
}

fn events_empty<T: Event>(events: Res<Events<T>>) -> bool {
    events.is_empty()
}

fn clean_deletable_audios(
    query_deletable_audios: Query<(Entity, &AudioSink), With<DeletableAudio>>,
    mut commands: Commands,
) {
    for (entity, audio_sink) in &query_deletable_audios {
        if audio_sink.empty() {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn audio_react_to_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut commands: Commands,
    click_audio: Res<assets::ClickAudio>,
) {
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Enter {
            if let Some(audio) = click_audio.audio.as_ref() {
                commands.spawn((
                    AudioPlayer(audio.clone()),
                    PlaybackSettings { ..default() },
                    DeletableAudio,
                ));
            };

            next_app_state.set(AppStates::InGame);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, clean_deletable_audios);
}
