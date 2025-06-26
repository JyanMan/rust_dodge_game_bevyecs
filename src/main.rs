use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod game;
mod config;
mod structs;
mod systems;
mod managers;
mod components;
mod math_helper;
mod world;
mod ecs;

pub fn main() {
    sdl2::hint::set("SDL_RENDER_DRIVER", "opengl");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().
        present_vsync().
        build().
        unwrap();

    let t_creator = canvas.texture_creator();
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");

    let mut game = game::Game::new(&t_creator);

    let mut dt_accumulator = 0.0;
    let fps: f32 = 60.0;
    let time_step = 1.0 / fps;

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut delta_time;
    let mut last_time = timer_subsystem.performance_counter() as f32;
    let mut curr_time;

    'running: loop {
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        curr_time = timer_subsystem.performance_counter() as f32;
        delta_time = (curr_time - last_time) / timer_subsystem.performance_frequency() as f32;
        last_time = timer_subsystem.performance_counter() as f32;
        
        dt_accumulator += delta_time;
        while dt_accumulator >= time_step {
            dt_accumulator -= delta_time;
            game.fixed_update(time_step);
        }

        for event in event_pump.poll_iter() {
            game.input(&event);
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        game.update(delta_time);
        game.draw(&mut canvas);
        // The rest of the game loop goes here...
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
