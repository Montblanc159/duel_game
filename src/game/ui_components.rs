use super::*;

#[derive(Component)]
pub struct PlayStateText;

#[derive(Component)]
pub struct RoundNumberText;

#[derive(Component)]
pub struct HealthBar {
    pub value: u8,
}

#[derive(Component)]
pub struct ManaBar {
    pub value: u8,
}

#[derive(Component)]
pub struct StaminaBar {
    pub value: u8,
}

#[derive(Component)]
pub struct BuffText {
    pub value: u8,
}

#[derive(Component)]
pub struct TimerUIText;

#[derive(Component)]
pub struct PressSpacebarText;

#[derive(Component)]
pub struct PlayerTickText;

#[derive(Component)]
pub struct AlertText;
