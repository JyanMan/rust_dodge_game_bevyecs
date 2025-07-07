use sdl2::keyboard::*;
use rand::*;
use crate::components::area::*;
use crate::components::animation::*;
use crate::components::animation_player::*;
use crate::components::entity_data::*;
use crate::components::position::*;
use crate::components::rigidbody::*;
use crate::components::sprite::*;
use crate::components::velocity::*;
use crate::systems::sprite_system::*;
use crate::systems::player_system::*;
use crate::systems::player_animation_system::*;
use crate::systems::physics_system::*;
use crate::systems::chunk_system::*;
use crate::systems::area_system::*;
use crate::systems::zombie_system::*;
use crate::systems::debug_system::*;
use crate::systems::animation_system::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;

pub struct Game {
    pub ecs: ECS,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut ecs = ECS::new();

        ecs.register_component::<Area>();
        ecs.register_component::<Animation>();
        ecs.register_component::<AnimationPlayer>();
        ecs.register_component::<Position>();
        ecs.register_component::<Sprite>();
        ecs.register_component::<Velocity>();
        ecs.register_component::<RigidBody>();
        ecs.register_component::<WalkerData>();

        zombie_register_components(&mut ecs);

        // STARTUP
        ecs.register_system_startup(
            Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
                player_init(ecs, renderer);
                player_animation_init(ecs, renderer);
                zombie_init(ecs, renderer);
                chunk_manager_init(ecs, renderer);
                area_manager_init(ecs, renderer);
            })
        );
        // UPDATE
        ecs.register_system_update(
            Box::new(|ecs: &mut ECS, delta_time: f32| {
                player_update(ecs, delta_time);
                player_animation_update(ecs, delta_time);
                chunk_manager_update(ecs, delta_time);
                animation_player_update(ecs, delta_time);
            })
        );
        // FIXED UPDATE 
        ecs.register_system_fixed_update(
            Box::new(|ecs: &mut ECS, time_step: f32| {
                player_fixed_update(ecs, time_step);
                physics_fixed_update(ecs, time_step);
                zombie_fixed_update(ecs, time_step);
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
        
        ecs.call_startup_systems(renderer);

        Self {
            ecs: ecs,
        }
    }   

    pub fn update(&mut self, delta_time: f32, renderer: &mut Renderer) {
        self.ecs.call_update_systems(delta_time);
        renderer.camera.update();
    }

    pub fn fixed_update(&mut self, time_step: f32) {
        self.ecs.call_fixed_update_systems(time_step);
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.ecs.call_draw_systems(renderer);
    }

    pub fn input(&mut self, k_state: &mut KeyboardState) {
        self.ecs.call_input_systems(k_state);
    }
}
