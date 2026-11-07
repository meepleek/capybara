use mplk_ecs::observer::readd_default_component;

use crate::{
    gameplay::{
        customer::{
            Customer, CustomerKind,
            order::{CustomerOrders, NewOrder, order_scene},
        },
        inventory::InventoryIndex,
        layout::GameLayout,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(process_new_order)
        .add_observer(readd_default_component::<CustomerOrders>);
}

fn process_new_order(
    ev: On<NewOrder>,
    layout: GameLayout,
    mut cmd: Commands,
    customer_q: Query<&InventoryIndex, With<Customer>>,
) {
    let idx = or_return!(customer_q.get(ev.customer_e));
    cmd.spawn_scene(order_scene(ev.event(), idx.0, &layout));
}

pub fn customer_scene(idx: u8, translation: Vec3, kind: CustomerKind) -> impl Scene {
    bsn! {
        #customer
        Customer
        kind
        CustomerOrders
        InventoryIndex(idx)
        Transform { translation }
        from_collection(move |sprites: &SpriteAssets| Sprite {
            image: sprites.bar_sheet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprites.bar_layout.clone(),
                index: 4
            }),
            custom_size: Some(Vec2::splat(256.)),
            ..default()
        })
        Pickable
        DespawnOnExit<Screen>(Screen::Gameplay)
    }
}
