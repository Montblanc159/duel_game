use bevy::prelude::*;
use rand::prelude::*;

pub struct HitAKeyPlugin;

// Settings
// ================================================================

const N_KEYS_PER_PLAYER: usize = 2;
const PLAYER_KEY_ASSIGNMENTS: [[KeyCode; N_KEYS_PER_PLAYER]; 2] = [
    [KeyCode::KeyS, KeyCode::KeyD],
    [KeyCode::KeyJ, KeyCode::KeyK],
];
const PLAYER_ONE_KEYS: [KeyCode; N_KEYS_PER_PLAYER] = PLAYER_KEY_ASSIGNMENTS[0];
const PLAYER_TWO_KEYS: [KeyCode; N_KEYS_PER_PLAYER] = PLAYER_KEY_ASSIGNMENTS[1];
const N_BULLETS: u8 = 3;
const N_DODGES: u8 = 3;
const N_FACETED_DICE: u8 = 100;
const N_MAX_ROUND: u8 = 5;

// Components
// ================================================================

#[derive(Component)]
struct Player {
    n: u8,
}

#[derive(Component)]
struct Health {
    value: u8,
}

#[derive(Component)]
struct Luck {
    n: u8,
}

#[derive(Component)]
struct Marksmanship {
    n: u8,
}

#[derive(Component)]
struct Dodges {
    n: u8,
}

#[derive(Component)]
struct Bullets {
    n: u8,
}

#[derive(Component, Debug)]
struct PlayerState(PlayerStates);

#[derive(Component)]
struct KeyAssignment([KeyCode; N_KEYS_PER_PLAYER]);

// UI components

#[derive(Component)]
struct PlayStateText;

#[derive(Component)]
struct RoundNumberText;

#[derive(Component)]
struct PlayerStateText {
    n: u8,
}

#[derive(Component)]
struct HealthText {
    n: u8,
}

#[derive(Component)]
struct BulletText {
    n: u8,
}

#[derive(Component)]
struct DodgeText {
    n: u8,
}
// Events
// ================================================================

#[derive(Event)]
struct EndGameEvent {
    player: Option<u8>,
    state: EndGames,
}

#[derive(Event)]
struct PlayerStateChangeEvent;

// Enums
// ================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerStates {
    Idle,
    Attacking,
    NotAttacking,
    Dodging,
    NotDodging,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EndGames {
    Tie,
    Winner,
}

// States
// ================================================================

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlayStates {
    Preparing,
    Betting,
    Fighting,
    RoundingUp,
    Finished,
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// enum GameStates {
//     Playing,
//     Menu,
//     Paused
// }

// Resources
// ================================================================
#[derive(Resource)]
struct PlayStateTimer(Timer);

#[derive(Resource)]
struct RoundCounter(u8);

// Game
// ================================================================

impl Plugin for HitAKeyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayStates::Betting);
        app.insert_resource(PlayStateTimer(Timer::from_seconds(8.0, TimerMode::Once)));
        app.insert_resource(RoundCounter(1));
        app.add_event::<EndGameEvent>();
        app.add_event::<PlayerStateChangeEvent>();
        app.add_systems(
            Startup,
            (
                (spawn_camera, spawn_players),
                (
                    spawn_play_state_text,
                    spawn_player_state_text,
                    spawn_round_number_text,
                    spawn_health_text,
                    spawn_bullet_text,
                    spawn_dodge_text,
                ),
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                listen_endgames,
                (
                    prepare_player_for_next_round,
                    restore_bullet,
                    restore_dodge,
                    increase_round_counter,
                    reset_game_timer,
                    next_play_state,
                )
                    .chain()
                    .run_if(in_state(PlayStates::RoundingUp)),
                player_state_text_update,
                round_number_text_update,
                play_state_text_update,
                health_text_update,
                bullet_text_update,
                dodge_text_update,
                set_timed_play_state,
                set_player_state.run_if(in_state(PlayStates::Betting)),
                (
                    display_play_state,
                    decrease_health,
                    (decrease_bullets, decrease_dodges),
                    reset_game_timer,
                    next_play_state,
                )
                    .chain()
                    .run_if(in_state(PlayStates::Fighting)),
                (
                    (display_play_state, print_stats, check_if_dead)
                        .chain()
                        .run_if(in_state(PlayStates::RoundingUp)),
                    check_if_last_round.run_if(in_state(PlayStates::RoundingUp)),
                    check_if_out_of_ammo.run_if(in_state(PlayStates::RoundingUp)),
                )
                    .chain(),
            )
                .chain(),
        );
    }
}

// Systems
// ================================================================

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
}

fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player { n: 1 },
        Health { value: 3 },
        Luck {
            n: N_FACETED_DICE - 10,
        },
        Marksmanship { n: N_FACETED_DICE },
        Dodges { n: N_DODGES },
        Bullets { n: N_BULLETS },
        KeyAssignment(PLAYER_ONE_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(255., 0., 0.))),
        Transform::from_xyz(-250., 0.0, 0.0),
    ));

    commands.spawn((
        Player { n: 2 },
        Health { value: 3 },
        Luck {
            n: N_FACETED_DICE - 10,
        },
        Marksmanship { n: N_FACETED_DICE },
        Dodges { n: N_DODGES },
        Bullets { n: N_BULLETS },
        KeyAssignment(PLAYER_TWO_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 255.))),
        Transform::from_xyz(250., 0.0, 0.0),
    ));
}

fn spawn_play_state_text(mut commands: Commands, query: Query<&Window>) {
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

fn spawn_player_state_text(
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
            PlayerStateText { n: player.n },
        ));
    }
}

fn player_state_text_update(
    mut ev_player_state: EventReader<PlayerStateChangeEvent>,
    mut query_ui: Query<(&mut Text, &PlayerStateText), With<PlayerStateText>>,
    query_state: Query<(&PlayerState, &Player), With<Player>>,
) {
    for _ev in ev_player_state.read() {
        for (mut text, player_state_text) in &mut query_ui {
            for (player_state, player) in &query_state {
                if player_state_text.n == player.n {
                    **text = format!("{:?}", player_state);
                }
            }
        }
    }
}

fn spawn_health_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<&Player, With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];
    let margin = 75.;

    for player in &query {
        let left_position = if player.n == 1 {
            margin
        } else {
            window.width() - margin
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
            HealthText { n: player.n },
        ));
    }
}

fn health_text_update(
    mut query_ui: Query<(&mut Text, &HealthText), With<HealthText>>,
    query_state: Query<(&Health, &Player), With<Player>>,
) {
    for (mut text, health_text) in &mut query_ui {
        for (health, player) in &query_state {
            if health_text.n == player.n {
                **text = format!("{} PV", health.value);
            }
        }
    }
}

fn spawn_bullet_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<&Player, With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];
    let margin = 75.;

    for player in &query {
        let left_position = if player.n == 1 {
            margin
        } else {
            window.width() - margin
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
            BulletText { n: player.n },
        ));
    }
}

fn bullet_text_update(
    mut query_ui: Query<(&mut Text, &BulletText), With<BulletText>>,
    query_state: Query<(&Bullets, &Player), With<Player>>,
) {
    for (mut text, bullet_text) in &mut query_ui {
        for (bullets, player) in &query_state {
            if bullet_text.n == player.n {
                **text = format!("{} bullets", bullets.n);
            }
        }
    }
}

fn spawn_dodge_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<&Player, With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];
    let margin = 75.;

    for player in &query {
        let left_position = if player.n == 1 {
            margin
        } else {
            window.width() - margin
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
            DodgeText { n: player.n },
        ));
    }
}

fn dodge_text_update(
    mut query_ui: Query<(&mut Text, &DodgeText), With<DodgeText>>,
    query_state: Query<(&Dodges, &Player), With<Player>>,
) {
    for (mut text, dodge_text) in &mut query_ui {
        for (dodges, player) in &query_state {
            if dodge_text.n == player.n {
                **text = format!("{} dodges", dodges.n);
            }
        }
    }
}

fn set_timed_play_state(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
    time: Res<Time>,
    mut play_state_timer: ResMut<PlayStateTimer>,
) {
    play_state_timer.0.tick(time.delta());

    if play_state_timer.0.just_finished() {
        next_play_state.set(play_state_transitions(*play_state.get()));
        play_state_timer.0.reset()
    }
}

fn next_play_state(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    next_play_state.set(play_state_transitions(*play_state.get()));
}

fn display_play_state(play_state: Res<State<PlayStates>>) {
    println!("============");
    println!("Game state: {:?}", play_state.get());
    println!("============");
}

fn reset_game_timer(mut play_state_timer: ResMut<PlayStateTimer>) {
    play_state_timer.0.reset()
}

fn listen_endgames(
    mut ev_endgame: EventReader<EndGameEvent>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    for ev in ev_endgame.read() {
        next_play_state.set(PlayStates::Finished);

        match ev.state {
            EndGames::Tie => println!("It's a tie!"),
            EndGames::Winner => println!("Player {} won!", ev.player.unwrap()),
        }
    }
}

// Betting
// ----------------------------------------------------------------

fn set_player_state(
    mut query: Query<(&KeyAssignment, &mut PlayerState, &Player, &Dodges, &Bullets)>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_pressed() {
        for (key_assignements, mut player_state, player, dodges, bullets) in &mut query {
            let requested_state =
                key_to_player_state(&key_assignements.0, key).unwrap_or(player_state.0);

            ev_change_player_state.send(PlayerStateChangeEvent);

            match requested_state {
                PlayerStates::Attacking => {
                    if bullets.n > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotAttacking;
                    }
                }
                PlayerStates::Dodging => {
                    if dodges.n > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotDodging;
                    }
                }
                _ => {}
            }

            println!("Player {:?} is {:?}", player.n, player_state.0)
        }
    }
}

// Fighting
// ----------------------------------------------------------------

fn decrease_health(
    mut query: Query<(&mut Health, &mut PlayerState, &mut Luck, &mut Marksmanship), With<Player>>,
) {
    let mut query_mut = query.iter_combinations_mut();
    while let Some(
        [(mut health_0, state_0, luck_0, marksmanship_0), (mut health_1, state_1, luck_1, marksmanship_1)],
    ) = query_mut.fetch_next()
    {
        match [state_0.0, state_1.0] {
            [PlayerStates::Attacking, PlayerStates::Attacking] => {
                if roll_the_dice(marksmanship_1.n) > roll_the_dice(luck_0.n) {
                    println!("Player 1 shot!");
                    health_0.value -= 1;
                } else {
                    println!("Player 1 missed!");
                }

                if roll_the_dice(marksmanship_0.n) > roll_the_dice(luck_1.n) {
                    println!("Player 2 shot!");
                    health_1.value -= 1;
                } else {
                    println!("Player 2 missed!");
                }
            }
            [PlayerStates::Attacking, PlayerStates::Idle | PlayerStates::NotAttacking | PlayerStates::NotDodging] => {
                if roll_the_dice(marksmanship_0.n) > roll_the_dice(luck_1.n) {
                    println!("Player 2 shot!");
                    health_1.value -= 1;
                } else {
                    println!("Player 2 missed!");
                }
            }
            [PlayerStates::Idle | PlayerStates::NotAttacking | PlayerStates::NotDodging, PlayerStates::Attacking] => {
                if roll_the_dice(marksmanship_1.n) > roll_the_dice(luck_0.n) {
                    println!("Player 1 shot!");
                    health_0.value -= 1;
                } else {
                    println!("Player 1 missed!");
                }
            }
            _ => {
                println!("Nothing happened !")
            } // [PlayerStates::Attacking, PlayerStates::Dodging] => {},
              // [PlayerStates::Dodging, PlayerStates::Attacking] => {},
              // [PlayerStates::Dodging, PlayerStates::Idle] => {},
              // [PlayerStates::Dodging, PlayerStates::Dodging] => {},
              // [PlayerStates::Idle, PlayerStates::Dodging] => {},
              // [PlayerStates::Idle, PlayerStates::Idle] => {},
        }
    }
}

fn decrease_dodges(mut query: Query<(&PlayerState, &mut Dodges), With<Player>>) {
    for (player_state, mut dodges) in &mut query {
        if player_state.0 == PlayerStates::Dodging {
            dodges.n -= 1
        }
    }
}

fn decrease_bullets(mut query: Query<(&PlayerState, &mut Bullets), With<Player>>) {
    for (player_state, mut bullets) in &mut query {
        if player_state.0 == PlayerStates::Attacking {
            bullets.n -= 1
        }
    }
}

// Rounding up
// ----------------------------------------------------------------

fn print_stats(query: Query<(&Player, &Bullets, &Dodges, &Health)>) {
    for (player, bullets, dodges, health) in &query {
        println!(
            "Player {} HP: {} Bullets: {} Dodges: {}",
            player.n, health.value, bullets.n, dodges.n
        );
    }
}

fn prepare_player_for_next_round(
    mut query: Query<&mut PlayerState, With<Player>>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
) {
    for mut player_state in &mut query {
        player_state.0 = PlayerStates::Idle;
        ev_change_player_state.send(PlayerStateChangeEvent);
    }
}

fn check_if_dead(mut ev_endgame: EventWriter<EndGameEvent>, mut query: Query<(&Health, &Player)>) {
    let mut dead = [false, false];

    for (health, player) in &mut query {
        if health.value == 0 {
            let index: usize = (player.n - 1).into();
            dead[index] = true
        }
    }

    match dead {
        [true, true] => {
            println!("Both players shot themselves to death.");
            ev_endgame.send(EndGameEvent {
                player: None,
                state: EndGames::Tie,
            });
        }
        [true, false] => {
            println!("Player 1 is dead.");
            ev_endgame.send(EndGameEvent {
                player: Some(2),
                state: EndGames::Winner,
            });
        }
        [false, true] => {
            println!("Player 2 is dead.");
            ev_endgame.send(EndGameEvent {
                player: Some(1),
                state: EndGames::Winner,
            });
        }
        [false, false] => {
            println!("Both players still alive.");
        }
    }
}

fn check_if_out_of_ammo(
    mut ev_endgame: EventWriter<EndGameEvent>,
    mut query: Query<(&Bullets, &Health, &Player)>,
) {
    let mut query_mut = query.iter_combinations_mut();
    while let Some([(bullets_0, health_0, player_0), (bullets_1, health_1, player_1)]) =
        query_mut.fetch_next()
    {
        let ammo = [bullets_0.n, bullets_1.n];

        if ammo.iter().all(|&x| x == 0) {
            let winner: Option<u8>;

            match health_0.value < health_1.value {
                true => winner = Some(player_1.n),
                false => {
                    if health_0.value == health_1.value {
                        winner = None;
                    } else {
                        winner = Some(player_0.n)
                    }
                }
            }

            if winner.is_some() {
                println!("Player {} has highest score!", winner.unwrap());

                ev_endgame.send(EndGameEvent {
                    player: winner,
                    state: EndGames::Winner,
                });
            } else {
                println!("Both players have same score.");

                ev_endgame.send(EndGameEvent {
                    player: None,
                    state: EndGames::Tie,
                });
            }
        }
    }
}

fn check_if_last_round(
    mut ev_endgame: EventWriter<EndGameEvent>,
    mut query: Query<(&Health, &Player)>,
    round: Res<RoundCounter>,
) {
    if round.0 == N_MAX_ROUND {
        let mut winner: Option<u8>;

        let mut query_mut = query.iter_combinations_mut();
        while let Some([(health_0, player_0), (health_1, player_1)]) = query_mut.fetch_next() {
            match health_0.value < health_1.value {
                true => winner = Some(player_1.n),
                false => {
                    if health_0.value == health_1.value {
                        winner = None;
                    } else {
                        winner = Some(player_0.n)
                    }
                }
            }

            if winner.is_some() {
                println!("Player {} has highest score!", winner.unwrap());

                ev_endgame.send(EndGameEvent {
                    player: winner,
                    state: EndGames::Winner,
                });
            } else {
                println!("Both players have same score.");

                ev_endgame.send(EndGameEvent {
                    player: None,
                    state: EndGames::Tie,
                });
            }
        }
    }
}

fn restore_dodge(round_counter: Res<RoundCounter>, mut query: Query<&mut Dodges, With<Player>>) {
    if round_counter.0 % 2 == 0 {
        for mut dodges in &mut query {
            dodges.n += 1;
            println!("1 dodge acquired!");
        }
    }
}

fn restore_bullet(round_counter: Res<RoundCounter>, mut query: Query<&mut Bullets, With<Player>>) {
    if round_counter.0 % 2 == 0 {
        for mut bullets in &mut query {
            bullets.n += 1;
            println!("1 bullet acquired!");
        }
    }
}

fn increase_round_counter(mut round_counter: ResMut<RoundCounter>) {
    round_counter.0 += 1;
}

// Helpers
// ================================================================

fn play_state_transitions(play_state: PlayStates) -> PlayStates {
    match play_state {
        PlayStates::Preparing => PlayStates::Betting,
        PlayStates::Betting => PlayStates::Fighting,
        PlayStates::Fighting => PlayStates::RoundingUp,
        PlayStates::RoundingUp => PlayStates::Preparing,
        _ => play_state,
    }
}

fn key_to_player_state(
    key_assignments: &[KeyCode; N_KEYS_PER_PLAYER],
    key: &KeyCode,
) -> Option<PlayerStates> {
    if *key == key_assignments[0] {
        Some(PlayerStates::Attacking)
    } else if *key == key_assignments[1] {
        Some(PlayerStates::Dodging)
    } else {
        None
    }
}

fn roll_the_dice(dice_max: u8) -> u8 {
    let mut rng = rand::rng();
    let nums: Vec<u8> = (1..dice_max).collect();

    *nums.choose(&mut rng).unwrap()
}

mod tests {
    #[cfg(test)]
    use crate::hit_a_key::*;

    #[test]
    fn test_roll_the_dice() {
        let possible_values: [u8; 3] = [1, 2, 3];
        assert!(possible_values.contains(&roll_the_dice(3)));
    }
}
