use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::*;
use crate::resources::AreaManager;

pub fn draw_entity_areas(
    query: Query<&Area>,
    mut renderer: NonSendMut<Renderer>
) {

    for area in &query {
        area.draw(&mut renderer);
    }
}

pub fn render_all_obb(
    query: Query<&OBB>,
    mut renderer: NonSendMut<Renderer>
) {

    for obb in &query {
        if obb.disabled {
            continue;
        }
        obb.draw(&mut renderer);
    }
}

pub fn draw_tile_areas(
    mut area_m: ResMut<AreaManager>,
    mut renderer: NonSendMut<Renderer>
) {
    area_m.draw_tile_areas(&mut renderer);
}
