use mplk_ecs::hiearchy::RotationOf;

use crate::{
    gameplay::{
        self,
        drag::DragPosition,
        grid::INGREDIENT_TILE_SIZE_F32,
        inventory::InventoryIndex,
        layout::GameLayout,
        piece::{self, INVENTORY_MAX, Piece, template::PieceTemplate},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_pieces.run_if(in_state(Screen::Gameplay)));
}

fn spawn_pieces(piece_q: Query<&Piece>, mut cmd: Commands, layout: GameLayout) {
    use PieceTemplate::*;

    let mut rng = rng();
    if piece_q.is_empty() {
        let lvl_pieces = [Line2, Line3, Line4, T, Square, Corner, L, LFlip, Z, ZFlip];
        for i in 0..INVENTORY_MAX {
            let template = lvl_pieces
                .choose(&mut rng)
                .expect("No valid piece template");
            let mut p = template.piece();
            for _ in 0..(rng.random_range(0..=3)) {
                p.rotate_cw();
            }
            let position = layout.piece_position(i);
            cmd.spawn_scene(bsn! {
                @{piece_scene(p, position, i)}
                InventoryIndex({i as u8})
            });
        }
    }
}

pub fn piece_scene(piece: Piece, position: impl Into<Vec2>, inventory_idx: u8) -> impl Scene {
    let mut tiles: Vec<Box<dyn Scene>> = Vec::new();
    for t in &piece.tiles {
        let translation = (t.as_world_vec2() * INGREDIENT_TILE_SIZE_F32).extend(0.);
        let tile = bsn! {
            #piece_tile
            Transform { translation }
            Visibility
            Pickable
            Children [
                #piece_border
                Transform { translation: { Vec3::Z * 0.1 } }
                from_collection(|sprites: &SpriteAssets| Sprite {
                    image: sprites.sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.layout.clone(),
                        index: 0
                    }),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(5.),
                        center_scale_mode: SliceScaleMode::Tile {
                            stretch_value: 1. },
                        ..default()
                    }),
                    custom_size: Some(Vec2::splat(INGREDIENT_TILE_SIZE_F32)),
                    ..default()
                })
                --
                #piece_content
                Transform { translation: { Vec2::new(11., -11.).extend(0.) } }
                from_collection(|sprites: &SpriteAssets| Sprite {
                    image: sprites.sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.layout.clone(),
                        index: 1
                    }),
                    custom_size: Some(Vec2::splat(89.)),
                    ..default()
                })
                --
                #piece_picking_overlay
                Mesh2d::from_collection(SpriteAssets::piece_picking_mesh)
                Pickable
            ]
        };
        tiles.push(Box::new(tile));
    }

    let position = position.into();
    let translation = position.extend(0.);
    bsn! {
        #piece_root
        piece
        Transform { translation }
        InventoryIndex(inventory_idx)
        DragPosition::new(position)
        Visibility
        DespawnOnExit::<Screen>(Screen::Gameplay)
        Pickable
        on(gameplay::drag::on_draggable_hover)
        on(gameplay::drag::on_draggable_blur)
        on(gameplay::drag::on_draggable_press)
        on(gameplay::drag::on_draggable_drag)
        on(piece::drag::on_drag_end)
        Children[
            #piece_sprites
            Visibility
            Transform
            RotationOf(#piece)
            Children [{tiles}]
        ]
    }
}
