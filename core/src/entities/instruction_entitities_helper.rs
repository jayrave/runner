use crate::components::Drawable;
use crate::graphics::data;
use crate::graphics::data::Tile;
use crate::rect::Rect;
use specs::join::Join;
use specs::WorldExt;
use specs::{Builder, Component, World};

/// Returns the max bottom of all the tiles created
pub(super) fn create_tiles_at_world_center<T>(
    world: &mut World,
    component: T,
    tile_to_world_divider: u32,
    tile_spacing: i32,
    extra_y_offset: i32,
    tiles: &[Tile],
) -> i32
where
    T: Component + Copy + Send + Sync,
{
    // Initially create drawables at 0, 0
    let mut last_icon_x_end_at = 0;
    let mut max_drawable_height = 0;
    let drawables: Vec<Drawable> = tiles
        .into_iter()
        .map(|tile| {
            let tile_data = data::build_tile_data(tile.clone());
            let width_in_world = tile_data.bounds_in_tile_sheet.width() / tile_to_world_divider;
            let height_in_world = tile_data.bounds_in_tile_sheet.height() / tile_to_world_divider;

            let bounds_in_world = Rect::new(
                last_icon_x_end_at + tile_spacing,
                0,
                width_in_world,
                height_in_world,
            );

            // Update tracked measures
            last_icon_x_end_at = bounds_in_world.right();
            if max_drawable_height < height_in_world {
                max_drawable_height = height_in_world;
            }

            Drawable {
                tile_data,
                world_bounds: bounds_in_world,
            }
        })
        .collect();

    // Compute offsets to move drawables to world center
    let height_offset: i32 = -((max_drawable_height / 2) as i32) + extra_y_offset;
    let width_offset = -last_icon_x_end_at / 2;

    // Offset drawables & create entities
    for mut drawable in drawables.into_iter() {
        drawable.world_bounds.offset(width_offset, height_offset);
        world.create_entity().with(component).with(drawable).build();
    }

    height_offset.abs()
}

pub(super) fn remove_all_entities_matching_type<T: Component>(world: &mut World) {
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
