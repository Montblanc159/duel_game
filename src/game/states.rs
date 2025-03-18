use super::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayStates {
    #[default]
    Paused,
    Preparing,
    Countdown,
    Betting,
    Fighting,
    RoundingUp,
    GameOver,
}

impl PlayStates {
    pub fn next(&self) -> Self {
        match *self {
            PlayStates::Preparing => PlayStates::Countdown,
            PlayStates::Countdown => PlayStates::Betting,
            PlayStates::Betting => PlayStates::Fighting,
            PlayStates::Fighting => PlayStates::RoundingUp,
            PlayStates::RoundingUp => PlayStates::Preparing,
            _ => *self,
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppStates {
    #[default]
    Menu,
    Loading,
    InGame,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AppStates>();
    app.init_state::<PlayStates>();
}
