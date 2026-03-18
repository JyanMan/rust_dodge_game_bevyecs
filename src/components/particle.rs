use bevy_ecs::prelude::*;
use std::vec::Vec;
use crate::components::*;

#[derive(Clone, Component)]
pub struct ParticleData {
    pub lifetime_sec: f32,
    pub speed: f32,
    pub timer: f32
}

// impl Particle {
//     pub fn spawn(world: &mut World, dir: Vector2, speed: f32, lifetime_sec: f32) -> Entity {
//         let init_vec = dir * speed;
//         world.spawn((
//             Particle { lifetime_sec, speed, timer: 0.0 },
//             Velocity { vec: init_vec },
//             GravityAffected(true),
//         )).id()
//     }
// }
