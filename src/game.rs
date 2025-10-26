use bevy_ecs::prelude::*;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::*;
use crate::core::renderer::*;
use crate::systems::world::*;
use crate::resources::*;
use crate::components::Vector2;
use crate::systems::entity::player::*;
use crate::systems::world::*;
use crate::systems::render::*;

#[allow(dead_code)]
pub struct Game {
    pub world: World,
    update_sched: Schedule,
    fixed_update_sched: Schedule,
    draw_sched: Schedule,
    input_sched: Schedule,
}

#[allow(dead_code, unused)]
impl Game {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut world = World::new();
        let mut update_sched = Schedule::default();
        let fixed_update_sched = Schedule::default();
        let mut draw_sched = Schedule::default();
        let mut input_sched = Schedule::default();

        let chunk_m = ChunkManager::new(Vector2::new(0.0, 0.0), &renderer.asset_m, 4);
        world.insert_resource(chunk_m);

        let area_m = AreaManager::new();
        world.insert_resource(area_m);

        let dt_res = DeltaTimeRes { delta_time: 0.0 };
        world.insert_resource(dt_res);

        let user_input_res = UserInputRes::default();
        world.insert_resource(user_input_res);

        update_sched.add_systems(chunk_system_update);
        input_sched.add_systems(player_system_input);

        // renderer.camera.set_target(Vector2::new(0.0, 0.0));

        player_spawn(&mut world, renderer);

        Self { world, update_sched, fixed_update_sched, draw_sched, input_sched }
    }   

    pub fn update(&mut self, delta_time: f32, renderer: &mut Renderer) {
        self.update_sched.run(&mut self.world);

        camera_system_update(&mut self.world, renderer);
        
        let mut dt_res = self.world.get_resource_mut::<DeltaTimeRes>().unwrap();
        dt_res.delta_time = delta_time;
    }

    pub fn fixed_update(&mut self, time_step: f32) {
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        chunk_system_draw(&mut self.world, renderer);
        //self.draw_sched.run(&mut self.world);
        sprite_system_draw(&mut self.world, renderer);
    }

    pub fn input(&mut self, event_pump: &mut EventPump) -> bool {
        let mut user_input_res = self.world.get_resource_mut::<UserInputRes>().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                // Event::Quit {..} |
                Event::KeyDown { keycode: Some(s), .. } => {
                    if (s == Keycode::Escape)  {
                        return false;
                    }
                    user_input_res.k_state.insert(s);
                },
                Event::KeyUp { keycode: Some(s), .. } => {
                    user_input_res.k_state.remove(&s);
                }
                _ => {}
            }
        }

        self.input_sched.run(&mut self.world);

        return true;
    }
}
