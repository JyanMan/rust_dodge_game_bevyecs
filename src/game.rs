use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::WindowContext;
use sdl2::render::*;
use crate::managers::asset_manager::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::components::velocity::*;
use crate::systems::sprite_system::*;
use crate::systems::player_system::*;
use crate::systems::chunk_system::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;

pub struct Game {
    pub ecs: ECS,
    // test_player: Position,
    // test_sprite: Sprite,
    // move_dir: i32,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut ecs = ECS::new();

        ecs.register_component::<Position>();
        ecs.register_component::<Sprite>();
        ecs.register_component::<Velocity>();

        // STARTUP
        ecs.register_system_startup(player_startup_system());
        ecs.register_system_startup(chunk_startup_system());

        // UPDATE
        ecs.register_system_update(player_update_system());
        ecs.register_system_update(chunk_update_system());

        // FIXED UPDATE 
        ecs.register_system_fixed_update(player_fixed_update_system());

        // INPUT
        ecs.register_system_input(player_input_system());

        // DRAW
        ecs.register_system_draw(chunk_draw_system());
        ecs.register_system_draw(sprite_draw_system());
        
        ecs.call_startup_systems(renderer);
        // let mut p_sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
        // p_sprite.set_sprite_sheet(6, 6);

        Self {
            ecs: ecs,
            // test_sprite: p_sprite,
            // test_player: Position::new(0.0, 100.0),
            // move_dir: 0
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
        // renderer.draw_to_cam(&self.test_sprite, &self.test_player);

        // if self.move_dir == 1 {
        //     self.test_player.x += 10.0;
        // }
        // if self.move_dir == -1 {
        //     self.test_player.x -= 10.0;
        // }
    }

    pub fn input(&mut self, event: &Event) {
        self.ecs.call_input_systems(event);

        // match event {
        //     Event::KeyDown { keycode: Some(Keycode::A), .. } => {
        //         self.move_dir = -1;
        //     },
        //     Event::KeyDown { keycode: Some(Keycode::D), .. } => {
        //         self.move_dir = 1;
        //     },
        //     Event::KeyUp { keycode: Some(Keycode::A), .. } => {
        //         self.move_dir = 0;
        //     },
        //     Event::KeyUp { keycode: Some(Keycode::D), .. } => {
        //         self.move_dir = 0;
        //     },
        //     _ => {}
        // }
    }
}
