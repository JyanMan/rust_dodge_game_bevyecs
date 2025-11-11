use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::core::*;

pub fn render_occupied_quad(world: &mut World, renderer: &mut Renderer) {
    let e_quad_map = world.get_resource::<EntityQuadMap>().unwrap(); 
    e_quad_map.draw_occupied_cells(renderer);
}
