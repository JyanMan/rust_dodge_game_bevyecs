use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::DeltaTime;
use crate::resources::asset_manager::TextureId;
use crate::core::Renderer;

pub fn spawn(
    world: &mut World,
    start_pos: Transform,
    dir: Vector2,
    speed: f32,
    lifetime_sec: f32,
    texture: TextureId,
    gravity: bool,
    scale: f32
) -> Entity {

    let init_vec = dir * speed;
    let renderer = world.get_non_send_resource::<Renderer>().expect(" asset manager non send resource was not found ");
    let mut sprite = Sprite::new(
        &renderer.asset_m,
        texture
    );
    sprite.scale = scale;
    if gravity {
        world.spawn((
            sprite,
            ParticleData { lifetime_sec, speed, timer: 0.0 },
            start_pos,
            Velocity { vec: init_vec },
            GravityAffected(true),
        )).id()
    }
    else {
        world.spawn((
            sprite,
            ParticleData { lifetime_sec, speed, timer: 0.0 },
            start_pos,
            Velocity { vec: init_vec },
        )).id()
    }
}

pub fn update_timer(
    mut query: Query<(Entity, &mut ParticleData)>,
    delta_time: Res<DeltaTime>,
    mut commands: Commands
) {
    for (e, mut particle) in &mut query {
        particle.timer += delta_time.0;
        if particle.timer >= particle.lifetime_sec {
            commands.entity(e).despawn();
        }
    }
}
