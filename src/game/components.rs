use std::ops::AddAssign;

use super::*;

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct MenuEntity;

#[derive(Component)]
#[require(Health, Luck, Buff, Damage, Dodges, Bullets, Marksmanship)]
pub struct Player {
    pub value: u8,
}

#[derive(Component)]
pub struct Health {
    pub value: u8,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            value: DEFAULT_HEALTH,
        }
    }
}

pub struct Dice {
    pub value: u8,
}

impl Dice {
    pub fn roll(&self) -> u8 {
        let mut rng = rand::rng();
        let nums: Vec<u8> = (1..self.value).collect();

        *nums.choose(&mut rng).unwrap()
    }
}

impl AddAssign for Dice {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

#[derive(Component)]
pub struct Luck {
    pub value: Dice,
}

impl Default for Luck {
    fn default() -> Self {
        Luck {
            value: Dice {
                value: DEFAULT_LUCK,
            },
        }
    }
}

#[derive(Component)]
pub struct Marksmanship {
    pub value: Dice,
}

impl Default for Marksmanship {
    fn default() -> Self {
        Marksmanship {
            value: Dice {
                value: DEFAULT_MARKSMANSHIP,
            },
        }
    }
}

#[derive(Component)]
pub struct Dodges {
    pub value: u8,
}

impl Default for Dodges {
    fn default() -> Self {
        Dodges { value: N_DODGES }
    }
}

#[derive(Component)]
pub struct Bullets {
    pub value: u8,
}

impl Default for Bullets {
    fn default() -> Self {
        Bullets { value: N_BULLETS }
    }
}

#[derive(Component)]
pub struct Damage {
    pub value: u8,
}

impl Default for Damage {
    fn default() -> Self {
        Damage {
            value: DEFAULT_DAMAGE,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Buff {
    pub value: Option<Buffes>,
}

impl Default for Buff {
    fn default() -> Self {
        Buff { value: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PlayerStates {
    #[default]
    Idle,
    Attacking,
    NotAttacking,
    Dodging,
    NotDodging,
    Buffing,
}

#[derive(Component, Debug, Default)]
pub struct PlayerState(pub PlayerStates);

#[derive(Component)]
pub struct KeyAssignment(pub [KeyCode; N_KEYS_PER_PLAYER]);

impl KeyAssignment {
    pub fn derive_player_state(&self, key: &KeyCode) -> Option<PlayerStates> {
        if *key == self.0[0] {
            Some(PlayerStates::Attacking)
        } else if *key == self.0[1] {
            Some(PlayerStates::Dodging)
        } else if *key == self.0[2] {
            Some(PlayerStates::Buffing)
        } else {
            None
        }
    }
}
