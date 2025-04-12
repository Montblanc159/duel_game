use super::*;

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
