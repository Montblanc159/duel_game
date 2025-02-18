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

#[derive(Component)]
struct PlayerState(PlayerStates);

#[derive(Component)]
struct KeyAssignment([KeyCode; N_KEYS_PER_PLAYER]);

// Events
// ================================================================

#[derive(Event)]
struct EndGameEvent {
    player: Option<u8>,
    state: EndGames
}

// Enums
// ================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerStates {
    Idle,
    Attacking,
    NotAttacking,
    Dodging,
    NotDodging
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
    Finished
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

// DOING:
// - refactoring winning/tie mechanics

// TODO:
// - add buffes
// - add state
// - refactor with events
// - Graphics & anims !!

impl Plugin for HitAKeyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayStates::Betting);
        app.insert_resource(PlayStateTimer(Timer::from_seconds(8.0, TimerMode::Once)));
        app.insert_resource(RoundCounter(1));
        app.add_event::<EndGameEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            listen_endgames,
            set_timed_play_state,
            set_player_state.run_if(in_state(PlayStates::Betting)),
            (
                display_play_state,
                decrease_health,
                (decrease_bullets, decrease_dodges),
                reset_game_timer,
                next_play_state
            ).chain().run_if(in_state(PlayStates::Fighting)),
            (
                (
                    display_play_state,
                    print_stats,
                    check_if_dead,
                ).chain().run_if(in_state(PlayStates::RoundingUp)),
                check_if_out_of_ammo.run_if(in_state(PlayStates::RoundingUp)),
                (
                    prepare_player_for_next_round,
                    restore_bullet,
                    restore_dodge,
                    increase_round_counter,
                    reset_game_timer,
                    next_play_state
                ).chain().run_if(in_state(PlayStates::RoundingUp))
            ).chain()
        ).chain());
    }
}

// Systems
// ================================================================

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player { n: 1 },
        Health { value: 3 },
        Luck { n: N_FACETED_DICE - 10 },
        Marksmanship { n: N_FACETED_DICE },
        Dodges { n: N_DODGES },
        Bullets { n: N_BULLETS},
        KeyAssignment(PLAYER_ONE_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(255., 0., 0.))),
        Transform::from_xyz(250., 0.0, 0.0),
    ));

    commands.spawn((
        Player { n: 2 },
        Health { value: 3 },
        Luck { n: N_FACETED_DICE - 10 },
        Marksmanship { n: N_FACETED_DICE },
        Dodges { n: N_DODGES },
        Bullets { n: N_BULLETS},
        KeyAssignment(PLAYER_TWO_KEYS),
        PlayerState(PlayerStates::Idle),
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 255.))),
        Transform::from_xyz(-250., 0.0, 0.0),
    ));
}

fn set_timed_play_state(play_state: Res<State<PlayStates>>, mut next_play_state: ResMut<NextState<PlayStates>>, time: Res<Time>, mut play_state_timer: ResMut<PlayStateTimer>) {
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
    mut next_play_state: ResMut<NextState<PlayStates>>
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
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_pressed() {
        for (
            key_assignements,
            mut player_state,
            player,
            dodges,
            bullets
        ) in &mut query {
            let requested_state = key_to_player_state(&key_assignements.0, key).unwrap_or(player_state.0);

            match requested_state {
                PlayerStates::Attacking => {
                    if bullets.n > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotAttacking;
                    }
                },
                PlayerStates::Dodging => {
                    if dodges.n > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotDodging;
                    }
                },
                _ => {},
            }

            println!("Player {:?} is {:?}", player.n, player_state.0)
        }
    }
}

// Fighting
// ----------------------------------------------------------------

fn decrease_health(
    mut query: Query<(
        &mut Health,
        &mut PlayerState,
        &mut Luck,
        &mut Marksmanship),
        With<Player>>
) {
    let mut query_mut = query.iter_combinations_mut();
    while let Some([(
        mut health_0,
        state_0,
        luck_0,
        marksmanship_0,
    ), (
        mut health_1,
        state_1,
        luck_1,
        marksmanship_1,
    )]) = query_mut.fetch_next() {
        match [state_0.0, state_1.0] {
            [PlayerStates::Attacking, PlayerStates::Attacking] => {
                if roll_the_dice(marksmanship_1.n) > roll_the_dice(luck_0.n) {
                    println!("Player 1 shot!");
                    health_0.value -= 1;
                } else {
                    println!("Player 2 misses!");
                }

                if roll_the_dice(marksmanship_0.n) > roll_the_dice(luck_1.n) {
                    println!("Player 2 shot!");
                    health_1.value -= 1;
                } else {
                    println!("Player 1 misses!");
                }
            },
            [PlayerStates::Attacking, PlayerStates::Idle | PlayerStates::NotAttacking | PlayerStates::NotDodging] => {
                if roll_the_dice(marksmanship_0.n) > roll_the_dice(luck_1.n) {
                    println!("Player 2 shot!");
                    health_1.value -= 1;
                } else {
                    println!("Player 1 misses!");
                }
            },
            [PlayerStates::Idle | PlayerStates::NotAttacking | PlayerStates::NotDodging, PlayerStates::Attacking] => {
                if roll_the_dice(marksmanship_1.n) > roll_the_dice(luck_0.n) {
                    println!("Player 1 shot!");
                    health_0.value -= 1;
                } else {
                    println!("Player 2 misses!");
                }
            },
            _ => { println!("Nothing happened !") },
            // [PlayerStates::Attacking, PlayerStates::Dodging] => {},
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
        if player_state.0 == PlayerStates::Dodging { dodges.n -= 1 }
    }
}

fn decrease_bullets(mut query: Query<(&PlayerState, &mut Bullets), With<Player>>) {
    for (player_state, mut bullets) in &mut query {
        if player_state.0 == PlayerStates::Attacking { bullets.n -= 1 }
    }
}


// Rounding up
// ----------------------------------------------------------------

fn print_stats(query: Query<(&Player, &Bullets, &Dodges, &Health)>) {
    for (player, bullets, dodges, health) in &query {
        println!("Player {} HP: {} Bullets: {} Dodges: {}", player.n, health.value, bullets.n, dodges.n);
    }
}

fn prepare_player_for_next_round(mut query: Query<&mut PlayerState, With<Player>>) {
    for mut player_state in  &mut query {
        player_state.0 = PlayerStates::Idle
    }
}

fn check_if_dead(mut ev_endgame: EventWriter<EndGameEvent>, mut query: Query<(&Health, &Player)>) {
    let mut dead= [false, false];

    for (health, player) in &mut query {
        if health.value == 0  {
            let index: usize = (player.n - 1).into();
            dead[index] = true
        }
    }

    match dead {
        [true, true] => {
            println!("Both players shot themselves to death.");
            ev_endgame.send(EndGameEvent { player: None, state: EndGames::Tie });
        },
        [true, false] => {
            println!("Player 1 is dead.");
            ev_endgame.send(EndGameEvent { player: Some(2), state: EndGames::Winner });
        },
        [false, true] => {
            println!("Player 2 is dead.");
            ev_endgame.send(EndGameEvent { player: Some(1), state: EndGames::Winner });
        },
        [false, false] => {
            println!("Both players still alive.");
        }
    }
}

fn check_if_out_of_ammo(mut ev_endgame: EventWriter<EndGameEvent>, mut query: Query<(&Bullets, &Player)>) {
    let mut out_of_ammo= [false, false];

    for (bullets, player) in &mut query {
        if bullets.n == 0  {
            let index: usize = (player.n - 1).into();
            out_of_ammo[index] = true
        }
    }

    if out_of_ammo.iter().all(|&x| x == true) {
        ev_endgame.send(EndGameEvent { player: None, state: EndGames::Tie });
        println!("Both players are out of ammo.");
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
        _ => play_state
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