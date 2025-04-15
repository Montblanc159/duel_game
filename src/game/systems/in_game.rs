use super::*;

pub mod betting;
pub mod countdown;
pub mod fighting;
pub mod game_over;
pub mod paused;
pub mod preparing;
pub mod rounding_up;

fn reset_game(mut rounds: ResMut<RoundCounter>, mut game_over: ResMut<GameOver>) {
    rounds.0 = 1; // reset rounds
    game_over.0 = false; // reset game_over
}

fn pause_game(mut next_play_state: ResMut<NextState<PlayStates>>) {
    next_play_state.set(PlayStates::Paused);
}

fn launch_game(mut next_play_state: ResMut<NextState<PlayStates>>) {
    next_play_state.set(PlayStates::Countdown);
}

fn spawn_bg(mut commands: Commands, bg_texture: Res<assets::BgSprite>, window: Single<&Window>) {
    if let Some(texture) = bg_texture.sprite.as_ref() {
        commands.spawn((
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            Transform::from_xyz(0., 0., -1.),
            InGameEntity,
        ));
    };
}

fn listen_spawn_player_tick_ui(
    mut commands: Commands,
    mut ev_tick_player: EventReader<TickPlayerEvent>,
    query: Query<(&Player, &Transform), With<Player>>,
    window_query: Query<&Window>,
) {
    for ev in ev_tick_player.read() {
        let window = window_query.single();
        let dimensions = [450., 450.];

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
                        font_size: DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(Color::srgb(255., 0., 0.)),
                    TextLayout::new_with_justify(JustifyText::Center),
                    GlobalZIndex(1),
                    PlayerTickText,
                    InGameEntity,
                ));
            }
        }
    }
}

fn despawn_player_tick_ui(mut commands: Commands, query: Query<Entity, With<PlayerTickText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn animate_player_tick_text_opacity(
    mut colors: Query<&mut TextColor, With<PlayerTickText>>,
    time: Res<Time>,
) {
    for mut color in &mut colors {
        let alpha = color.0.alpha();
        color.0.set_alpha(alpha - time.delta_secs() * 0.65);
    }
}

fn animate_player_tick_font_size(
    mut text_fonts: Query<&mut TextFont, With<PlayerTickText>>,
    time: Res<Time>,
) {
    for mut text_font in &mut text_fonts {
        text_font.font_size += time.delta_secs() * 10.;
    }
}

fn spawn_timer_ui(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    let dimensions = [200., 200.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            top: Val::Px(75.),
            left: Val::Px((window.width() / 2.) - (dimensions[0] / 2.)),
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Text::default(),
        TextFont {
            font_size: DEFAULT_FONT_SIZE * 0.5,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        TimerUIText,
        InGameEntity,
    ));
}

fn despawn_timer_ui(mut commands: Commands, query: Query<Entity, With<TimerUIText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_players(
    mut commands: Commands,
    hand_texture: Res<assets::HandSpritesheet>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Some(texture) = hand_texture.spritesheet.as_ref() {
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(150), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.spawn((
            Player { value: 1 },
            Target { value: 2 },
            KeyAssignment(PLAYER_ONE_KEYS),
            PlayerState(PlayerStates::Idle),
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 1,
                }),
                custom_size: Some(Vec2::new(500., 500.)),
                ..default()
            },
            Transform::from_xyz(-350., 0.0, 0.0),
            InGameEntity,
        ));

        commands.spawn((
            Player { value: 2 },
            Target { value: 1 },
            KeyAssignment(PLAYER_TWO_KEYS),
            PlayerState(PlayerStates::Idle),
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 1,
                }),
                custom_size: Some(Vec2::new(500., 500.)),
                flip_x: true,
                ..default()
            },
            Transform::from_xyz(350., 0.0, 0.0),
            InGameEntity,
        ));
    }
}

fn spawn_play_state_text(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let dimensions = [250., 200.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            bottom: Val::Px(75. - (dimensions[1] / 2.)),
            left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
            ..default()
        },
        Text::default(),
        TextFont {
            font_size: DEFAULT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        PlayStateText,
        InGameEntity,
    ));
}

fn play_state_text_update(
    play_state: Res<State<PlayStates>>,
    mut query: Query<&mut Text, With<PlayStateText>>,
) {
    for mut text in &mut query {
        **text = format!("{:?}", play_state.get());
    }
}

fn spawn_round_number_text(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let dimensions = [250., 100.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            top: Val::Px(75. - (dimensions[1] / 2.)),
            left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
            ..default()
        },
        Text::new(format!("Round 1/{}", N_MAX_ROUND)),
        TextFont {
            font_size: DEFAULT_FONT_SIZE * 0.25,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        RoundNumberText,
        InGameEntity,
    ));
}

fn round_number_text_update(
    round_counter: Res<RoundCounter>,
    mut query: Query<&mut Text, With<RoundNumberText>>,
) {
    for mut text in &mut query {
        **text = format!("Round {}/{}", round_counter.0, N_MAX_ROUND);
    }
}

fn player_state_hand_texture_update(
    mut ev_player_state: EventReader<PlayerStateChangeEvent>,
    mut query_state: Query<(&PlayerState, &Player, &HandTextureIndices, &mut Sprite), With<Player>>,
) {
    for ev in ev_player_state.read() {
        for (player_state, player, hand_texture_indices, mut sprite) in &mut query_state {
            if ev.player == player.value {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = player_state.derive_hand_texture_index(hand_texture_indices);
                }
            }
        }
    }
}

fn spawn_health_bar(
    mut commands: Commands,
    query: Query<(&Player, &Transform), With<Player>>,
    texture: Res<assets::HealthSpritesheet>,
    window: Single<&Window>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Some(texture) = texture.spritesheet.as_ref() {
        let layout = TextureAtlasLayout::from_grid(UVec2 { x: 150, y: 25 }, 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        for (player, transform) in &query {
            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 0,
                    }),
                    custom_size: Some(Vec2::new(300., 50.)),
                    // flip_x: true,
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x * 1.5,
                    -window.height() / 2. + 75.,
                    1.,
                ),
                HealthBar {
                    value: player.value,
                },
                InGameEntity,
            ));
        }
    }
}

fn health_bar_update(
    mut query_ui: Query<(&HealthBar, &mut Sprite)>,
    query_state: Query<(&Health, &Player), With<Player>>,
) {
    for (health_bar, mut sprite) in &mut query_ui {
        for (health, player) in &query_state {
            if health_bar.value == player.value {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = 3_u8.checked_sub(health.value).unwrap_or(0) as usize;
                }
            }
        }
    }
}

fn spawn_mana_bar(
    mut commands: Commands,
    query: Query<(&Player, &Transform), With<Player>>,
    texture: Res<assets::ManaSpritesheet>,
    window: Single<&Window>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let dimensions = [25., 200.];

    if let Some(texture) = texture.spritesheet.as_ref() {
        let layout = TextureAtlasLayout::from_grid(UVec2 { x: 150, y: 11 }, 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        for (player, transform) in &query {
            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 0,
                    }),
                    custom_size: Some(Vec2::new(300., 22.)),
                    // flip_x: true,
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x * 1.5,
                    -window.height() / 2. + 120.,
                    1.,
                ),
                ManaBar {
                    value: player.value,
                },
                InGameEntity,
            ));
        }
    }
}

fn mana_bar_update(
    mut query_ui: Query<(&ManaBar, &mut Sprite)>,
    query_state: Query<(&Bullets, &Player), With<Player>>,
) {
    for (mana_bar, mut sprite) in &mut query_ui {
        for (bullets, player) in &query_state {
            if mana_bar.value == player.value {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = 3_u8.checked_sub(bullets.value).unwrap_or(0) as usize;
                }
            }
        }
    }
}

fn spawn_stamina_bar(
    mut commands: Commands,
    query: Query<(&Player, &Transform), With<Player>>,
    texture: Res<assets::StaminaSpritesheet>,
    window: Single<&Window>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let dimensions = [25., 200.];

    if let Some(texture) = texture.spritesheet.as_ref() {
        let layout = TextureAtlasLayout::from_grid(UVec2 { x: 150, y: 10 }, 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        for (player, transform) in &query {
            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 0,
                    }),
                    custom_size: Some(Vec2::new(300., 20.)),
                    // flip_x: true,
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x * 1.5,
                    -window.height() / 2. + 145.,
                    1.,
                ),
                StaminaBar {
                    value: player.value,
                },
                InGameEntity,
            ));
        }
    }
}

fn stamina_bar_update(
    mut query_ui: Query<(&StaminaBar, &mut Sprite)>,
    query_state: Query<(&Dodges, &Player), With<Player>>,
) {
    for (mana_bar, mut sprite) in &mut query_ui {
        for (dodges, player) in &query_state {
            if mana_bar.value == player.value {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = 3_u8.checked_sub(dodges.value).unwrap_or(0) as usize;
                }
            }
        }
    }
}

fn next_play_state(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    next_play_state.set(play_state.get().next());
}

fn wait_for_input_to_next_play_state(
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

fn listen_game_overs(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    for _ev in ev_game_over.read() {
        next_play_state.set(PlayStates::GameOver);
    }
}

fn listen_spawn_alert_text(
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
                font_size: DEFAULT_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            AlertText,
            InGameEntity,
        ));
    }
}

fn despawn_alert_text(mut commands: Commands, query: Query<Entity, With<AlertText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppStates::InGame),
        (
            spawn_bg,
            spawn_players,
            (
                spawn_play_state_text,
                spawn_round_number_text,
                spawn_health_bar,
                spawn_mana_bar,
                spawn_stamina_bar,
            ),
            launch_game,
        )
            .chain(),
    );

    app.add_systems(
        OnExit(AppStates::InGame),
        (pause_game, reset_game, clean_system::<InGameEntity>).chain(),
    );

    app.add_systems(
        Update,
        (
            listen_game_overs,
            listen_spawn_alert_text,
            listen_spawn_player_tick_ui,
            animate_player_tick_text_opacity,
            animate_player_tick_font_size,
            round_number_text_update,
            play_state_text_update,
            player_state_hand_texture_update,
            health_bar_update,
            mana_bar_update,
            stamina_bar_update,
        )
            .run_if(in_state(AppStates::InGame)),
    );
}
