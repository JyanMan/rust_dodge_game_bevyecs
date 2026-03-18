use rand::*;
use bevy_ecs::prelude::*;
use crate::components::*;
use crate::systems::particle_system;
use crate::resources::asset_manager::TextureId;

pub fn spawn(commands: &mut Commands, pos: Transform, dir: Vector2) {
    commands.queue(move |world: &mut World| {
        let mut rng = rand::thread_rng();
        for _ in 0..2 {

            let rng_x = rng.gen_range(0..30) as f32;
            let rng_y = rng.gen_range(-30..10) as f32;
            let ran_speed = rng.gen_range(50.. 100) as f32;

            let ang_rad = (rng_y).atan2(rng_x) + (dir.y).atan2(dir.x);

            let ran_dir = Vector2::new(ang_rad.cos(), ang_rad.sin()).normalize();
            particle_system::spawn(
                world,
                pos,
                ran_dir,
                ran_speed,
                0.2,
                TextureId::BloodParticle,
                true,
                0.5
            );       
        }
    });
}
