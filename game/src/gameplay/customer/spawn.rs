use crate::{
    gameplay::{
        customer::{
            Customer,
            order::{NewOrder, order_scene},
        },
        ingredient::Ingredient,
        inventory::InventoryIndex,
        layout::GameLayout,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_observer(spawn_order);
}

pub const CUSTOMER_COUNT: u8 = 3;

fn setup(mut cmd: Commands, layout: GameLayout) {
    use Ingredient::*;

    let mut customer_e = Entity::PLACEHOLDER;
    for i in 0..CUSTOMER_COUNT {
        let pos = layout.customer_position(i);
        customer_e = cmd.spawn_scene(customer_scene(i, pos.extend(1.))).id();
    }

    cmd.trigger(NewOrder {
        ingredients: vec![Apple, Water],
        duration: sec(60.),
        customer_e,
    });
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

fn customer_scene(idx: u8, translation: Vec3) -> impl Scene {
    bsn! {
        #customer
        Customer
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
