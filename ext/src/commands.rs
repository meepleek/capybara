use bevy::prelude::*;

pub trait CommandsExt {
    /** Try to insert a bundle to an entity */
    fn try_insert_to(&mut self, entity: Entity, bundle: impl Bundle);

    /** Try to delete a component from an entity */
    fn try_remove_from<B: Bundle>(&mut self, entity: Entity);
}
impl CommandsExt for Commands<'_, '_> {
    fn try_insert_to(&mut self, entity: Entity, bundle: impl Bundle) {
        if let Ok(mut e_cmd) = self.get_entity(entity) {
            e_cmd.try_insert(bundle);
        }
    }

    fn try_remove_from<B: Bundle>(&mut self, entity: Entity) {
        if let Ok(mut e_cmd) = self.get_entity(entity) {
            e_cmd.try_remove::<B>();
        }
    }
}
