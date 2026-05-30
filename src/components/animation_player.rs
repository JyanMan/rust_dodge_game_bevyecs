use bevy_ecs::prelude::*;

use crate::components::animation::*;

#[derive(Component, Clone, Default)]
#[component(storage = "Table")]
pub struct AnimationPlayer {
    playing: bool,
    curr_anim: usize,
    num_anims: usize,
    anims: Vec<Animation>,
    pub elapsed: f32
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
            num_anims,
            anims,
            elapsed: 0.0
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

    /* false: no frame update just timer incrementation
        true: frame is incremented */
    pub fn update_timer(&mut self, delta_time: f32) -> bool {
        if self.playing {
            let curr_anim = &mut self.anims[self.curr_anim];
            let res = curr_anim.play(delta_time);
            if res {
                self.elapsed = 0.0;
            }
            else {
                self.elapsed += delta_time;
            }
            return res;
        }
        false
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
