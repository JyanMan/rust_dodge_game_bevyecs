use bevy_ecs::prelude::*;
use std::vec::Vec;
use crate::components::*;
use crate::resources::*;

pub fn health_knock_timer(
    mut query: Query<(&mut Health, &mut KnockbackTrigger, &mut Velocity)>,
    delta_time: Res<DeltaTime>
) {
    for (mut health, mut knock, mut vel) in &mut query {
        health.timer(delta_time.0);

        if knock.timer(delta_time.0) {
            // knock timer ended
            vel.vec = vel.vec * 0.1;
        }
        if knock.knocked { 
            vel.vec = knock.dir * knock.knocked_force as f32;
            knock.knocked_force = (knock.knocked_force as f32 * 0.5).round() as i32;
        }
    } 
}

pub fn health_update(
    mut query: Query<(Entity, &mut Health)>,
    mut commands: Commands
) {

    for (e, health) in &mut query {
        if health.current <= 0 {
            commands.entity(e).despawn();
        }
    }
}
