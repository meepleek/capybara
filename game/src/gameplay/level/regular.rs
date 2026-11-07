use std::ops::Range;

use mplk_ecs::observer::ensure_single_at_most;

use crate::{
    gameplay::{
        customer::{
            CustomerKind,
            order::{CustomerOrders, NewOrder, Order, OrderFinished, OrderMissed},
        },
        ingredient::{CommonIngredient, Ingredient},
        level::{LevelFinished, LevelIndex, Tool},
        piece::{AddPiece, INVENTORY_MAX, Piece, bag::PieceBag, template::PieceTemplate},
    },
    prelude::*,
    screens::run_conditions::in_regular_level,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(processed_missed_order)
        .add_observer(processed_finished_order)
        .add_observer(ensure_single_at_most::<LevelData>);
    app.add_systems(Update, (spawn_pieces, add_orders).run_if(in_regular_level));
}

#[derive(Component, Debug, Clone)]
pub struct LevelData {
    pub piece_bag: Vec<PieceTemplate>,
    pub piece_bag_random_count: u8,
    pub concurrent_order_max: u8,
    pub order_delay: Range<u8>,
    pub order_count: u8,
    pub max_order_ingredient_rank: u8,
    pub customers: [CustomerKind; 3],
}
impl LevelData {
    pub fn basics() -> Self {
        LevelData {
            piece_bag: PieceTemplate::classic_bag(),
            piece_bag_random_count: 1,
            concurrent_order_max: 2,
            order_delay: 20..30,
            order_count: 5,
            // fused ingredients are rank 2
            max_order_ingredient_rank: 2,
            customers: CustomerKind::regulars(),
        }
    }

    pub fn new_order(
        &self,
        customer_kind: CustomerKind,
        customer_e: Entity,
        lvl_idx: u8,
        rng: &mut impl Rng,
    ) -> NewOrder {
        let can_use_fused_ingredients =
            lvl_idx > Tool::IngredientFuser.min_lvl() && customer_kind.order_fused_ingredients();
        let rank = rng
            .random_range(customer_kind.ingredient_rank())
            .min(self.max_order_ingredient_rank);
        let common_ingredients = CommonIngredient::valid_ingredients(lvl_idx);
        let fused_ingredients = if can_use_fused_ingredients {
            common_ingredients.iter().map(|ing| ing.fuse()).collect()
        } else {
            Vec::new()
        };
        let mut ingredients = Vec::with_capacity(rank as usize);
        let mut current_rank = 0;
        while ingredients.len() < 6 && current_rank < rank {
            let rank_delta = rank - current_rank;
            let ingredient =
                if !fused_ingredients.is_empty() && rank_delta >= 2 && rng.random::<bool>() {
                    current_rank += 2;
                    Ingredient::Fused(
                        *fused_ingredients
                            .choose(rng)
                            .expect("invalid fused ingredient set"),
                    )
                } else {
                    current_rank += 1;
                    Ingredient::Common(
                        *common_ingredients
                            .choose(rng)
                            .expect("invalid common ingredient set"),
                    )
                };
            ingredients.push(ingredient);
        }

        NewOrder {
            ingredients,
            duration: customer_kind.order_duration(),
            customer_e,
        }
    }
}
#[derive(Component, Debug, Clone, Default)]
pub struct LevelStats {
    pub spawned_order_count: u8,
}

fn processed_finished_order(
    _ev: On<OrderFinished>,
    lvl_stats: Single<&LevelStats>,
    lvl_data: Single<&LevelData>,
    mut cmd: Commands,
) {
    if lvl_stats.spawned_order_count >= lvl_data.order_count {
        cmd.trigger(LevelFinished);
    }
}

fn processed_missed_order(
    _ev: On<OrderMissed>,
    mut next: ResMut<NextState<Screen>>,
    _lvl_data: Single<&LevelData>, // filter out tutorials etc.
) {
    next.set(Screen::GameOver);
}

fn spawn_pieces(
    piece_q: Query<(), With<Piece>>,
    mut cmd: Commands,
    mut piece_bag: Single<&mut PieceBag>,
) {
    let mut rng = rng();
    if piece_q.is_empty() {
        for i in 0..INVENTORY_MAX {
            let piece = piece_bag.choose(&mut rng);
            let cw_rotations = rng.random_range(0..=3);
            cmd.trigger(AddPiece {
                piece,
                cw_rotations,
                idx: Some(i),
            });
        }
    }
}

fn add_orders(
    customer_q: Query<(Entity, &CustomerKind, &CustomerOrders)>,
    order_q: Query<(), With<Order>>,
    mut lvl_stats: Single<&mut LevelStats>,
    lvl_data: Single<&LevelData>,
    lvl_idx: Res<LevelIndex>,
    mut cmd: Commands,
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    if lvl_stats.spawned_order_count >= lvl_data.order_count {
        return;
    }

    let timer = timer.get_or_insert_with(|| Timer::new(sec(1.), TimerMode::Repeating));
    timer.tick(time.delta());
    let order_count = order_q.count();
    if order_count >= lvl_data.concurrent_order_max as usize {
        return;
    }

    if timer.is_finished() || order_count == 0 {
        let mut rng = rng();
        timer.set_duration(sec(rng.random_range(lvl_data.order_delay.clone()) as f32));
        let (customer_e, customer_kind, _) = or_return!(
            customer_q
                .iter()
                .sort_unstable_by_key::<&CustomerOrders, _>(|orders| orders.len())
                .next()
        );
        let new_order = lvl_data.new_order(*customer_kind, customer_e, lvl_idx.0, &mut rng);
        cmd.trigger(new_order);
        lvl_stats.spawned_order_count += 1;
    }
}
