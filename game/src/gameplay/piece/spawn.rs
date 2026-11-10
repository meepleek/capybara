use crate::{
    gameplay::{
        self,
        drag::DragPosition,
        grid::{INGREDIENT_TILE_SIZE_F32, IngredientGrid},
        inventory::InventoryIndex,
        layout::GameLayout,
        piece::{AddPiece, Piece, ability::rotate_piece},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_piece);
}

pub fn piece_scene(piece: Piece, position: impl Into<Vec2>, inventory_idx: u8) -> impl Scene {
    let mut tiles: Vec<Box<dyn Scene>> = Vec::new();
    for t in &piece.tiles {
        let translation = (t.as_world_vec2() * INGREDIENT_TILE_SIZE_F32).extend(2.);
        let tile = bsn! {
            #piece_tile
            Transform { translation }
            Visibility
            Pickable
            Children [
                #piece_border
                Transform { translation: { Vec3::Z * 0.1 } }
                from_collection(|sprites: &SpriteAssets| Sprite {
                    image: sprites.ingredients_sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.ingredients_layout.clone(),
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
                    image: sprites.ingredients_sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.ingredients_layout.clone(),
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

    let sprites_translation = (piece.center * INGREDIENT_TILE_SIZE_F32).extend(0.);
    let position = position.into();
    let transform = Transform::from_translation(position.extend(0.));
    bsn! {
        #piece_root
        piece
        transform
        InventoryIndex(inventory_idx)
        DragPosition::new(position)
        Visibility
        DespawnOnExit::<Screen>(Screen::Gameplay)
        Pickable
        on(gameplay::drag::on_draggable_hover)
        on(gameplay::drag::on_draggable_blur)
        on(gameplay::drag::on_draggable_press)
        on(gameplay::drag::on_draggable_drag)
        on(gameplay::drag::on_draggable_drag_end_restore_position::<Piece>)
        on(gameplay::drag::on_draggable_drag_end_snapped::<Piece, IngredientGrid>)
        on(rotate_piece)
        Children[
            #piece_sprites
            Visibility
            Transform { translation: sprites_translation }
            Children [{tiles}]
        ]
    }
}

fn add_piece(
    ev: On<AddPiece>,
    mut cmd: Commands,
    layout: GameLayout,
    piece_q: Query<(), With<Piece>>,
) {
    // 1 works for odds, 0 works for evens
    // todo: why thell is that?!?
    let mut p = ev.piece.piece().with_rotations(ev.cw_rotations as i32 % 2);

    for _ in 0..ev.cw_rotations {
        p.rotate_cw();
    }
    let idx = ev.idx.unwrap_or_else(|| piece_q.count() as u8);
    let position = layout.piece_position(idx);
    tracing::warn!(?p);
    cmd.spawn_scene(piece_scene(p, position, idx));
}
