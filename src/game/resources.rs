use super::*;

#[derive(Resource)]
pub struct BettingTimer(pub Timer);

#[derive(Resource)]
pub struct CountdownTimer(pub Timer);

#[derive(Resource)]
pub struct RoundCounter(pub u8);

#[derive(Resource)]
pub struct GameOver(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BettingTimer(Timer::from_seconds(
        DEFAULT_BETTING_TIMER,
        TimerMode::Once,
    )));
    app.insert_resource(CountdownTimer(Timer::from_seconds(
        DEFAULT_COUNTDOWN_TIMER,
        TimerMode::Once,
    )));
    app.insert_resource(RoundCounter(1));
    app.insert_resource(GameOver(false));
}
