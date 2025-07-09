use crate::core::renderer::*;
use crate::components::Vector2;
use crate::ecs::ecs::*;
use crate::resources::chunk_manager::*;
use crate::resources::area_manager::*;
use crate::resources::MouseInput;

pub fn register_all_resources(ecs: &mut ECS, renderer: &mut Renderer) {
    ecs.add_resource::<ChunkManager>(ChunkManager::new(
         Vector2::new(0.0, 0.0),
         &renderer.asset_m,
         4,
    ));
    ecs.add_resource::<AreaManager>(AreaManager::new());
    ecs.add_resource::<MouseInput>(MouseInput::default());
}
