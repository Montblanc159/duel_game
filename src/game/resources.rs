use super::*;

pub mod assets;

#[derive(Resource)]
pub struct BettingTimer(pub Timer);

#[derive(Resource)]
pub struct CountdownTimer(pub Timer);

#[derive(Resource)]
pub struct RoundCounter(pub u8);

#[derive(Resource)]
pub struct GameOver(pub bool);

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

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

    app.insert_resource(assets::HandSpritesheet { ..default() });

    app.insert_resource(AssetsLoading(default()));
}
