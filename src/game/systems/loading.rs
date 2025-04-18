use super::*;
use bevy::asset::LoadState;
use bevy::asset::UntypedAssetId;

fn load_main_theme_audio(
    asset_server: Res<AssetServer>,
    mut hand: ResMut<assets::MainThemeAudio>,
    mut loading: ResMut<AssetsLoading>,
) {
    let handle = asset_server.load("audios/main-theme.wav");

    loading.0.push(handle.clone().untyped());

    hand.audio = Some(handle);
}

fn load_menu_audio(
    asset_server: Res<AssetServer>,
    mut hand: ResMut<assets::MenuAudio>,
    mut loading: ResMut<AssetsLoading>,
) {
    let handle = asset_server.load("audios/menu-audio.wav");

    loading.0.push(handle.clone().untyped());

    hand.audio = Some(handle);
}

fn load_hands_textures(
    asset_server: Res<AssetServer>,
    mut hand: ResMut<assets::HandSpritesheet>,
    mut loading: ResMut<AssetsLoading>,
) {
    let hand_spritesheet_handle = asset_server.load("images/hand_spritesheet.png");

    loading.0.push(hand_spritesheet_handle.clone().untyped());

    hand.spritesheet = Some(hand_spritesheet_handle);
}

fn load_health_textures(
    asset_server: Res<AssetServer>,
    mut health_sprite: ResMut<assets::HealthSpritesheet>,
    mut loading: ResMut<AssetsLoading>,
) {
    let sprite_handle = asset_server.load("images/health.png");

    loading.0.push(sprite_handle.clone().untyped());

    health_sprite.spritesheet = Some(sprite_handle);
}

fn load_stamina_textures(
    asset_server: Res<AssetServer>,
    mut stamina_sprite: ResMut<assets::StaminaSpritesheet>,
    mut loading: ResMut<AssetsLoading>,
) {
    let sprite_handle = asset_server.load("images/stamina.png");

    loading.0.push(sprite_handle.clone().untyped());

    stamina_sprite.spritesheet = Some(sprite_handle);
}

fn load_mana_textures(
    asset_server: Res<AssetServer>,
    mut mana_sprite: ResMut<assets::ManaSpritesheet>,
    mut loading: ResMut<AssetsLoading>,
) {
    let sprite_handle = asset_server.load("images/mana.png");

    loading.0.push(sprite_handle.clone().untyped());

    mana_sprite.spritesheet = Some(sprite_handle);
}

fn load_bg_textures(
    asset_server: Res<AssetServer>,
    mut hand: ResMut<assets::BgSprite>,
    mut loading: ResMut<AssetsLoading>,
) {
    let bg_sprite_handle = asset_server.load("images/bg.png");

    loading.0.push(bg_sprite_handle.clone().untyped());

    hand.sprite = Some(bg_sprite_handle);
}

fn check_assets_loaded(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed(_) => {
            panic!("Resources failed to load")
        }
        LoadState::Loaded => {
            loading.0 = vec![];
            next_app_state.set(AppStates::Menu);
        }
        _ => {}
    }
}

fn spawn_loading_text(mut commands: Commands, window: Single<&Window>) {
    commands
        .spawn((
            Node {
                width: Val::Px(window.width()),
                height: Val::Px(window.height()),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GlobalZIndex(2),
            LoadingEntity,
        ))
        .with_child((
            Node {
                width: Val::Px(window.width()),
                ..default()
            },
            Text::new("Loading..."),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont::from_font_size(150.),
            LoadingEntity,
        ));
}

// HACK FOR FORGOTTEN METHOD IN UPDATE

trait AssetServerExt {
    fn get_group_load_state(&self, handles: impl IntoIterator<Item = UntypedAssetId>) -> LoadState;
}

impl AssetServerExt for AssetServer {
    fn get_group_load_state(&self, handles: impl IntoIterator<Item = UntypedAssetId>) -> LoadState {
        let mut load_state = LoadState::Loaded;

        for untyped_asset_id in handles {
            match self.get_load_state(untyped_asset_id) {
                Some(LoadState::Loaded) => continue,
                Some(LoadState::Loading) => {
                    load_state = LoadState::Loading;
                }
                Some(LoadState::Failed(e)) => return LoadState::Failed(e),
                Some(LoadState::NotLoaded) => return LoadState::NotLoaded,
                None => return LoadState::NotLoaded,
            }
        }

        load_state
    }
}

// END OF HACK

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppStates::Loading),
        (
            load_main_theme_audio,
            load_menu_audio,
            load_hands_textures,
            load_bg_textures,
            load_health_textures,
            load_mana_textures,
            load_stamina_textures,
        ),
    );
    app.add_systems(
        Update,
        (spawn_loading_text, check_assets_loaded).run_if(in_state(AppStates::Loading)),
    );
    app.add_systems(OnExit(AppStates::Loading), clean_system::<LoadingEntity>);
}
