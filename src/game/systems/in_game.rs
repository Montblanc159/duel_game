use super::*;

pub mod betting;
pub mod countdown;
pub mod fighting;
pub mod game_over;
pub mod paused;
pub mod preparing;
pub mod rounding_up;

pub use betting::*;
pub use countdown::*;
pub use fighting::*;
pub use game_over::*;
// pub use paused::*;
pub use preparing::*;
pub use rounding_up::*;

pub fn reset_game(mut rounds: ResMut<RoundCounter>, mut game_over: ResMut<GameOver>) {
    rounds.0 = 1; // reset rounds
    game_over.0 = false; // reset game_over
}

pub fn pause_game(mut next_play_state: ResMut<NextState<PlayStates>>) {
    next_play_state.set(PlayStates::Paused);
}

pub fn launch_game(mut next_play_state: ResMut<NextState<PlayStates>>) {
    next_play_state.set(PlayStates::Countdown);
}

pub fn listen_spawn_player_tick_ui(
    mut commands: Commands,
    mut ev_tick_player: EventReader<TickPlayerEvent>,
    query: Query<(&Player, &Transform), With<Player>>,
    window_query: Query<&Window>,
) {
    for ev in ev_tick_player.read() {
        let window = window_query.single();
        let dimensions = [450., 250.];

        for (player, transform) in &query {
            if player.value == ev.player {
                commands.spawn((
                    Node {
                        width: Val::Px(dimensions[0]),
                        height: Val::Px(dimensions[1]),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(
                            transform.translation.y + window.height() / 2. - (dimensions[1] / 2.),
                        ),
                        left: Val::Px(
                            transform.translation.x + window.width() / 2. - (dimensions[0] / 2.),
                        ),
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Text::new(&ev.value),
                    TextFont {
                        font_size: 25.,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    PlayerTickText,
                    InGameEntity,
                ));
            }
        }
    }
}

pub fn despawn_player_tick_ui(mut commands: Commands, query: Query<Entity, With<PlayerTickText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn animate_player_tick_text_opacity(
    mut colors: Query<&mut TextColor, With<PlayerTickText>>,
    time: Res<Time>,
) {
    for mut color in &mut colors {
        let alpha = color.0.alpha();
        color.0.set_alpha(alpha - time.delta_secs() * 0.65);
    }
}

pub fn animate_player_tick_font_size(
    mut text_fonts: Query<&mut TextFont, With<PlayerTickText>>,
    time: Res<Time>,
) {
    for mut text_font in &mut text_fonts {
        text_font.font_size += time.delta_secs() * 10.;
    }
}

pub fn spawn_timer_ui(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    let dimensions = [70., 70.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            top: Val::Px(DEFAULT_MARGIN - (dimensions[1] / 2.)),
            left: Val::Px((window.width() / 2.) - (dimensions[0] / 2.)),
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Text::default(),
        TextFont {
            font_size: 50.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        TimerUIText,
        InGameEntity,
    ));
}

pub fn despawn_timer_ui(mut commands: Commands, query: Query<Entity, With<TimerUIText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player { value: 1 },
        KeyAssignment(PLAYER_ONE_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(255., 0., 0.))),
        Transform::from_xyz(-250., 0.0, 0.0),
        InGameEntity,
    ));

    commands.spawn((
        Player { value: 2 },
        KeyAssignment(PLAYER_TWO_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 255.))),
        Transform::from_xyz(250., 0.0, 0.0),
        InGameEntity,
    ));
}

pub fn spawn_play_state_text(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let dimensions = [250., 50.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            bottom: Val::Px(25. - (dimensions[1] / 2.)),
            left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
            ..default()
        },
        Text::default(),
        TextFont {
            font_size: 30.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        PlayStateText,
        InGameEntity,
    ));
}

pub fn play_state_text_update(
    play_state: Res<State<PlayStates>>,
    mut query: Query<&mut Text, With<PlayStateText>>,
) {
    for mut text in &mut query {
        **text = format!("{:?}", play_state.get());
    }
}

pub fn spawn_round_number_text(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let dimensions = [250., 50.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            bottom: Val::Px(45. - (dimensions[1] / 2.)),
            left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
            ..default()
        },
        Text::new(format!("Round 1/{}", N_MAX_ROUND)),
        TextFont {
            font_size: 15.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        RoundNumberText,
        InGameEntity,
    ));
}

pub fn round_number_text_update(
    round_counter: Res<RoundCounter>,
    mut query: Query<&mut Text, With<RoundNumberText>>,
) {
    for mut text in &mut query {
        **text = format!("Round {}/{}", round_counter.0, N_MAX_ROUND);
    }
}

pub fn spawn_player_state_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<(&Transform, &PlayerState, &Player), With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [250., 125.];

    for (transform, player_state, player) in &query {
        commands.spawn((
            Node {
                width: Val::Px(dimensions[0]),
                height: Val::Px(dimensions[1]),
                position_type: PositionType::Absolute,
                bottom: Val::Px(
                    transform.translation.y + window.height() / 2. - (dimensions[1] / 2.) - 125.,
                ),
                left: Val::Px(transform.translation.x + window.width() / 2. - (dimensions[0] / 2.)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::new(format!("{:?}", player_state)),
            TextFont {
                font_size: 10.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            PlayerStateText {
                value: player.value,
            },
            InGameEntity,
        ));
    }
}

pub fn player_state_text_update(
    mut ev_player_state: EventReader<PlayerStateChangeEvent>,
    mut query_ui: Query<(&mut Text, &PlayerStateText), With<PlayerStateText>>,
    query_state: Query<(&PlayerState, &Player), With<Player>>,
) {
    for _ev in ev_player_state.read() {
        for (mut text, player_state_text) in &mut query_ui {
            for (player_state, player) in &query_state {
                if player_state_text.value == player.value {
                    **text = format!("{:?}", player_state);
                }
            }
        }
    }
}

pub fn spawn_health_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<&Player, With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];

    for player in &query {
        let left_position = if player.value == 1 {
            DEFAULT_MARGIN
        } else {
            window.width() - DEFAULT_MARGIN
        };

        commands.spawn((
            Node {
                width: Val::Px(dimensions[0]),
                height: Val::Px(dimensions[1]),
                position_type: PositionType::Absolute,
                top: Val::Px(25.),
                left: Val::Px(left_position - (dimensions[0] / 2.)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::default(),
            TextFont {
                font_size: 20.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            HealthText {
                value: player.value,
            },
            InGameEntity,
        ));
    }
}

pub fn health_text_update(
    mut query_ui: Query<(&mut Text, &HealthText), With<HealthText>>,
    query_state: Query<(&Health, &Player), With<Player>>,
) {
    for (mut text, health_text) in &mut query_ui {
        for (health, player) in &query_state {
            if health_text.value == player.value {
                **text = format!("{} PV", health.value);
            }
        }
    }
}

pub fn spawn_bullet_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<&Player, With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];

    for player in &query {
        let left_position = if player.value == 1 {
            DEFAULT_MARGIN
        } else {
            window.width() - DEFAULT_MARGIN
        };

        commands.spawn((
            Node {
                width: Val::Px(dimensions[0]),
                height: Val::Px(dimensions[1]),
                position_type: PositionType::Absolute,
                top: Val::Px(100.),
                left: Val::Px(left_position - (dimensions[0] / 2.)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::default(),
            TextFont {
                font_size: 20.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            BulletText {
                value: player.value,
            },
            InGameEntity,
        ));
    }
}

pub fn bullet_text_update(
    mut query_ui: Query<(&mut Text, &BulletText), With<BulletText>>,
    query_state: Query<(&Bullets, &Player), With<Player>>,
) {
    for (mut text, bullet_text) in &mut query_ui {
        for (bullets, player) in &query_state {
            if bullet_text.value == player.value {
                **text = format!("{} bullets", bullets.value);
            }
        }
    }
}

pub fn spawn_dodge_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<&Player, With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];

    for player in &query {
        let left_position = if player.value == 1 {
            DEFAULT_MARGIN
        } else {
            window.width() - DEFAULT_MARGIN
        };

        commands.spawn((
            Node {
                width: Val::Px(dimensions[0]),
                height: Val::Px(dimensions[1]),
                position_type: PositionType::Absolute,
                top: Val::Px(175.),
                left: Val::Px(left_position - (dimensions[0] / 2.)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::default(),
            TextFont {
                font_size: 20.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            DodgeText {
                value: player.value,
            },
            InGameEntity,
        ));
    }
}

pub fn dodge_text_update(
    mut query_ui: Query<(&mut Text, &DodgeText), With<DodgeText>>,
    query_state: Query<(&Dodges, &Player), With<Player>>,
) {
    for (mut text, dodge_text) in &mut query_ui {
        for (dodges, player) in &query_state {
            if dodge_text.value == player.value {
                **text = format!("{} dodges", dodges.value);
            }
        }
    }
}

pub fn next_play_state(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    next_play_state.set(play_state.get().next());
}

pub fn wait_for_input_to_next_play_state(
    keys: Res<ButtonInput<KeyCode>>,
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Space {
            next_play_state.set(play_state.get().next());
        }
    }
}

pub fn listen_game_overs(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    for ev in ev_game_over.read() {
        next_play_state.set(PlayStates::GameOver);

        match ev.state {
            GameOvers::Tie => println!("It's a tie!"),
            GameOvers::Winner => println!("Player {} won!", ev.player.unwrap()),
        }
    }
}

pub fn listen_spawn_alert_text(
    mut ev_alert: EventReader<AlertEvent>,
    window_query: Query<&Window>,
    mut commands: Commands,
) {
    for ev in ev_alert.read() {
        let window = window_query.single();
        let dimensions = [window.width(), 250.];

        commands.spawn((
            Node {
                width: Val::Px(dimensions[0]),
                height: Val::Px(dimensions[1]),
                position_type: PositionType::Absolute,
                bottom: Val::Px(window.height() / 2. - (dimensions[1] / 2.)),
                left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::new(&ev.value),
            TextFont {
                font_size: 100.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            AlertText,
            InGameEntity,
        ));
    }
}

pub fn despawn_alert_text(mut commands: Commands, query: Query<Entity, With<AlertText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
