use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::Area;
use crate::resources::AreaManager;

pub fn debug_draw_entity_areas(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query::<&Area>();

    for area in query.iter(world) {
        area.draw(renderer);
    }

    let mut area_m = world.get_resource_mut::<AreaManager>().unwrap();
    area_m.draw_tile_areas(renderer);
}
