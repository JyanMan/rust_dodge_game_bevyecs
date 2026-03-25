use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Timer {
    duration: f32,
    timer: f32,
    paused: bool
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Self {
            duration, timer: 0.0, paused: false
        }
    }
    pub fn new_paused(duration: f32) -> Self {
        Self {
            duration, timer: 0.0, paused: true
        }
    }
    pub fn tick(&mut self, delta_time: f32) -> &mut Self {
        if self.paused {
            return self;
        }
        self.timer += delta_time;
        if self.timer >= self.duration {
            self.reset();
        }
        self
    }
    pub fn just_finished(&self) -> bool {
        self.timer == 0.0
    }
    pub fn reset(&mut self) {
        self.timer = 0.0;
    }
    pub fn pause(&mut self) -> &mut Self {
        self.paused = true;
        self
    }
    pub fn start(&mut self) -> &mut Self {
        self.paused = false;
        self
    }
}

