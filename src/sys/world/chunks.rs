use sdl2::render::*;
use bevy_ecs::prelude::*;

use crate::resources::chunk_manager::*;
use crate::resources::area_manager::*;
use crate::resources::*;
use crate::config::*;
use crate::components::*;

pub fn generate(
    mut chunk_m: ResMut<ChunkManager>, 
    mut area_m: ResMut<AreaManager>, 
    query: Query<&Transform, With<PlayerTag>>
){
    for transform in &query {
        chunk_m.generate(&transform.pos, &mut area_m);
    }
}

// pub fn chunk_system_draw(world: &mut World, renderer: &mut Renderer) {
//     let mut chunk_m = world.get_resource_mut::<ChunkManager>().unwrap();
//     chunk_m.draw(renderer);
// }
pub fn init(asset_m: NonSend<AssetManager>, mut commands: Commands) {
    let chunk_m = ChunkManager::new(Vector2::new(0.0, 0.0), &asset_m, RENDER_DISTANCE);
    commands.insert_resource(chunk_m);
    println!("STARTED chunk manager");
   
}

// pub fn draw(
//     mut canvas: NonSendMut<WindowCanvas>,
//     camera: Res<Camera>,
//     asset_m: NonSend<AssetManager>,
//     mut chunk_m: ResMut<ChunkManager>
    
// ) {
//     chunk_m.draw(&mut canvas, &asset_m, &camera);
// }
pub fn draw(mut chunk_m: ResMut<ChunkManager>, mut draw_list: ResMut<DrawList>) {
    chunk_m.draw(&mut draw_list);
}
