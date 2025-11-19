use bevy_ecs::prelude::*;
use crate::config::*;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub immune: bool,
    pub immune_timer: f32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max: max,
            immune: false,
            immune_timer: 0.0,
        }
    }

    pub fn timer(&mut self, delta_time: f32) -> bool {
        if !self.immune { return false }

        self.immune_timer += delta_time;
        if self.immune_timer >= IMMUNE_TIME {
            self.immune_timer = 0.0;
            self.immune = false;
            return true;
        }
        false
    }

    pub fn set_immune(&mut self) {
       self.immune = true; 
    }

    pub fn hit_and_immune(&mut self, damage: i32) {
        self.current -= damage; 
        self.set_immune();
    }
}
