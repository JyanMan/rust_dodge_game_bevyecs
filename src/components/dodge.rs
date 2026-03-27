use bevy_ecs::prelude::*;
use crate::components::timer::*;

const DODGE_REGEN_DUR: f32 = 1.0;

#[derive(Component)]
pub struct DodgeStamina {
    pub timer: Timer,
    pub in_between_timer: Timer, // prevent holding dodge
    pub max_stack: i32,
    pub stack: i32,
    pub regen: bool
}
impl DodgeStamina {
    pub fn new(max_stack: i32) -> Self {
        Self {
            stack: max_stack,
            max_stack,
            in_between_timer: Timer::new_paused(0.15),
            timer: Timer::new(DODGE_REGEN_DUR),
            regen: false
        }
    }

    pub fn can_dodge(&self) -> bool {
        self.in_between_timer.just_finished() &&
        self.stack > 0
    }

    pub fn use_dodge(&mut self) {
        self.stack -= 1;
        self.in_between_timer.start();
        self.regen = true;
    }

    pub fn successful_dodge(&mut self) {
        if self.regen {
            self.stack += 1;
            println!("success dodge");
            self.regen = false;
        }
    }
}
