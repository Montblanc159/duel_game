use super::*;

pub const N_KEYS_PER_PLAYER: usize = 3;
pub const PLAYER_KEY_ASSIGNMENTS: [[KeyCode; N_KEYS_PER_PLAYER]; 2] = [
    [KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyF],
    [KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL],
];
pub const PLAYER_ONE_KEYS: [KeyCode; N_KEYS_PER_PLAYER] = PLAYER_KEY_ASSIGNMENTS[0];
pub const PLAYER_TWO_KEYS: [KeyCode; N_KEYS_PER_PLAYER] = PLAYER_KEY_ASSIGNMENTS[1];
pub const N_BULLETS: u8 = 2;
pub const N_DODGES: u8 = 1;
pub const DEFAULT_MARKSMANSHIP: u8 = 100;
pub const DEFAULT_LUCK: u8 = 50;
pub const N_MAX_ROUND: u8 = 6;
pub const DEFAULT_DAMAGE: u8 = 1;
pub const DEFAULT_HEALTH: u8 = 3;
pub const DEFAULT_COUNTDOWN_TIMER: f32 = 3.0;
pub const DEFAULT_BETTING_TIMER: f32 = 5.0;
