use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::DeltaTime;
use crate::sys::world::damage_counter;

pub fn inflictor<S: Component + Clone>(
    mut query: Query<(&StatusInflictor<S>, &EntityOverlappingOBBs)>,
    mut commands: Commands
) {
    for (inflictor, e_over_obbs) in &mut query {
        for (e, _) in e_over_obbs.0.iter() {
            inflictor.inflict(*e, &mut commands);
        }
    }
}

pub fn damage_over_time(
    mut query: Query<(Entity, &mut DamageOverTime, &mut Health, &Transform), Without<Dying>>,
    dt: Res<DeltaTime>,
    mut commands: Commands
) {
    for (e, mut dot, mut health, trans) in &mut query {
        if dot.dot_timer.tick(dt.0).just_finished() {
            let damage = dot.damage.round() as i32;
            health.raw_damage(damage);
            damage_counter::spawn(&mut commands, trans.global, damage);
        }
        if dot.duration_s.tick(dt.0).just_finished() {
            commands.entity(e).remove::<DamageOverTime>(); 
        }
    }
}
