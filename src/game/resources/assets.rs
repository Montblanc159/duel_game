use super::*;

#[derive(Resource, Default)]
pub struct HandSpritesheet {
    pub spritesheet: Option<Handle<Image>>,
}
