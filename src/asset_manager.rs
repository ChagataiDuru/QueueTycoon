use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppGeneralState {
    #[default]
    AssetLoading,
    MainMenu,
}

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppGeneralState>()
           .add_loading_state(
            LoadingState::new(AppGeneralState::AssetLoading)
                .continue_to_state(AppGeneralState::MainMenu)
                .load_collection::<AudioAssets>()
                .load_collection::<TileAssets>(),
        )
        .add_systems(OnEnter(AppGeneralState::MainMenu), start_background_audio);
    }
}

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/main_menu.ogg")]
    background: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "tiles/WoodTile.png")]
    #[asset(image(sampler = nearest))]
    woodtile: Handle<Image>,
}

fn start_background_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.background.clone(),
        settings: PlaybackSettings::LOOP,
    });
}