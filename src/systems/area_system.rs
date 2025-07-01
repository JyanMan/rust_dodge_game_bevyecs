use crate::core::renderer::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::area_manager::*;

pub fn area_manager_start() -> StartFn {
    Box::new(|ecs: &mut ECS, _renderer: &mut Renderer| {
        ecs.add_resource::<AreaManager>(AreaManager::new());
        // let area_m = ecs.create_entity();
        // ecs.register_component::<AreaManager>();

        // ecs.add_component::<AreaManager>(area_m, AreaManager::new());
    })
}
