use bevy_ecs::prelude::*;

use crate::components::*;
use crate::resources::DeltaTime;

pub fn timer(
    mut query: Query<&mut DodgeStamina>,
    dt: Res<DeltaTime>
) {
    for mut dodge_stam in &mut query {
        if dodge_stam.stack >= dodge_stam.max_stack {
            dodge_stam.timer.reset();
            continue;
        }
        if dodge_stam.timer.tick(dt.0).just_finished() {
            dodge_stam.stack = dodge_stam.max_stack;
        }
        if dodge_stam.in_between_timer.tick(dt.0).just_finished() {
           dodge_stam.in_between_timer.pause(); 
        }
    }
}
