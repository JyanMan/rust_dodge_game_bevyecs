use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::EventPump;
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::config::*;
use crate::sys;
use crate::resources::*;
use crate::components::*;
use crate::plugins::*;

pub struct MainGame;

impl Plugin for MainGame {
    fn build(&self, app: &mut App) {

        app.add_plugins(EntityPlugin);
        app.add_plugins(WorldPlugin);
        app.add_plugins(WeaponPlugin);
        app.add_plugins(ProcAnimPlugin);

        app.init_resource::<DeltaTime>();
        app.init_resource::<TimeStep>();
        app.init_resource::<KeyInput>();
        app.init_resource::<MouseInput>();

        app.insert_resource(
            EntityQuadMap::new(
                Vector2::new(0.0, 0.0), RENDER_DISTANCE
            )
        );

        app.add_systems(Update, (
            // sys::render::sprite_update_trans,
            sys::anim::update_all,
            sys::entity::status::damage_over_time,
        ));

        app.add_systems(PreRender, (
            sys::world::chunks::draw,
            sys::render::sprites_draw,
            sys::render::proc_anim_edges,
            // sys::debug::render_all_obb,
            // sys::render::texts_draw,
            // sys::render::health_bar_draw,
            // sys::render::dodge_stamina_draw,
        ));
        // app.add_systems(Render, (
        //     sys::render::sprites_draw,
        //     sys::render::texts_draw.after(sys::render::sprites_draw),
        // ));
        // app.add_systems(PostRender, (
        //     sys::render::health_bar_draw,
        //     sys::render::dodge_stamina_draw,
        //     sys::debug::render_all_obb,
        //     // sys::debug::constraints,
        // ));

        app.add_systems(Input, (
            user_input,
        ));
    }
}

pub struct Test;

impl Plugin for Test {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_spawn);
    }
}

pub fn init_spawn(world: &mut World) {
    let player_e = sys::entity::player::spawn(world);
    sys::weapon::steel_sword::spawn(world, player_e);
    sys::entity::health::player::health_bar_spawn(world);
    // sys::entity::zombie::mass_spawn(world);
}


pub fn user_input(
    mut event_pump: NonSendMut<EventPump>,
    mut user_input_res: ResMut<KeyInput>,
    mut mouse_input_res: ResMut<MouseInput>,
    mut writer: MessageWriter<AppExit>
) {
    
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                writer.write(AppExit::Success);
            }
            Event::KeyDown {
                keycode: Some(s), ..
            } => {
                if s == Keycode::Escape {
                    writer.write(AppExit::Success);
                }
                user_input_res.0.insert(s);
            }
            Event::KeyUp {
                keycode: Some(s), ..
            } => {
                user_input_res.0.remove(&s);
            }
            _ => {}
        }
    }

    // let mut mouse_input = world.get_resource_mut::<MouseInput>().unwrap();
    let mouse_state = event_pump.mouse_state();
    mouse_input_res.pos = Vector2::new(mouse_state.x() as f32, mouse_state.y() as f32);
}
