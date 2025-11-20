use bevy_ecs::prelude::*;
use std::ptr::*;

use crate::components::Vector2;

#[allow(dead_code)]
#[derive(Clone)]
pub enum AnimData {
    SpriteFrame { value: i32, target: Entity },
    OBBOffset { offset: Vector2, target: Entity },
    Debug { msg: String },
    OBBUpdate { target: Entity }
}

#[derive(Clone)]
pub struct AnimFrame {
    // pub data: Vec<AnimData>,
    pub data: Box<[AnimData]>,
    // frame: i32,
}

impl AnimFrame {
    pub fn new(data_slice: &[AnimData]) -> Self {
        let data = data_slice.to_vec().into_boxed_slice();
        Self { data }
    }
}

#[derive(Clone, Default)]
pub struct Animation {
    frames: Box<[AnimFrame]>,
    frame_num: usize,
    // playing: bool,
    s_per_frame: f32,
    curr_frame: usize,
    play_timer: f32,
}

impl Animation {
    pub fn new(s_per_frame: f32, frame_slice: &[AnimFrame]) -> Self {
        let frames = frame_slice.to_vec().into_boxed_slice();
        Self {
            frame_num: frames.len(),
            frames,
            s_per_frame,
            curr_frame: 0,
            play_timer: 0.0,
        }
    }

    pub fn stop(&mut self) {
        self.play_timer = 0.0;
        self.curr_frame = 0;
    }

    pub fn play(&mut self, delta_time: f32) -> bool {
        // self play timer is zero if a new animation is played
        if self.frame_num == 0 { return false; }
        
        // consider updated if a new animation is set (play_timer == 0)
        let mut updated = self.play_timer == 0.0;

        while self.play_timer >= self.s_per_frame {
            self.curr_frame = (self.curr_frame + 1) % (self.frame_num);
            self.play_timer -= self.s_per_frame;
            updated = true;
        }

        self.play_timer += delta_time;

        updated
    }

    pub fn curr_frame(&self) -> &AnimFrame { &self.frames[self.curr_frame] }
}
