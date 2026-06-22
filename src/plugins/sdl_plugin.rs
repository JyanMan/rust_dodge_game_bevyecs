use sdl2::pixels::*;
use sdl2::rect::*;
use sdl2::ttf::Sdl2TtfContext;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::*;
use bevy_app::prelude::*;
use static_cell::StaticCell;

use sdl2::render::*;
use sdl2::video::WindowContext;

// use crate::core::renderer::*;
use crate::config::*;
use crate::resources::*;
use crate::sys;
use crate::components::Vector2;

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
         
        app.add_systems(PreRender, (
            sys::world::chunks::draw,
            sys::render::sprites_draw.after(sys::world::chunks::draw),
            sys::render::proc_anim_edges.after(sys::render::sprites_draw),
            // sys::debug::render_all_obb,
            // sys::render::texts_draw,
            // sys::render::health_bar_draw,
            // sys::render::dodge_stamina_draw,
        ));
        app.add_systems(Render, flush_draw);
        
        // app.add_systems(PreRender, set_background);
        // app.add_systems(PostRender, canvas_present);
        // app.add_systems(Input, exit_input_system);
    }
}

struct PixelatedCanvas<'a>(pub Texture<'a>);

pub fn custom_runner(mut app: App) -> AppExit {

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let window = video_subsystem.window("dodge the man", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        // .opengl()
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().
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

    let t_creator = T_CREATOR.init(canvas.texture_creator());

    // low resolution canvas
    let mut pixelated_canvas = t_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, RES_WIDTH, RES_HEIGHT)
        .unwrap();
    pixelated_canvas.set_scale_mode(sdl2::render::ScaleMode::Nearest);
    pixelated_canvas.set_blend_mode(BlendMode::Blend);

    
    // canvas_static.set_logical_size(320, 180).unwrap();
    // app.insert_non_send_resource( Renderer::new(
    //     t_creator,
    //     TTF_CTX.init(ttf_ctx),
    //     Camera::new(),
    // ));
    app.insert_non_send_resource(sdl_context.event_pump().unwrap());
    app.insert_non_send_resource(canvas);
    app.insert_non_send_resource(PixelatedCanvas(pixelated_canvas));
    app.insert_non_send_resource(AssetManager::new(t_creator, TTF_CTX.init(ttf_ctx)));
    // app.insert_non_send_resource(t_creator);
    app.insert_resource(Camera::new());
    app.init_resource::<DrawList>();
    // app.insert_non_send_resource(AssetManager::new(t_creator, TTF_CTX.init(ttf_ctx)));

    app.world_mut().run_schedule(Startup);

    // TIME STEP IS FIXED
    let mut ts_res = app.world_mut().get_resource_mut::<TimeStep>().unwrap();
    ts_res.0 = time_step;

    // let window_pixel_ratio = RES_WIDTH as f32 / SCREEN_WIDTH as f32;

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

        app.world_mut().run_schedule(PreRender);
        app.world_mut().run_schedule(Render);
        // app.world_mut().run_schedule(PostRender);


        if let Some(exit) = app.should_exit() {
            return exit;
        }
    }
}

fn flush_draw(
    mut canvas: NonSendMut<WindowCanvas>,
    mut pixelated_canvas: NonSendMut<PixelatedCanvas>,
    mut draw_list: ResMut<DrawList>,
    asset_m: NonSend<AssetManager>,
    camera: Res<Camera>,
) {
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();

    // let world = app.world_mut();
    canvas.with_texture_canvas(&mut pixelated_canvas.0, |texture_canvas| {
        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        texture_canvas.clear();

        for draw_cmd in draw_list.get_list(DrawLayer::Pixelated).unwrap().drain(..) {
            draw_cmd.draw(texture_canvas, &asset_m, &camera);
        }
        
    }).unwrap();
    for draw_cmd in draw_list.get_list(DrawLayer::UI).unwrap().drain(..) {
        draw_cmd.draw(&mut canvas, &asset_m, &camera);
    }
    // sys::debug::render_all_obb(world, &mut canvas);
    // sys::render::texts_draw(world, &mut canvas);
    // sys::render::health_bar_draw(world, &mut canvas);
    // sys::render::dodge_stamina_draw(world, &mut canvas);

    // ADJUST SCALE AND POSITION OF PIXELATED_CANVAS
    let cam_scale = camera.scale;
    let cam_lerped_pos = camera.get_lerped_pos();
    let cam_pos = camera.get_pos();


    let lerped_pos = (cam_lerped_pos - cam_pos) * cam_scale;

    let screen_center = Vector2::new(HALF_WIDTH_F, HALF_HEIGHT_F);
    let res_center = Vector2::new(RES_WIDTH as f32 / 2.0, RES_HEIGHT as f32 / 2.0) * cam_scale;
    canvas.copy(&pixelated_canvas.0, None, Some(Rect::new(
        // (SCREEN_WIDTH as f32 / (cam_scale*0.5)) as i32, (SCREEN_HEIGHT as f32 / (cam_scale * 0.5)) as i32,
        // -screen_center.x as i32, -screen_center.y as i32,
        (screen_center.x - res_center.x - lerped_pos.x) as i32,
        (screen_center.y - res_center.y - lerped_pos.y) as i32,
        (RES_WIDTH as f32 * cam_scale) as u32,
        (RES_HEIGHT as f32 * cam_scale) as u32
    ))).unwrap();
    // let world = app.world_mut();
    // sys::world::chunks::draw(world, &mut canvas);
    // sys::render::sprites_draw(world, &mut canvas);
    // sys::render::texts_draw(world, &mut canvas);
    // sys::render::health_bar_draw(world, &mut canvas);
    // sys::render::dodge_stamina_draw(world, &mut canvas);
    // sys::debug::render_all_obb(world, &mut canvas);
    // sys::render::proc_anim_edges(world, &mut canvas, &mut trans_list);
    canvas.present();
}

pub fn set_background(mut canvas: NonSendMut<WindowCanvas>) {
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
}

pub fn canvas_present(mut canvas: NonSendMut<WindowCanvas>) {
    canvas.present();
}
