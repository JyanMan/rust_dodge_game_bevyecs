use sdl2::pixels::*;
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
use crate::sys;

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

        app.init_schedule(PreUpdate);
        app.init_schedule(Update);
        app.init_schedule(PostUpdate);

        app.init_schedule(FixedPreUpdate);
        app.init_schedule(FixedUpdate);
        app.init_schedule(FixedPostUpdate);

        // override main schedules
        
        // app.add_systems(PreRender, set_background);
        // app.add_systems(PostRender, canvas_present);
        // app.add_systems(Input, exit_input_system);
    }
}

pub fn custom_runner(mut app: App) -> AppExit {

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let window = video_subsystem.window("dodge the man", 1280, 720)
        // .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().
        present_vsync().
        build().
        unwrap();

    let ttf_ctx = sdl2::ttf::init().unwrap();
    
    // let mut event_pump = sdl_context.event_pump().unwrap();

    let mut dt_accumulator = 0.0;
    let fps: f32 = 60.0;
    let time_step = 1.0 / fps;

    let mut delta_time;
    let mut last_time = timer_subsystem.performance_counter() as f32;
    let mut curr_time;

    // let canvas_static: &'static mut WindowCanvas = CANVAS.init(canvas);

    let t_creator = T_CREATOR.init(canvas.texture_creator());
    let mut render_target = t_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 320, 180)
        .unwrap();
    render_target.set_scale_mode(sdl2::render::ScaleMode::Nearest);

    
    // canvas_static.set_logical_size(320, 180).unwrap();
    app.insert_non_send_resource( Renderer::new(
        t_creator,
        TTF_CTX.init(ttf_ctx),
        Camera::new(),
    ));
    // let mut renderer = Renderer::new(
    //     t_creator,
    //     TTF_CTX.init(ttf_ctx),
    //     Camera::new(),
    // );
    app.insert_non_send_resource(sdl_context.event_pump().unwrap());
    // app.insert_non_send_resource(AssetManager::new(t_creator, TTF_CTX.init(ttf_ctx)));

    app.world_mut().run_schedule(Startup);

    // TIME STEP IS FIXED
    let mut ts_res = app.world_mut().get_resource_mut::<TimeStep>().unwrap();
    ts_res.0 = time_step;

    
    let mut trans_list = bevy_ecs::storage::SparseSet::new();

    // let player_e = sys::entity::player::spawn(app.world_mut(), &mut renderer);
    // // sys::weapon::steel_sword::spawn(world, player_e);
    // sys::entity::health::player::health_bar_spawn(app.world_mut(), &mut renderer);

    loop {
        curr_time = timer_subsystem.performance_counter() as f32;
        delta_time = (curr_time - last_time) / timer_subsystem.performance_frequency() as f32;
        last_time = curr_time;

        app.world_mut().run_schedule(Input);
        
        dt_accumulator += delta_time;
        while dt_accumulator >= time_step {
            dt_accumulator -= time_step;
            app.world_mut().run_schedule(FixedPreUpdate);
            app.world_mut().run_schedule(FixedUpdate);
            app.world_mut().run_schedule(FixedPostUpdate);
        }
        let mut dt_res = app.world_mut().get_resource_mut::<DeltaTime>().unwrap();
        dt_res.0 = delta_time;

        app.world_mut().run_schedule(PreUpdate);
        app.world_mut().run_schedule(Update);
        app.world_mut().run_schedule(PostUpdate);

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        canvas.with_texture_canvas(&mut render_target, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(50, 50, 50));
            texture_canvas.clear();

            // let renderer = world.get_non_send_resource_mut::<Renderer>().unwrap();
            // let mut renderer = renderer.into_inner();
            use bevy_ecs::system::*;
            let world = app.world_mut();
            // let mut system_state: SystemState<NonSendMut<Renderer>> = SystemState::new(world);
            // let mut renderer= system_state.get_mut(world);
            
            sys::world::chunks::draw(world, texture_canvas);
            sys::render::sprites_draw(world, texture_canvas);
            sys::render::texts_draw(world, texture_canvas);
            sys::render::health_bar_draw(world, texture_canvas);
            sys::render::dodge_stamina_draw(world, texture_canvas);
            sys::debug::render_all_obb(world, texture_canvas);
            sys::render::proc_anim_edges(world, texture_canvas, &mut trans_list);

            // texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
            // texture_canvas.draw_line(Point::new(25, 25), Point::new(50, 50)).unwrap();
            texture_canvas.present();
        }).unwrap();

        canvas.copy(&render_target, None, None).unwrap();
        canvas.present();



        app.world_mut().run_schedule(PreRender);
        app.world_mut().run_schedule(Render);
        app.world_mut().run_schedule(PostRender);

        if let Some(exit) = app.should_exit() {
            return exit;
        }
    }
}

// pub fn set_background(mut renderer: NonSendMut<Renderer>) {
//     renderer.canvas.set_draw_color(Color::RGB(100, 100, 100));
//     renderer.canvas.clear();
// }

// pub fn canvas_present(mut renderer: NonSendMut<Renderer>) {
//     renderer.canvas.present();
// }
