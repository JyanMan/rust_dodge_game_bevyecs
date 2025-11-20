use bevy_ecs::prelude::*;
use crate::config::*;

#[derive(Component)]
pub struct DamageCounterTimer(pub f32);


impl DamageCounterTimer {
    pub fn new() -> Self { Self(0.0) }

    pub fn timer(&mut self, delta_time: f32) {
        self.0 += delta_time;
        if self.0 >= DAMAGE_COUNTER_TIME {
            self.0 = -1.0;
        }
    }
}
