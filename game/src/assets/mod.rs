use bevy_asset_loader::prelude::*;
use iyes_progress::ProgressPlugin;

use crate::{gameplay::ingredient::INGREDIENT_TILE_SIZE_F32, prelude::*};

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
            LoadingState::new(Screen::Splash)
                .load_collection::<SpriteAssets>()
                .finally_init_resource::<FinalizeResourceAssets>(), // .load_collection::<FontAssets>()
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
    pub piece_picking_mesh: Handle<Mesh>,
}

#[derive(Resource)]
struct FinalizeResourceAssets {}
impl FromWorld for FinalizeResourceAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let picking_mesh_size = INGREDIENT_TILE_SIZE_F32 * 1.1;
        let piece_picking_mesh = meshes.add(Rectangle::new(picking_mesh_size, picking_mesh_size));
        let mut sprites = world.resource_mut::<SpriteAssets>();
        sprites.piece_picking_mesh = piece_picking_mesh;
        FinalizeResourceAssets {}
    }
}
