use crate::prelude::*;

#[derive(Debug, Component, Clone, FromTemplate, Deref, DerefMut)]
pub struct InventoryIndex(pub u8);
