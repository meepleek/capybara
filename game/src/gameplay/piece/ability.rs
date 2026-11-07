use crate::{
    gameplay::{
        drag::DraggableDrop,
        ingredient::{AbilityUsed, CommonIngredient, FusedIngredient, Ingredient},
        piece::{Piece, PieceRotation, RotatePiece},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_piece_drop);
}

fn on_piece_drop(ev: On<DraggableDrop<Ingredient, Piece>>, mut cmd: Commands) {
    use CommonIngredient::*;
    use FusedIngredient::*;
    use PieceRotation::*;

    let rotation = match ev.draggable {
        Ingredient::Common(common_ingredient) => match common_ingredient {
            Apple => CW(1),
            Orange => CCW(1),
            Honey | Ginger | Air | Coffee | Water => unreachable!(),
        },
        Ingredient::Fused(fused_ingredient) => match fused_ingredient {
            Pineapple => CW(2),
            Lime => CCW(2),
            Ice | HoneyPlus | Jalapeno | AirPlus | Chocolate => unreachable!(),
        },
    };
    or_return!(cmd.get_entity(ev.target_entity)).trigger(|e| RotatePiece {
        entity: e,
        rotation,
    });
    cmd.trigger(AbilityUsed(ev.draggable));
}

pub fn rotate_piece(ev: On<RotatePiece>, mut piece_q: Query<&mut Piece>, mut cmd: Commands) {
    let mut piece = or_return!(piece_q.get_mut(ev.entity));
    match ev.rotation {
        PieceRotation::CW(rotations) => {
            for _ in 0..rotations {
                piece.rotate_cw();
            }
        }
        PieceRotation::CCW(rotations) => {
            for _ in 0..rotations {
                piece.rotate_ccw();
            }
        }
    }
    cmd.try_insert_to(
        ev.entity,
        TransformRotationDegreesLensSrc::new(ev.rotation.to_rot2d().as_degrees()).duration(ms(300)),
    );
}
