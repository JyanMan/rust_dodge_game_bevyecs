use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod game;
mod config;
// mod structs;
mod core;
mod systems;
mod resources;
mod components;
mod math_helper;
mod ecs;
mod tests;

use crate::game::*;
use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::camera::Camera;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().
        present_vsync().
        build().
        unwrap();

    let t_creator = canvas.texture_creator();
    let mut renderer = Renderer::new(
        canvas,
        AssetManager::new(&t_creator),
        Camera::new(),
    );

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");

    let mut game = Game::new(&mut renderer);

    let mut dt_accumulator = 0.0;
    let fps: f32 = 60.0;
    let time_step = 1.0 / fps;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut delta_time;
    let mut last_time = timer_subsystem.performance_counter() as f32;
    let mut curr_time;

    'running: loop {
        renderer.canvas.set_draw_color(Color::RGB(100, 100, 100));
        renderer.canvas.clear();

        curr_time = timer_subsystem.performance_counter() as f32;
        delta_time = (curr_time - last_time) / timer_subsystem.performance_frequency() as f32;
        last_time = timer_subsystem.performance_counter() as f32;
        
        dt_accumulator += delta_time;
        while dt_accumulator >= time_step {
            dt_accumulator -= delta_time;
            game.fixed_update(time_step);
        }

        let state = game.input(&mut event_pump);
        if !state {
            break 'running;
        }
        renderer.alpha = dt_accumulator / time_step;
        game.update(delta_time, &mut renderer);
        game.draw(&mut renderer);
        // The rest of the game loop goes here...
        renderer.canvas.present();
    }
}
