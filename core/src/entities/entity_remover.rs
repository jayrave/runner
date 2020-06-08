use specs::{World, Component};
use specs::WorldExt;
use specs::join::Join;
use crate::components::Drawable;

pub(super) fn remove_all_entities_matching_type<T : Component>(world: &mut World) {
    let mut entities_to_be_removed = Vec::new();
    {
        let entities = world.entities();
        let icons_storage = world.read_storage::<T>();
        let drawables_storage = world.read_storage::<Drawable>();

        for (entity, _, _) in (&entities, &icons_storage, &drawables_storage).join() {
            entities_to_be_removed.push(entity);
        }
    }

    let entities = world.entities_mut();
    for entity in entities_to_be_removed {
        entities
            .delete(entity)
            .expect("Entity couldn't be deleted!")
    }
}
