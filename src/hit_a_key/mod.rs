use bevy::prelude::*;
use rand::{distr::StandardUniform, prelude::*};

pub struct HitAKeyPlugin;

// Settings
// ================================================================

const N_KEYS_PER_PLAYER: usize = 3;
const PLAYER_KEY_ASSIGNMENTS: [[KeyCode; N_KEYS_PER_PLAYER]; 2] = [
    [KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyF],
    [KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL],
];
const PLAYER_ONE_KEYS: [KeyCode; N_KEYS_PER_PLAYER] = PLAYER_KEY_ASSIGNMENTS[0];
const PLAYER_TWO_KEYS: [KeyCode; N_KEYS_PER_PLAYER] = PLAYER_KEY_ASSIGNMENTS[1];
const N_BULLETS: u8 = 3;
const N_DODGES: u8 = 3;
const N_FACETED_DICE: u8 = 100;
const N_MAX_ROUND: u8 = 8;
const DEFAULT_LUCK: u8 = N_FACETED_DICE - (N_FACETED_DICE as f32 * 0.1) as u8;
const DEFAULT_DAMAGE: u8 = 1;
const DEFAULT_HEALTH: u8 = 5;

// UI_DEFAULTS
// ================================================================

const DEFAULT_MARGIN: f32 = 75.;

// Components
// ================================================================

#[derive(Component)]
struct Player {
    value: u8,
}

#[derive(Component)]
struct Health {
    value: u8,
}

#[derive(Component)]
struct Luck {
    value: u8,
}

#[derive(Component)]
struct Marksmanship {
    value: u8,
}

#[derive(Component)]
struct Dodges {
    value: u8,
}

#[derive(Component)]
struct Bullets {
    value: u8,
}

#[derive(Component)]
struct Damage {
    value: u8,
}

#[derive(Component, Clone, Copy)]
struct Buff {
    value: Option<Buffes>,
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
    value: u8,
}

#[derive(Component)]
struct HealthText {
    value: u8,
}

#[derive(Component)]
struct BulletText {
    value: u8,
}

#[derive(Component)]
struct DodgeText {
    value: u8,
}

#[derive(Component)]
struct TimerUIText;

#[derive(Component)]

struct PressSpacebarText;

#[derive(Component)]

struct PlayerTickText;

#[derive(Component)]

struct AlertText;

// Events
// ================================================================

#[derive(Event)]
struct GameOverEvent {
    player: Option<u8>,
    state: GameOvers,
}

#[derive(Event)]
struct PlayerStateChangeEvent;

#[derive(Event)]
struct DamageEvent {
    player: u8,
    value: u8,
}

#[derive(Event)]
struct MissedEvent {
    player: u8,
}

#[derive(Event)]
struct TickPlayerEvent {
    player: u8,
    value: String,
}

#[derive(Event)]
struct AlertEvent {
    value: String,
}

// Enums
// ================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerStates {
    Idle,
    Attacking,
    NotAttacking,
    Dodging,
    NotDodging,
    Buffing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameOvers {
    Tie,
    Winner,
}

#[derive(PartialEq, Clone, Copy)]
enum Buffes {
    DoubleDamageBuff,
    HealBuff,
}

impl Distribution<Buffes> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Buffes {
        let index = rng.random_range(0..1);
        match index {
            0 => Buffes::DoubleDamageBuff,
            1 => Buffes::HealBuff,
            _ => unreachable!(),
        }
    }
}

// States
// ================================================================

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlayStates {
    Preparing,
    Countdown,
    Betting,
    Fighting,
    RoundingUp,
    GameOver,
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
struct BettingTimer(Timer);

#[derive(Resource)]
struct CountdownTimer(Timer);

#[derive(Resource)]
struct RoundCounter(u8);

#[derive(Resource)]
struct GameOver(bool);

// Game
// ================================================================

impl Plugin for HitAKeyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayStates::Betting);

        app.insert_resource(BettingTimer(Timer::from_seconds(8.0, TimerMode::Once)));
        app.insert_resource(CountdownTimer(Timer::from_seconds(3.0, TimerMode::Once)));
        app.insert_resource(RoundCounter(1));
        app.insert_resource(GameOver(false));

        app.add_event::<GameOverEvent>();
        app.add_event::<PlayerStateChangeEvent>();
        app.add_event::<DamageEvent>();
        app.add_event::<MissedEvent>();
        app.add_event::<TickPlayerEvent>();
        app.add_event::<AlertEvent>();

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
            OnEnter(PlayStates::Preparing),
            (check_if_last_round, spawn_press_spacebar_ui),
        );
        app.add_systems(
            OnExit(PlayStates::Preparing),
            (despawn_press_spacebar_ui, despawn_alert_text),
        );

        app.add_systems(
            OnEnter(PlayStates::Countdown),
            (reset_countdown_timer, spawn_timer_ui).chain(),
        );

        app.add_systems(OnExit(PlayStates::Countdown), despawn_timer_ui);

        app.add_systems(
            OnEnter(PlayStates::Betting),
            (reset_betting_timer, spawn_timer_ui).chain(),
        );

        app.add_systems(
            OnExit(PlayStates::Betting),
            (
                despawn_timer_ui,
                (add_buffes, (double_damage_buff, heal_buff)).chain(),
            ),
        );

        app.add_systems(
            OnEnter(PlayStates::Fighting),
            (fight, decrease_bullets, decrease_dodges),
        );

        app.add_systems(
            OnExit(PlayStates::Fighting),
            (remove_buffes, despawn_player_tick_ui),
        );

        app.add_systems(
            OnEnter(PlayStates::RoundingUp),
            (
                check_if_dead,
                check_if_no_more_rounds.run_if(is_not_game_over),
                check_if_out_of_ammo.run_if(is_not_game_over),
            )
                .chain(),
        );

        app.add_systems(
            OnExit(PlayStates::RoundingUp),
            (
                prepare_player_for_next_round,
                restore_bullet,
                restore_dodge,
                increase_round_counter,
            )
                .run_if(is_not_game_over),
        );

        app.add_systems(
            Update,
            (
                listen_game_overs,
                listen_spawn_alert_text,
                player_state_text_update,
                round_number_text_update,
                play_state_text_update,
                health_text_update,
                bullet_text_update,
                dodge_text_update,
                wait_for_input_to_next_play_state.run_if(in_state(PlayStates::Preparing)),
                (countdown, update_countdown_timer_ui)
                    .chain()
                    .run_if(in_state(PlayStates::Countdown)),
                (betting_countdown, set_player_state, update_betting_timer_ui)
                    .run_if(in_state(PlayStates::Betting)),
                (
                    listen_damage_event,
                    listen_missed_event,
                    listen_spawn_player_tick_ui,
                    animate_player_tick_text_opacity,
                    animate_player_tick_font_size,
                    (next_play_state).run_if(check_fighting_phase_ended),
                )
                    .run_if(in_state(PlayStates::Fighting))
                    .chain(),
                next_play_state
                    .run_if(in_state(PlayStates::RoundingUp))
                    .run_if(is_not_game_over),
            ),
        );
    }
}

// Systems
// ================================================================

fn spawn_timer_ui(mut commands: Commands, window_query: Query<&Window>) {
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
    ));
}

fn despawn_timer_ui(mut commands: Commands, query: Query<Entity, With<TimerUIText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
}

fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player { value: 1 },
        Health {
            value: DEFAULT_HEALTH,
        },
        Luck {
            value: DEFAULT_LUCK,
        },
        Damage {
            value: DEFAULT_DAMAGE,
        },
        Marksmanship {
            value: N_FACETED_DICE,
        },
        Dodges { value: N_DODGES },
        Bullets { value: N_BULLETS },
        Buff { value: None },
        KeyAssignment(PLAYER_ONE_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(255., 0., 0.))),
        Transform::from_xyz(-250., 0.0, 0.0),
    ));

    commands.spawn((
        Player { value: 2 },
        Health {
            value: DEFAULT_HEALTH,
        },
        Luck {
            value: DEFAULT_LUCK,
        },
        Damage {
            value: DEFAULT_DAMAGE,
        },
        Marksmanship {
            value: N_FACETED_DICE,
        },
        Dodges { value: N_DODGES },
        Bullets { value: N_BULLETS },
        Buff { value: None },
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
            PlayerStateText {
                value: player.value,
            },
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
                if player_state_text.value == player.value {
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
        ));
    }
}

fn health_text_update(
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

fn spawn_bullet_text(
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
        ));
    }
}

fn bullet_text_update(
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

fn spawn_dodge_text(
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
        ));
    }
}

fn dodge_text_update(
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

fn next_play_state(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    next_play_state.set(play_state_transitions(*play_state.get()));
}

fn wait_for_input_to_next_play_state(
    keys: Res<ButtonInput<KeyCode>>,
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
) {
    for key in keys.get_pressed() {
        if *key == KeyCode::Space {
            next_play_state.set(play_state_transitions(*play_state.get()));
        }
    }
}

fn listen_game_overs(
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
                font_size: 100.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            AlertText,
        ));
    }
}

fn despawn_alert_text(mut commands: Commands, query: Query<Entity, With<AlertText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// Preparing
// ----------------------------------------------------------------

fn spawn_press_spacebar_ui(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let dimensions = [500., 50.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            bottom: Val::Px(DEFAULT_MARGIN * 2. - (dimensions[1] / 2.)),
            left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
            ..default()
        },
        Text::new("Press space bar"),
        TextFont {
            font_size: 30.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        PressSpacebarText,
    ));
}

fn despawn_press_spacebar_ui(
    mut commands: Commands,
    query: Query<Entity, With<PressSpacebarText>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn check_if_last_round(round: Res<RoundCounter>, mut ev_last_round: EventWriter<AlertEvent>) {
    if round.0 == N_MAX_ROUND {
        ev_last_round.send(AlertEvent {
            value: "Last round!".into(),
        });
    }
}

// Countdown
// ----------------------------------------------------------------

fn countdown(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
    time: Res<Time>,
    mut countdown_timer: ResMut<CountdownTimer>,
) {
    countdown_timer.0.tick(time.delta());

    if countdown_timer.0.just_finished() {
        next_play_state.set(play_state_transitions(*play_state.get()));
        countdown_timer.0.reset()
    }
}

fn reset_countdown_timer(mut countdown_timer: ResMut<CountdownTimer>) {
    countdown_timer.0.reset()
}

// UI
fn update_countdown_timer_ui(
    betting_timer: Res<CountdownTimer>,
    mut query: Query<&mut Text, With<TimerUIText>>,
) {
    for mut text in &mut query {
        **text = format!("{}", betting_timer.0.remaining_secs().round());
    }
}

// Betting
// ----------------------------------------------------------------

fn betting_countdown(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
    time: Res<Time>,
    mut betting_timer: ResMut<BettingTimer>,
) {
    betting_timer.0.tick(time.delta());

    if betting_timer.0.just_finished() {
        next_play_state.set(play_state_transitions(*play_state.get()));
    }
}

fn reset_betting_timer(mut betting_timer: ResMut<BettingTimer>) {
    betting_timer.0.reset()
}

fn set_player_state(
    mut query: Query<(&KeyAssignment, &mut PlayerState, &Dodges, &Bullets)>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_pressed() {
        for (key_assignements, mut player_state, dodges, bullets) in &mut query {
            let requested_state =
                key_to_player_state(&key_assignements.0, key).unwrap_or(player_state.0);

            ev_change_player_state.send(PlayerStateChangeEvent);

            match requested_state {
                PlayerStates::Attacking => {
                    if bullets.value > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotAttacking;
                    }
                }
                PlayerStates::Dodging => {
                    if dodges.value > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotDodging;
                    }
                }
                other_player_state => player_state.0 = other_player_state,
            }
        }
    }
}

fn add_buffes(mut query: Query<(&mut Buff, &PlayerState), With<Player>>) {
    for (mut buff, player_state) in &mut query {
        if player_state.0 == PlayerStates::Buffing {
            let random_buff: Buffes = rand::random();
            buff.value = Some(random_buff);
        }
    }
}

fn remove_buffes(mut query: Query<&mut Buff, With<Player>>) {
    for mut buff in &mut query {
        buff.value = None;
    }
}

fn double_damage_buff(mut query: Query<(&Buff, &mut Damage), With<Player>>) {
    for (&buff, mut damage) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::DoubleDamageBuff {
                damage.value *= 2;
            }
        }
    }
}

fn heal_buff(mut query: Query<(&Buff, &mut Health), With<Player>>) {
    for (&buff, mut health) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::HealBuff {
                health.value += 1;
            }
        }
    }
}

// UI
fn update_betting_timer_ui(
    betting_timer: Res<BettingTimer>,
    mut query: Query<&mut Text, With<TimerUIText>>,
) {
    for mut text in &mut query {
        **text = format!("{}", betting_timer.0.remaining_secs().round());
    }
}

// Fighting
// ----------------------------------------------------------------

fn fight(
    query: Query<(&PlayerState, &Luck, &Marksmanship, &Damage, &Player), With<Player>>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_missed: EventWriter<MissedEvent>,
) {
    let mut query_mut = query.iter_combinations();
    while let Some(
        [(state_0, luck_0, marksmanship_0, damage_0, player_0), (state_1, luck_1, marksmanship_1, damage_1, player_1)],
    ) = query_mut.fetch_next()
    {
        match [state_0.0, state_1.0] {
            [PlayerStates::Attacking, PlayerStates::Attacking] => {
                if roll_the_dice(marksmanship_1.value) > roll_the_dice(luck_0.value) {
                    ev_damage.send(DamageEvent {
                        player: player_0.value,
                        value: damage_1.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_0.value,
                    });
                }

                if roll_the_dice(marksmanship_0.value) > roll_the_dice(luck_1.value) {
                    ev_damage.send(DamageEvent {
                        player: player_1.value,
                        value: damage_0.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_1.value,
                    });
                }
            }
            [PlayerStates::Attacking, PlayerStates::Idle
            | PlayerStates::NotAttacking
            | PlayerStates::NotDodging
            | PlayerStates::Buffing] => {
                if roll_the_dice(marksmanship_0.value) > roll_the_dice(luck_1.value) {
                    ev_damage.send(DamageEvent {
                        player: player_1.value,
                        value: damage_0.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_1.value,
                    });
                }
            }
            [PlayerStates::Idle
            | PlayerStates::NotAttacking
            | PlayerStates::NotDodging
            | PlayerStates::Buffing, PlayerStates::Attacking] => {
                if roll_the_dice(marksmanship_1.value) > roll_the_dice(luck_0.value) {
                    ev_damage.send(DamageEvent {
                        player: player_0.value,
                        value: damage_1.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_0.value,
                    });
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

fn listen_damage_event(
    mut ev_damage: EventReader<DamageEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    mut query: Query<(&mut Health, &Player), With<Player>>,
) {
    for ev in ev_damage.read() {
        for (mut health, player) in &mut query {
            if player.value == ev.player {
                health.value = health.value.checked_sub(ev.value).unwrap_or(0);
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: format!("-{}", ev.value),
                });
            }
        }
    }
}

fn listen_missed_event(
    mut ev_missed: EventReader<MissedEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    query: Query<&Player>,
) {
    for ev in ev_missed.read() {
        for player in &query {
            if player.value == ev.player {
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "Missed!".into(),
                });
            }
        }
    }
}

fn listen_spawn_player_tick_ui(
    mut commands: Commands,
    mut ev_tick_player: EventReader<TickPlayerEvent>,
    query: Query<(&Player, &Transform), With<Player>>,
    window_query: Query<&Window>,
) {
    for ev in ev_tick_player.read() {
        let window = window_query.single();
        let dimensions = [250., 250.];

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

fn decrease_dodges(mut query: Query<(&PlayerState, &mut Dodges), With<Player>>) {
    for (player_state, mut dodges) in &mut query {
        if player_state.0 == PlayerStates::Dodging {
            dodges.value -= 1
        }
    }
}

fn decrease_bullets(mut query: Query<(&PlayerState, &mut Bullets), With<Player>>) {
    for (player_state, mut bullets) in &mut query {
        if player_state.0 == PlayerStates::Attacking {
            bullets.value -= 1
        }
    }
}

fn check_fighting_phase_ended(query: Query<&TextColor, With<PlayerTickText>>) -> bool {
    let mut conditions: Vec<bool> = vec![];

    for color in &query {
        conditions.push(color.0.alpha() <= 0.);
    }

    conditions.iter().all(|condition| *condition)
}
// Rounding up
// ----------------------------------------------------------------

fn prepare_player_for_next_round(
    mut query: Query<&mut PlayerState, With<Player>>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
) {
    for mut player_state in &mut query {
        player_state.0 = PlayerStates::Idle;
        ev_change_player_state.send(PlayerStateChangeEvent);
    }
}

fn is_not_game_over(game_over: Res<GameOver>) -> bool {
    !game_over.0
}

fn check_if_dead(
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut query: Query<(&Health, &Player)>,
    mut game_over: ResMut<GameOver>,
) {
    let mut dead = [false, false];

    for (health, player) in &mut query {
        if health.value == 0 {
            let index: usize = (player.value - 1).into();
            dead[index] = true
        }
    }

    match dead {
        [true, true] => {
            println!("Both players shot themselves to death.");
            ev_game_over.send(GameOverEvent {
                player: None,
                state: GameOvers::Tie,
            });
            game_over.0 = true;
        }
        [true, false] => {
            println!("Player 1 is dead.");
            ev_game_over.send(GameOverEvent {
                player: Some(2),
                state: GameOvers::Winner,
            });
            game_over.0 = true;
        }
        [false, true] => {
            println!("Player 2 is dead.");
            ev_game_over.send(GameOverEvent {
                player: Some(1),
                state: GameOvers::Winner,
            });
            game_over.0 = true;
        }
        [false, false] => {
            println!("Both players still alive.");
        }
    }
}

fn check_if_out_of_ammo(
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut query: Query<(&Bullets, &Health, &Player)>,
    mut game_over: ResMut<GameOver>,
) {
    let mut query_mut = query.iter_combinations_mut();
    while let Some([(bullets_0, health_0, player_0), (bullets_1, health_1, player_1)]) =
        query_mut.fetch_next()
    {
        let ammo = [bullets_0.value, bullets_1.value];

        if ammo.iter().all(|&x| x == 0) {
            game_over.0 = true;

            let winner: Option<u8>;

            match health_0.value < health_1.value {
                true => winner = Some(player_1.value),
                false => {
                    if health_0.value == health_1.value {
                        winner = None;
                    } else {
                        winner = Some(player_0.value)
                    }
                }
            }

            if winner.is_some() {
                println!("Player {} has highest score!", winner.unwrap());

                ev_game_over.send(GameOverEvent {
                    player: winner,
                    state: GameOvers::Winner,
                });
            } else {
                println!("Both players have same score.");

                ev_game_over.send(GameOverEvent {
                    player: None,
                    state: GameOvers::Tie,
                });
            }
        }
    }
}

fn check_if_no_more_rounds(
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut query: Query<(&Health, &Player)>,
    mut game_over: ResMut<GameOver>,
    round: Res<RoundCounter>,
) {
    if round.0 == N_MAX_ROUND {
        game_over.0 = true;

        let mut winner: Option<u8>;

        let mut query_mut = query.iter_combinations_mut();
        while let Some([(health_0, player_0), (health_1, player_1)]) = query_mut.fetch_next() {
            match health_0.value < health_1.value {
                true => winner = Some(player_1.value),
                false => {
                    if health_0.value == health_1.value {
                        winner = None;
                    } else {
                        winner = Some(player_0.value)
                    }
                }
            }

            if winner.is_some() {
                println!("Player {} has highest score!", winner.unwrap());

                ev_game_over.send(GameOverEvent {
                    player: winner,
                    state: GameOvers::Winner,
                });
            } else {
                println!("Both players have same score.");

                ev_game_over.send(GameOverEvent {
                    player: None,
                    state: GameOvers::Tie,
                });
            }
        }
    }
}

fn restore_dodge(
    round_counter: Res<RoundCounter>,
    mut query: Query<(&mut Dodges, &Luck), With<Player>>,
) {
    if round_counter.0 % 2 == 0 {
        for (mut dodges, luck) in &mut query {
            if roll_the_dice(N_FACETED_DICE) <= luck.value {
                dodges.value += 1;
                println!("1 dodge acquired!");
            } else {
                println!("Failed to restore a dodge!");
            }
        }
    }
}

fn restore_bullet(
    round_counter: Res<RoundCounter>,
    mut query: Query<(&mut Bullets, &Luck), With<Player>>,
) {
    if round_counter.0 % 2 == 0 {
        for (mut bullets, luck) in &mut query {
            if roll_the_dice(N_FACETED_DICE) <= luck.value {
                bullets.value += 1;
                println!("1 bullet acquired!");
            } else {
                println!("Failed to restore a bullet!");
            }
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
        PlayStates::Preparing => PlayStates::Countdown,
        PlayStates::Countdown => PlayStates::Betting,
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
    } else if *key == key_assignments[2] {
        Some(PlayerStates::Buffing)
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
