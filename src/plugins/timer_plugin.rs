use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::sys;

pub struct Timers;

impl Plugin for Timers {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            sys::weapon::attack_timer_and_signal_update,
            sys::particle::update_timer,
            sys::entity::health::knock_timer,
            sys::entity::dodge_stamina::timer,
            sys::entity::player::timers_update,
        ));
    }
}
