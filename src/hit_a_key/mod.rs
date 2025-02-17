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
pub struct Player {
    n: u8,
}

#[derive(Component)]
pub struct Health {
    value: u8,
}

#[derive(Component)]
pub struct Luck {
    n: u8,
}

#[derive(Component)]
pub struct Marksmanship {
    n: u8,
}

#[derive(Component)]
pub struct Dodges {
    n: u8,
}

#[derive(Component)]
pub struct Bullets {
    n: u8,
}

#[derive(Component)]
pub struct PlayerState(PlayerStates);

#[derive(Component)]
pub struct KeyAssignment([KeyCode; N_KEYS_PER_PLAYER]);

// Enums
// ================================================================

#[derive(Debug, Clone, Copy)]
pub enum PlayerStates {
    Idle,
    Attacking,
    NotAttacking,
    Dodging,
    NotDodging
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStates {
    Menu,
    Loading,
    // Paused,
    Betting,
    Fighting,
    RoundingUp
}

// Resources
// ================================================================
#[derive(Resource)]
struct GameState(GameStates);

#[derive(Resource)]
struct GameStateTimer(Timer);

#[derive(Resource)]
struct RoundCounter(u8);


// Game
// ================================================================

// TODO:
// - add buffes
// - refactor with events
// - Graphics & anims !!

impl Plugin for HitAKeyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState(GameStates::Betting));
        app.insert_resource(GameStateTimer(Timer::from_seconds(8.0, TimerMode::Once)));
        app.insert_resource(RoundCounter(1));
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            set_game_state,
            set_player_state.run_if(game_state_is_betting),
            (decrease_health, next_phase).chain().run_if(game_state_is_fighting),
            print_stats.run_if(game_state_is_rounding_up),
            (
                check_if_dead.run_if(game_state_is_rounding_up),
                check_if_out_of_ammo.run_if(game_state_is_rounding_up),
                restore_bullet.run_if(game_state_is_rounding_up),
                restore_dodge.run_if(game_state_is_rounding_up),
                increase_round_counter.run_if(game_state_is_rounding_up),
            ).chain()
        ).chain());
    }
}

fn set_game_state(mut game_state: ResMut<GameState>, time: Res<Time>, mut game_state_timer: ResMut<GameStateTimer> ) {
    game_state_timer.0.tick(time.delta());

    if game_state_timer.0.just_finished() {
        game_state.0 = game_state_transitions(game_state.0);
        println!("Game state: {:?}", game_state.0);
        game_state_timer.0.reset()
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

fn next_phase(
    mut game_state_timer: ResMut<GameStateTimer>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut PlayerState, &mut Dodges, &mut Bullets), With<Player>>
) {
    for (mut player_state, mut dodges, mut bullets) in  &mut query {
        match player_state.0 {
            PlayerStates::Dodging => dodges.n -= 1,
            PlayerStates::Attacking => bullets.n -= 1,
            _ => {}
        }
        player_state.0 = PlayerStates::Idle
    }

    game_state.0 = game_state_transitions(game_state.0);
    println!("Game state: {:?}", game_state.0);
    game_state_timer.0.reset()
}

// Rounding up
// ----------------------------------------------------------------

fn print_stats(query: Query<(&Player, &Bullets, &Dodges, &Health)>) {
    for (player, bullets, dodges, health) in &query {
        println!("Player {} HP: {} Bullets: {} Dodges: {}", player.n, health.value, bullets.n, dodges.n);
    }
}


fn check_if_dead(mut game_state: ResMut<GameState>, mut query: Query<(&Health, &Player)>) {
    let mut dead= [false, false];

    for (health, player) in &mut query {
        if health.value == 0  {
            let index: usize = (player.n - 1).into();
            dead[index] = true
        }
    }

    match dead {
        [true, true] => {
            println!("Both players shot themselves to death. It's a tie!");
            game_state.0 = GameStates::Menu;
        },
        [true, false] => {
            println!("Player 1 is dead. Player 2 wins!");
            game_state.0 = GameStates::Menu;
        },
        [false, true] => {
            println!("Player 2 is dead. Player 1 wins!");
            game_state.0 = GameStates::Menu;
        },
        [false, false] => {
            println!("Both players still alive. Prepare for next round!");
        }
    }
}

fn check_if_out_of_ammo(mut game_state: ResMut<GameState>, mut query: Query<(&Bullets, &Player)>) {
    let mut out_of_ammo= [false, false];

    for (bullets, player) in &mut query {
        if bullets.n == 0  {
            let index: usize = (player.n - 1).into();
            out_of_ammo[index] = true
        }
    }

    if out_of_ammo.iter().all(|&x| x == true) {
        println!("Both players are out of ammo. It's a tie!");
        game_state.0 = GameStates::Menu;
    }
}

fn restore_dodge(round_counter: Res<RoundCounter>, mut query: Query<&mut Dodges, With<Player>>) {
    if round_counter.0 > 2 && round_counter.0 % 2 == 0 {
        for mut dodges in &mut query {
            dodges.n += 1;
            println!("1 dodge acquired!");
        }
    }
}

fn restore_bullet(round_counter: Res<RoundCounter>, mut query: Query<&mut Bullets, With<Player>>) {
    if round_counter.0 > 2 && round_counter.0 % 2 == 0 {
        for mut bullets in &mut query {
            bullets.n += 1;
            println!("1 bullet acquired!");
        }
    }
}

fn increase_round_counter(mut round_counter: ResMut<RoundCounter>, mut game_state: ResMut<GameState>) {
    round_counter.0 += 1;
    game_state.0 = GameStates::Loading;
}


// Helpers
// ================================================================

fn game_state_transitions(game_state: GameStates) -> GameStates {
    match game_state {
        GameStates::Betting => GameStates::Fighting,
        GameStates::Fighting => GameStates::RoundingUp,
        GameStates::RoundingUp => GameStates::Loading,
        GameStates::Loading => GameStates::Betting,
        _ => game_state
    }
}

fn game_state_is_betting(game_state: Res<GameState>) -> bool {
    game_state.0 == GameStates::Betting
}

fn game_state_is_rounding_up(game_state: Res<GameState>) -> bool {
    game_state.0 == GameStates::RoundingUp
}

fn game_state_is_fighting(game_state: Res<GameState>) -> bool {
    game_state.0 == GameStates::Fighting
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