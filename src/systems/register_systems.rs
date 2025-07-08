use sdl2::keyboard::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;
use super::animation::*;
// use super::debug::*;
use super::entity::zombie::*;
use super::entity::player::*;
use super::physics::*;
use super::render::*;
use super::world::*;

pub fn register_all_systems(ecs: &mut ECS, renderer: &mut Renderer) {
    // STARTUP
    ecs.register_system_startup(
        Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
            player_init(ecs, renderer);
            player_animation_init(ecs, renderer);
            zombie_init(ecs, renderer);
            zombie_animation_init(ecs, renderer);
            chunk_manager_init(ecs, renderer);
            area_manager_init(ecs, renderer);
        })
    );
    // UPDATE
    ecs.register_system_update(
        Box::new(|ecs: &mut ECS, delta_time: f32| {
            player_update(ecs, delta_time);
            //walker_animation_update(ecs, delta_time);
            chunk_manager_update(ecs, delta_time);
            walker_animation_update(ecs, delta_time);
            animation_player_update(ecs, delta_time);
        })
    );
    // FIXED UPDATE 
    ecs.register_system_fixed_update(
        Box::new(|ecs: &mut ECS, time_step: f32| {
            player_fixed_update(ecs, time_step);
            zombie_fixed_update(ecs, time_step);
            physics_fixed_update(ecs, time_step);
        })
    );
    // INPUT
    ecs.register_system_input(
        Box::new(|ecs: &mut ECS, k_state: &mut KeyboardState| {
            player_input(ecs, k_state);
        })
    );

    // DRAW
    ecs.register_system_draw(
        Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
            chunk_manager_draw(ecs, renderer);
            sprite_draw(ecs, renderer);
        }) 
    );
}
