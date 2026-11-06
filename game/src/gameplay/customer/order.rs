use mplk_ecs::relationship_1_to_n;

use crate::{
    gameplay::{
        customer::Customer,
        ingredient::Ingredient,
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (tick_order_timer, update_order_time).run_if(in_state(Screen::Gameplay)),
    );
    app.add_observer(spawn_order);
}

#[derive(Component, Debug, Clone, FromTemplate)]
pub struct Order {
    pub ingredients: HashSet<Ingredient>,
    pub timer: Timer,
}
impl InventoryPosition for Order {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2 {
        layout.customer_order_position(idx.0)
    }
}

relationship_1_to_n!(OrderOf, CustomerOrders);

#[derive(Event, Debug)]
pub struct NewOrder {
    pub ingredients: Vec<Ingredient>,
    pub duration: Duration,
    pub customer_e: Entity,
}

#[derive(Event)]
pub struct OrderDone(Entity);

#[derive(Event)]
pub struct OrderMissed(Entity);

pub(super) fn order_scene(order: &NewOrder, idx: u8, layout: &GameLayout) -> impl Scene {
    let mut ingredient_icons: Vec<Box<dyn Scene>> = Vec::new();
    for (i, ingredient) in order.ingredients.iter().enumerate() {
        let position = layout.order_ingredient_position(i as u8);
        let ingredient_scene = ingredient.icon_scene(position.extend(0.));
        ingredient_icons.push(Box::new(ingredient_scene));
    }

    let translation = layout.customer_order_position(idx).extend(1.);
    bsn! {
        #order
        Order {
            ingredients: {order.ingredients.iter().cloned().collect::<HashSet<_>>()},
            timer: Timer::new(order.duration, TimerMode::Once),
        }
        InventoryIndex(idx)
        ChildOf({ order.customer_e })
        OrderOf({ order.customer_e })
        Transform { translation }
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children [
            #order_title
            Text2d("Drink Name")
            Transform::from_translation(Vec3::Y * 55.)
            --
            #order_time
            OrderTimeText
            Text2d({format!("{} s", order.duration.as_secs_f32())})
            Transform::from_translation(Vec3::Y * 35.)
            --
            {ingredient_icons}
        ]
    }
}

fn spawn_order(
    ev: On<NewOrder>,
    layout: GameLayout,
    mut cmd: Commands,
    customer_q: Query<&InventoryIndex, With<Customer>>,
) {
    let idx = or_return!(customer_q.get(ev.customer_e));
    cmd.spawn_scene(order_scene(ev.event(), idx.0, &layout));
}

fn tick_order_timer(mut order_q: Query<(Entity, &mut Order)>, time: Res<Time>, mut cmd: Commands) {
    for (e, mut order) in &mut order_q {
        order.timer.tick(time.delta());
        if order.timer.just_finished() {
            cmd.trigger(OrderMissed(e));
        }
    }
}

#[derive(Component, FromTemplate)]
struct OrderTimeText;

fn update_order_time(
    mut text_q: Query<(&mut Text2d, &ChildOf), With<OrderTimeText>>,
    order_q: Query<&Order>,
) {
    for (mut text, parent) in &mut text_q {
        let order = or_continue!(order_q.get(parent.0));
        text.0 = format!("{} s", order.timer.remaining_secs().round());
    }
}
