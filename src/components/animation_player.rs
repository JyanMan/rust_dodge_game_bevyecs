use bevy_ecs::prelude::*;

use crate::components::animation::*;

#[derive(Component, Clone, Default)]
pub struct AnimationPlayer {
    playing: bool,
    curr_anim: usize,
    num_anims: usize,
    anims: Vec<Animation>
}

impl AnimationPlayer {
    pub fn new(num_anims: usize) -> Self {
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
        assert!(index < self.num_anims);

        if index == self.curr_anim && self.playing {
            return;
        }

        self.anims[self.curr_anim].stop();
        self.curr_anim = index;
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.anims[self.curr_anim].stop();
    }

    pub fn update_timer(&mut self, delta_time: f32) {
        if self.playing {
            self.anims[self.curr_anim].play(delta_time);
        }
    }

    pub fn curr_anim(&self) -> &Animation {
        &self.anims[self.curr_anim] 
    }

    pub fn add_anim(&mut self, index: usize, anim: Animation) {
        assert!(index < self.num_anims);

        self.anims[index] = anim;
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }
}
