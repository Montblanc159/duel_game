use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameOvers {
    Tie,
    Winner,
}

#[derive(Event)]
pub struct GameOverEvent {
    pub player: Option<u8>,
    pub state: GameOvers,
}

#[derive(Event)]
pub struct PlayerStateChangeEvent {
    pub player: u8,
}

#[derive(Event)]
pub struct DamageEvent {
    pub player: u8,
    pub value: u8,
}

#[derive(Event)]
pub struct MissedEvent {
    pub player: u8,
}

#[derive(Event)]
pub struct DodgedEvent {
    pub player: u8,
}

#[derive(Event)]
pub struct TickPlayerEvent {
    pub player: u8,
    pub value: String,
}

#[derive(Event)]
pub struct AlertEvent {
    pub value: String,
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<GameOverEvent>();
    app.add_event::<PlayerStateChangeEvent>();
    app.add_event::<DamageEvent>();
    app.add_event::<MissedEvent>();
    app.add_event::<DodgedEvent>();
    app.add_event::<TickPlayerEvent>();
    app.add_event::<AlertEvent>();
}
