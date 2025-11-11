use std::any::TypeId;
use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::Transform;
use crate::components::entity::*;
use crate::ecs::ecs::*;
use crate::resources::chunk_manager::*;
use crate::resources::area_manager::*;

pub fn chunk_system_update(
    mut chunk_m: ResMut<ChunkManager>, 
    mut area_m: ResMut<AreaManager>, 
    query: Query<&Transform, With<PlayerTag>>
){
    for transform in &query {
        chunk_m.generate(&transform.global, &mut area_m);
    }
}

pub fn chunk_system_draw(world: &mut World, renderer: &mut Renderer) {
    let mut chunk_m = world.get_resource_mut::<ChunkManager>().unwrap();
    chunk_m.draw(renderer);
}
