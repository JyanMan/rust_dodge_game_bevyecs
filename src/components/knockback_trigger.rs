use bevy_ecs::prelude::*;
use crate::config::*;
use crate::components::Vector2;

#[derive(Component, Default)]
pub struct KnockbackTrigger {
    pub knocked: bool,
    pub knocked_force: i32,
    pub knock_timer: f32,
    pub dir: Vector2,
}

impl KnockbackTrigger {
    pub fn trigger(&mut self, knock_force: i32, knock_dir: Vector2) {
        self.knocked_force = knock_force;
        self.knocked = true;
        self.dir = knock_dir;
    }

    pub fn timer(&mut self, delta_time: f32) -> bool {
        if !self.knocked { return false }

        self.knock_timer += delta_time;
        if self.knock_timer > KNOCK_TIME {
            self.knock_timer = 0.0;
            self.knocked = false;
            self.knocked_force = 0;
            self.dir = Vector2::zero();
            return true;
        }
        return false;
    }
}
