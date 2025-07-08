use sdl2::keyboard::*;
use crate::components::area::*;
use crate::components::animation::*;
use crate::components::animation_player::*;
use crate::components::walker_data::*;
use crate::components::position::*;
use crate::components::rigidbody::*;
use crate::components::sprite::*;
use crate::components::register_components::*;
use crate::systems::register_systems::*;
// use crate::components::state_machine::*;
use crate::components::velocity::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;

pub struct Game {
    pub ecs: ECS,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut ecs = ECS::new();

        //ecs.register_component::<Area>();
        //ecs.register_component::<Animation>();
        //ecs.register_component::<AnimationPlayer>();
        //ecs.register_component::<Position>();
        //ecs.register_component::<RigidBody>();
        //ecs.register_component::<Sprite>();
        //ecs.register_component::<Velocity>();
        //ecs.register_component::<WalkerData>();

        // zombie_register_components(&mut ecs);

        register_all_components(&mut ecs);
        register_all_systems(&mut ecs, renderer);
        
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
