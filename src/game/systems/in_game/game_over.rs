use super::*;

fn spawn_winner_text(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut commands: Commands,
    query: Query<&Window>,
) {
    let window = query.single();
    // let dimensions = [window.width(), window.height()];

    for ev in ev_game_over.read() {
        let mut game_over_screen = commands.spawn((
            Node {
                width: Val::Px(window.width()),
                height: Val::Px(window.height()),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(25., 0., 255., 0.5)),
            InGameEntity,
        ));

        match ev.state {
            GameOvers::Winner => {
                game_over_screen.with_child((
                    Node {
                        width: Val::Px(window.width()),
                        ..default()
                    },
                    Text::new(format!("Player {} won", ev.player.unwrap())),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont::from_font_size(125.),
                    InGameEntity,
                ));
            }
            GameOvers::Tie => {
                game_over_screen.with_child((
                    Node {
                        width: Val::Px(window.width()),
                        ..default()
                    },
                    Text::new("It's a tie!"),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont::from_font_size(125.),
                    InGameEntity,
                ));
            }
        }
    }
}

fn spawn_press_enter_text(mut commands: Commands, windows_query: Query<&Window>) {
    let window = windows_query.single();

    commands
        .spawn((
            Node {
                width: Val::Px(window.width()),
                height: Val::Px(window.height()),
                align_content: AlignContent::Center,
                align_items: AlignItems::End,
                ..default()
            },
            GlobalZIndex(2),
            InGameEntity,
        ))
        .with_child((
            Node {
                width: Val::Px(window.width()),
                ..default()
            },
            Text::new("Press Enter to exit"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont::from_font_size(75.),
            InGameEntity,
        ));
}

fn spawn_press_space_text(mut commands: Commands, windows_query: Query<&Window>) {
    let window = windows_query.single();

    commands
        .spawn((
            Node {
                width: Val::Px(window.width()),
                height: Val::Px(window.height()),
                align_content: AlignContent::Center,
                align_items: AlignItems::End,
                bottom: Val::Px(100.),
                ..default()
            },
            GlobalZIndex(2),
            InGameEntity,
        ))
        .with_child((
            Node {
                width: Val::Px(window.width()),
                ..default()
            },
            Text::new("Press Space to play again"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont::from_font_size(75.),
            InGameEntity,
        ));
}

fn wait_for_input_to_exit_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Enter {
            next_app_state.set(AppStates::Menu);
        }
    }
}

fn wait_for_input_to_start_new_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Space {
            next_app_state.set(AppStates::Loading);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayStates::GameOver),
        (
            spawn_winner_text,
            spawn_press_enter_text,
            spawn_press_space_text,
        ),
    );
    // app.add_systems(OnExit(PlayStates::GameOver));

    app.add_systems(
        Update,
        (
            wait_for_input_to_exit_game,
            wait_for_input_to_start_new_game,
        )
            .run_if(in_state(PlayStates::GameOver))
            .run_if(in_state(AppStates::InGame)),
    );
}
