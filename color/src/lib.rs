use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default, Deref, DerefMut)]
pub struct OutlineColor(pub Color);
