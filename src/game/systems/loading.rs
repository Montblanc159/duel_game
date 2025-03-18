use super::*;
use bevy::asset::LoadState;
use bevy::asset::UntypedAssetId;

fn load_hands_textures(
    asset_server: Res<AssetServer>,
    mut hand: ResMut<assets::HandSpritesheet>,
    mut loading: ResMut<AssetsLoading>,
) {
    let hand_spritesheet_handle = asset_server.load("images/hand_spritesheet.png");

    loading.0.push(hand_spritesheet_handle.clone().untyped());

    hand.spritesheet = Some(hand_spritesheet_handle);
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
            next_app_state.set(AppStates::InGame);
        }
        _ => {}
    }
}

fn spawn_loading_text(mut commands: Commands, windows_query: Query<&Window>) {
    let window = windows_query.single();

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
            InGameEntity,
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
    app.add_systems(OnEnter(AppStates::Loading), load_hands_textures);
    app.add_systems(
        Update,
        (spawn_loading_text, check_assets_loaded).run_if(in_state(AppStates::Loading)),
    );
    app.add_systems(OnExit(AppStates::Loading), clean_system::<LoadingEntity>);
}
