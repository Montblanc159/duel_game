use super::*;

// AUDIO

#[derive(Resource, Default)]
pub struct MainThemeAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct MenuAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct MenuTransitionAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct ShootAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct DodgeAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct BuffAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct DamageAudio {
    pub audio: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct StateChangeAudio {
    pub audio: Option<Handle<AudioSource>>,
}

// Textures

#[derive(Resource, Default)]
pub struct HandSpritesheet {
    pub spritesheet: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct BgSprite {
    pub sprite: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct HealthSpritesheet {
    pub spritesheet: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct ManaSpritesheet {
    pub spritesheet: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct StaminaSpritesheet {
    pub spritesheet: Option<Handle<Image>>,
}
