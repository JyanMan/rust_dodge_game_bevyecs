use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::*;
use bevy_app::prelude::*;
use static_cell::StaticCell;

use sdl2::render::*;
use sdl2::video::WindowContext;

use crate::core::renderer::*;
use crate::config::*;
use crate::resources::*;

static CANVAS: StaticCell<Canvas<sdl2::video::Window>> = StaticCell::new();
static TTF_CTX: StaticCell<Sdl2TtfContext> = StaticCell::new();
static T_CREATOR: StaticCell<TextureCreator<WindowContext>> = StaticCell::new();

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Render;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PostRender;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PreRender;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Input;

#[derive(Default)]
pub struct SDLInit;

impl Plugin for SDLInit {
    fn build(&self, app: &mut App) {

        app.init_schedule(PreRender);
        app.init_schedule(Render);
        app.init_schedule(PostRender);
        app.init_schedule(Input);
        app.init_schedule(Update);
        app.init_schedule(FixedUpdate);

        // override main schedules
        
        app.add_systems(PreRender, set_background);
        app.add_systems(PostRender, canvas_present);
        // app.add_systems(Input, exit_input_system);
    }
}

pub fn custom_runner(mut app: App) -> AppExit {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("dodge the man", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().
        present_vsync().
        build().
        unwrap();

    let ttf_ctx = sdl2::ttf::init().unwrap();
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");
    
    // let mut event_pump = sdl_context.event_pump().unwrap();

    let mut dt_accumulator = 0.0;
    let fps: f32 = 60.0;
    let time_step = 1.0 / fps;

    let mut delta_time;
    let mut last_time = timer_subsystem.performance_counter() as f32;
    let mut curr_time;

    let canvas_static: &'static mut WindowCanvas = CANVAS.init(canvas);
    app.insert_non_send_resource( Renderer::new(
        canvas_static,
        TTF_CTX.init(ttf_ctx),
        T_CREATOR.init(canvas_static.texture_creator()),
        Camera::new(),
    ));
    app.insert_non_send_resource(sdl_context.event_pump().unwrap());

    app.world_mut().run_schedule(Startup);

    // TIME STEP IS FIXED
    let mut ts_res = app.world_mut().get_resource_mut::<TimeStep>().unwrap();
    ts_res.0 = time_step;

    loop {
        curr_time = timer_subsystem.performance_counter() as f32;
        delta_time = (curr_time - last_time) / timer_subsystem.performance_frequency() as f32;
        last_time = curr_time;

        app.world_mut().run_schedule(Input);
        
        // let state = game.input(&mut event_pump);
        // if !state {
        //     break 'running;
        // }
        dt_accumulator += delta_time;
        while dt_accumulator >= time_step {
            dt_accumulator -= time_step;
            app.world_mut().run_schedule(FixedUpdate);
        }
        let mut dt_res = app.world_mut().get_resource_mut::<DeltaTime>().unwrap();
        dt_res.0 = delta_time;
        app.world_mut().run_schedule(Update);

        app.world_mut().run_schedule(PreRender);
        app.world_mut().run_schedule(Render);
        app.world_mut().run_schedule(PostRender);

        if let Some(exit) = app.should_exit() {
            return exit;
        }
    }
}

pub fn set_background(mut renderer: NonSendMut<Renderer>) {
    renderer.canvas.set_draw_color(Color::RGB(100, 100, 100));
    renderer.canvas.clear();
}

pub fn canvas_present(mut renderer: NonSendMut<Renderer>) {
    renderer.canvas.present();
}
