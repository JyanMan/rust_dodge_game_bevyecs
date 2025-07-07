use std::collections::HashMap;
use crate::components::animation::*;

#[derive(Clone, Default)]
pub struct AnimationPlayer {
    playing: bool,
    curr_anim: usize,
    num_anims: i32,
    anims: Vec<Animation>
}

impl AnimationPlayer {
    pub fn new(num_anims: i32) -> Self {
        let mut anims = vec![];
        for _ in 0..num_anims {
            anims.push(Animation::default());
        }
        Self {
            playing: false,
            curr_anim: 0,
            num_anims: num_anims,
            anims: anims
        }
    }
    pub fn play(&mut self, index: usize) {
        if index == self.curr_anim {
            return;
        }
        self.anims[self.curr_anim].stop();
        self.curr_anim = index;
        self.playing = true;
    }

    pub fn update_play(&mut self, delta_time: f32) {
        if self.playing {
            self.anims[self.curr_anim].play(delta_time);
        }
    }

    pub fn add_anim(&mut self, index: usize, anim: Animation) {
        self.anims[index] = anim;
    }
}
