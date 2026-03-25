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

        app.init_resource::<AreaManager>();
        app.init_resource::<DeltaTime>();
        app.init_resource::<TimeStep>();
        app.init_resource::<KeyInput>();
        app.init_resource::<MouseInput>();

        app.insert_resource(
            EntityQuadMap::new(
                Vector2::new(0.0, 0.0), RENDER_DISTANCE
            )
        );

        app.add_systems(Startup, (
            // init_chunk_manager,
            sys::world::chunks::init,
        ));

        // use crate::systems::entity_sys;
        app.add_systems(Update, (
            sys::state_machine::update,
            sys::particle::update_timer,
            sys::entity::health::knock_timer,
            sys::entity::dodge_stamina::timer,
            sys::entity::player::timers_update,
            sys::entity::health::player::health_bar_update,
            sys::entity::hit_reaction::set_knocked_as_stunned,
            // sys::weapon::from_player_input_update,
            sys::weapon::attack_timer_and_signal_update.before(sys::weapon::anim_update),
            sys::weapon::anim_update.after(sys::weapon::attack_timer_and_signal_update),
            sys::weapon::lost_owner,
            sys::world::damage_counter::update,
            sys::world::damage_counter::despawn_update,
            sys::anim::update_all,
            // sys::anim::walker::update,
        ));
        app.add_systems(FixedPreUpdate, (
            // sys::entity::player::movement_state_update,
            // sys::entity::zombie::movement_update,
            sys::entity::zombie::state_handler,
            sys::entity::player::state_handler,
            sys::anim::walker::anim_state_handler
                // .after(sys::entity::player::movement_state_update),
        ));
        app.add_systems(FixedUpdate, (
            sys::physics::gravity,
            sys::physics::walker_collision .after(sys::physics::gravity),
            sys::physics::pos_vel_update.after(sys::physics::walker_collision),
            sys::physics::transform_update.after(sys::physics::pos_vel_update),
            sys::physics::area_update.after(sys::physics::transform_update),
            sys::physics::obb_update.after(sys::physics::transform_update),
            sys::world::entity_quad::update .after(sys::physics::gravity),
            sys::world::entity_quad::update_overlapping_obbs .after(sys::world::entity_quad::update),
            sys::entity::hit_reaction::update .after(sys::world::entity_quad::update_overlapping_obbs),
        ));
        app.add_systems(FixedPostUpdate, (
            sys::entity::health::update,
            sys::world::chunks::generate,
            sys::world::entity_quad::generate,
            sys::world::camera::update,
        ));

        app.add_systems(Render, (
            sys::world::chunks::draw .before(sys::render::sprites_draw),
            sys::render::sprites_draw,
            sys::render::health_bar_draw,
            sys::render::texts_draw.after(sys::render::sprites_draw),
            // sys::debug::render_all_obb
        ));

        app.add_systems(Input, (
            user_input,
            sys::entity::player::input_update
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
    sys::entity::zombie::mass_spawn(world);
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
