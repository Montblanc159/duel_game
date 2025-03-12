use super::*;

#[derive(Resource)]
pub struct BettingTimer(pub Timer);

#[derive(Resource)]
pub struct CountdownTimer(pub Timer);

#[derive(Resource)]
pub struct RoundCounter(pub u8);

#[derive(Resource)]
pub struct GameOver(pub bool);
