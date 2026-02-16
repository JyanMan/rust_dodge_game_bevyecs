use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::resources::chunk_manager::*;
use crate::resources::area_manager::*;
use crate::config::*;
use crate::components::*;

pub fn chunk_system_update(
    mut chunk_m: ResMut<ChunkManager>, 
    mut area_m: ResMut<AreaManager>, 
    query: Query<&Transform, With<PlayerTag>>
){
    for transform in &query {
        chunk_m.generate(&transform.global, &mut area_m);
    }
}

// pub fn chunk_system_draw(world: &mut World, renderer: &mut Renderer) {
//     let mut chunk_m = world.get_resource_mut::<ChunkManager>().unwrap();
//     chunk_m.draw(renderer);
// }
pub fn init_chunk_manager(renderer: NonSend<Renderer>, mut commands: Commands) {
    let chunk_m = ChunkManager::new(Vector2::new(0.0, 0.0), &renderer.asset_m, RENDER_DISTANCE);
    commands.insert_resource(chunk_m);
    println!("STARTED chunk manager");
   
}

pub fn chunk_system_draw(mut chunk_m: ResMut<ChunkManager>, mut renderer: NonSendMut<Renderer>) {
    chunk_m.draw(&mut renderer);
}
