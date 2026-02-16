use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::EventPump;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::*;
use bevy_app::prelude::*;
use static_cell::StaticCell;
use std::time::Duration;

use sdl2::render::*;
use sdl2::video::WindowContext;

use crate::core::renderer::*;
use crate::config::*;
use crate::camera::*;
use crate::systems::*;
use crate::systems::world::*;
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
            init_chunk_manager,
        ));

        app.add_systems(Update, (
            // player_timer_system,
            player_timer_system,
            player_health_bar_update,
            player_weapon_signal_update,
            health_knock_timer,
            damage_counter_update,
            entity_knocked_reaction,
            weapon_attack_timer_and_signal_update.before(weapon_system_animation_update),
            weapon_system_animation_update.after(weapon_attack_timer_and_signal_update),
            // enemy_weapon_system_animation_update.before(animation_player_update),
            animation_player_update,
            weapon_lost_owner,
            damage_counter_despawn_update,
        ));
        app.add_systems(FixedUpdate, (
            player_movement_system.before(gravity_system),
            zombie_movement_system.before(gravity_system),
            //PHYSICS
            gravity_system,
            walker_collision_system.after(gravity_system),
            pos_vel_update_system.after(walker_collision_system),
            update_entity_quad_system.after(gravity_system),
            update_entity_overlapping_obbs.after(update_entity_quad_system),
            transform_update_system.after(pos_vel_update_system),
            area_update_system.after(transform_update_system),
            obb_update_system.after(transform_update_system),
            entity_hit_update.after(update_entity_overlapping_obbs),
            // steel_sword_test_overlap.after(update_entity_overlapping_obbs),
            // player_test_overlap,
            walker_animation_update.after(transform_update_system),
            chunk_system_update,
            quad_generation_system,
            health_update,
            camera_system_update,
            // player_movement_system.before(gravity_system),
        ));

        app.add_systems(Render, (
            chunk_system_draw.before(sprite_system_draw),
            sprite_system_draw,
            health_bar_system_draw,
            text_system_draw.after(sprite_system_draw),
        ));

        app.add_systems(Input, (user_input, player_system_input));
    }
}

pub struct Test;

impl Plugin for Test {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_spawn);
    }
}

pub fn init_spawn(world: &mut World) {
    let player_e = player_spawn(world);
    steel_sword_spawn(world, player_e);
    player_health_bar_spawn(world);
    zombie_init(world);
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
