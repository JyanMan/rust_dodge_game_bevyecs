use sdl2::EventPump;
use crate::components::register_components::*;
use crate::systems::register_systems::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;
use crate::resources::register_resources::*;

pub struct Game {
    pub ecs: ECS,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut ecs = ECS::new();

        register_all_resources(&mut ecs, renderer);
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

    pub fn input(&mut self, event: &mut EventPump) {
        self.ecs.call_input_systems(event);
    }
}
