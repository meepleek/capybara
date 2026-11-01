use std::marker::PhantomData;

use crate::{GridStorage, prelude::*};
use bevy::prelude::*;
use mplk_ecs::transform::TrackPosition;

pub trait DebugGridValue<TValue = ()> {
    fn value_to_char(value: &TValue) -> char;
}

#[derive(Component)]
pub struct HideDebugGrid;

#[derive(Resource, Default)]
pub struct DebugGridColor<TGrid, TValue: Send + Sync = ()> {
    color: Color,
    _phantom_grid: PhantomData<TGrid>,
    _phantom_val: PhantomData<TValue>,
}
impl<TGrid, TValue: Send + Sync> DebugGridColor<TGrid, TValue> {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            _phantom_grid: PhantomData,
            _phantom_val: PhantomData,
        }
    }
}

pub struct DebugGridPlugin<
    TGrid: GridSize + GridTileSize + TrackPosition + GridStorage<TValue> + DebugGridValue<TValue>,
    TValue = (),
> {
    color: Color,
    _phantom_grid: PhantomData<TGrid>,
    _phantom_val: PhantomData<TValue>,
}
impl<
    TGrid: Component
        + GridSize
        + GridTileSize
        + TrackPosition
        + GridStorage<TValue>
        + DebugGridValue<TValue>,
    TValue: Send + Sync + 'static,
> Default for DebugGridPlugin<TGrid, TValue>
{
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            _phantom_grid: PhantomData,
            _phantom_val: PhantomData,
        }
    }
}
impl<
    TGrid: Component
        + GridSize
        + GridTileSize
        + TrackPosition
        + GridStorage<TValue>
        + DebugGridValue<TValue>,
    TValue: Send + Sync + 'static,
> DebugGridPlugin<TGrid, TValue>
{
    pub fn new(color: Color) -> Self {
        Self { color, ..default() }
    }

    fn draw_grid_gizmos(
        grid_q: Query<(&GlobalTransform, &TGrid), Without<HideDebugGrid>>,
        mut gizmos: Gizmos,
        color: Res<DebugGridColor<TGrid, TValue>>,
    ) {
        for (grid_t, grid) in grid_q {
            gizmos
                .grid_2d(
                    Isometry2d::from_translation(grid_t.translation().truncate()),
                    grid.grid_size().as_uvec2(),
                    Vec2::splat(grid.tile_size() as _),
                    color.color,
                )
                .outer_edges();
            for t in grid.iter_tiles() {
                if let Some(val) = grid.get(t) {
                    let pos = grid.tile_to_world(t).expect("invalid tile pos");
                    let c = TGrid::value_to_char(val);
                    gizmos.text_2d(
                        pos,
                        &c.to_string(),
                        grid.tile_size() as f32 * 0.7,
                        Vec2::ZERO,
                        color.color,
                    );
                }
            }
        }
    }
}
impl<
    TGrid: Component
        + GridSize
        + GridTileSize
        + TrackPosition
        + GridStorage<TValue>
        + DebugGridValue<TValue>,
    TValue: Send + Sync + 'static,
> Plugin for DebugGridPlugin<TGrid, TValue>
{
    fn build(&self, app: &mut App) {
        let color = self.color;
        app.insert_resource(DebugGridColor::<TGrid, TValue>::new(color))
            .add_systems(Update, Self::draw_grid_gizmos);
    }
}
