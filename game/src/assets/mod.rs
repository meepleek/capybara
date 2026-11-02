use bevy_asset_loader::prelude::*;
use iyes_progress::ProgressPlugin;

use crate::prelude::*;

#[derive(Resource, Debug)]
pub(super) struct InitialScreen {
    pub initial: Screen,
    pub show_splash: bool,
}

pub(super) fn plugin(initial_screen: Screen) -> impl Fn(&mut App) {
    move |app| {
        let show_splash = initial_screen == Screen::Splash;
        app.insert_resource(InitialScreen {
            initial: if show_splash {
                Screen::MainMenu
            } else {
                initial_screen
            },
            show_splash,
        });
        app.add_plugins(ProgressPlugin::<Screen>::new());
        app.add_loading_state(
            LoadingState::new(Screen::Splash).load_collection::<SpriteAssets>(), // .load_collection::<FontAssets>()
                                                                                 // .load_collection::<SfxAssets>()
                                                                                 // .load_collection::<MusicAssets>()
        );
        // app.add_systems(Startup, setup_particles);
        //
        //

        // // default font
        // Assets::insert(
        //     &mut app.world_mut().resource_mut(),
        //     AssetId::default(),
        //     Font {
        //         data: include_bytes!("../../assets/fonts/AtkinsonHyperlegibleMono_medium.ttf")
        //             .to_vec()
        //             .into(),
        //         ..default()
        //     },
        // )
        // .expect("Failed to set default font");
    }
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 4))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/capy.png")]
    pub sheet: Handle<Image>,
}
