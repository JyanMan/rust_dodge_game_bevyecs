use bevy_ecs::prelude::*;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::*;
use crate::core::renderer::*;
use crate::systems::world::*;
use crate::resources::*;
use crate::components::Vector2;
use crate::systems::*;

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
    fn register_resources(&mut self, renderer: &mut Renderer) {
        let chunk_m = ChunkManager::new(Vector2::new(0.0, 0.0), &renderer.asset_m, 3);
        self.world.insert_resource(chunk_m);

        let area_m = AreaManager::new();
        self.world.insert_resource(area_m);

        let dt_res = DeltaTime(0.0);
        self.world.insert_resource(dt_res);

        let ts_res = TimeStep(0.0);
        self.world.insert_resource(ts_res);

        let user_input_res = KeyInput::default();
        self.world.insert_resource(user_input_res);

        let mouse_input = MouseInput::default();
        self.world.insert_resource(mouse_input);
    }

    fn register_systems(&mut self, renderer: &mut Renderer) {
        self.update_sched.add_systems((
            chunk_system_update,
            player_timer_system,
            animation_player_update,
            walker_animation_update,
            weapon_system_animation_update,
            weapon_attack_timer_and_signal_update
        ));
        self.fixed_update_sched.add_systems((
            player_movement_system,
            gravity_system.after(player_movement_system),
            collision_system.after(gravity_system),
            pos_vel_update_system.after(collision_system),
            transform_update_system.after(pos_vel_update_system),
            area_update_system.after(transform_update_system),
        ));
        self.input_sched.add_systems(player_system_input);
    }

    pub fn new(renderer: &mut Renderer) -> Self {

        let mut game = Self {
            world: World::new(), 
            update_sched: Schedule::default(),
            fixed_update_sched: Schedule::default(),
            draw_sched: Schedule::default(),
            input_sched: Schedule::default(),
        };

        game.register_systems(renderer);
        game.register_resources(renderer);

        let player_e = player_spawn(&mut game.world, renderer);
        steel_sword_spawn(&mut game.world, renderer, player_e);

        game
    }   

    pub fn update(&mut self, delta_time: f32, renderer: &mut Renderer) {
        let mut delta_time_res = self.world.get_resource_mut::<DeltaTime>().unwrap();
        delta_time_res.0 = delta_time;

        self.update_sched.run(&mut self.world);

        camera_system_update(&mut self.world, renderer);
    }

    pub fn fixed_update(&mut self, time_step: f32) {
        let mut time_step_res = self.world.get_resource_mut::<TimeStep>().unwrap();
        time_step_res.0 = time_step;

        self.fixed_update_sched.run(&mut self.world)
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        chunk_system_draw(&mut self.world, renderer);
        sprite_system_draw(&mut self.world, renderer);
        // debug_draw_entity_areas(&mut self.world, renderer);
    }

    pub fn input(&mut self, event_pump: &mut EventPump) -> bool {
        let mut user_input_res = self.world.get_resource_mut::<KeyInput>().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    return false;
                },
                Event::KeyDown { keycode: Some(s), .. } => {
                    if (s == Keycode::Escape)  {
                        return false;
                    }
                    user_input_res.0.insert(s);
                },
                Event::KeyUp { keycode: Some(s), .. } => {
                    user_input_res.0.remove(&s);
                }
                _ => {}
            }
        }

        let mut mouse_input = self.world.get_resource_mut::<MouseInput>().unwrap();
        let mouse_state = event_pump.mouse_state();
        mouse_input.pos = Vector2::new(
            mouse_state.x() as f32, mouse_state.y() as f32
        );

        self.input_sched.run(&mut self.world);

        return true;
    }
}
