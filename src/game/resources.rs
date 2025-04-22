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

    // Audios
    app.insert_resource(assets::MainThemeAudio { ..default() });
    app.insert_resource(assets::MenuAudio { ..default() });
    app.insert_resource(assets::StateChangeAudio { ..default() });
    app.insert_resource(assets::MenuTransitionAudio { ..default() });
    app.insert_resource(assets::ShootAudio { ..default() });
    app.insert_resource(assets::DodgeAudio { ..default() });
    app.insert_resource(assets::BuffAudio { ..default() });
    app.insert_resource(assets::DamageAudio { ..default() });

    // Textures
    app.insert_resource(assets::HandSpritesheet { ..default() });
    app.insert_resource(assets::HealthSpritesheet { ..default() });
    app.insert_resource(assets::StaminaSpritesheet { ..default() });
    app.insert_resource(assets::ManaSpritesheet { ..default() });
    app.insert_resource(assets::BgSprite { ..default() });

    app.insert_resource(AssetsLoading(default()));
}
