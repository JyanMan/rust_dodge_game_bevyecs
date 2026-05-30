use bevy_ecs::prelude::*;
use std::ptr::*;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::mem::Discriminant;

use crate::components::Vector2;

// TODO: use a 2d matrix for animframe data storage in animation
// #[derive(Component)]
// pub struct AnimExecutor<C: Component> {
//     anim: Vec<AnimData>,
//     _phantom: PhantomData<C>,
// }

// #[derive(Component, Default)]
// pub struct TweenAnim {
//     pub from: Option<AnimFrame>,
//     pub to: Option<AnimFrame>,
//     pub elapsed: f32,
//     pub duration: f32
// }

#[allow(dead_code)]
#[derive(Clone)]
pub enum AnimData {
    SpriteFrame { value: i32, target: Entity },
    TransformLocal { value: Vector2, target: Entity },
    SpriteAngle { value: f64, target: Entity },
    // OBBOffset { offset: Vector2, target: Entity },
    Debug { msg: String },
}

#[derive(Clone, Default)]
pub struct AnimFrames {
    // flat 2d array
    width: usize,
    height: usize,
    data: Vec<Option<AnimData>>,
}

impl AnimFrames {
    pub fn new(data_slice: &[&[AnimData]]) -> Self {
        let height = data_slice.len();
        let mut width = 0;
        let mut data_types = HashMap::new();

        for row in data_slice.iter() {
            for anim_data in row.iter() {
                let disc = std::mem::discriminant(anim_data);
                // check if contains key disc, increment width otherwise
                if let std::collections::hash_map::Entry::Vacant(e) = data_types.entry(disc) {
                    e.insert(width);
                    width += 1;
                }
            }
        }

        let mut data = vec![None; height * width];

        for (y, row) in data_slice.iter().enumerate() {
            for anim_data in row.iter() {
                let x = data_types.get(&std::mem::discriminant(anim_data)).expect("anim data type was not registered");
                data[y * width + x] = Some(anim_data.clone());  
            }
        }
        Self {
            data,
            width,
            height
        }
    }

    pub fn get_anim_data(&self, x: usize, frame_idx: usize) -> &Option<AnimData> {
        &self.data[frame_idx * self.width + x]
    }

    pub fn get_frame(&self, frame_idx: usize) -> &[Option<AnimData>] {
        let y = frame_idx * self.width;
        &self.data[y..(y + self.width)]
    }
    // pub fn get_data(&self) -> &Vec<Option<AnimData>> { &self.data }
}

#[derive(Clone, Default)]
pub struct Animation {
    frames: AnimFrames,
    pub frame_num: usize,
    // playing: bool,
    pub s_per_frame: f32,
    curr_frame: usize,
    play_timer: f32,
}

impl Animation {
    // pub fn s_per_frame(&self) -> f32 { self.s_per_frame }
    pub fn new(s_per_frame: f32, frames: AnimFrames) -> Self {
        // let frames = frame_slice.to_vec().into_boxed_slice();
        Self {
            frame_num: frames.height,
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

    pub fn anim_frames(&self) -> &AnimFrames {
        &self.frames
    }

    // pub fn anim_frame(&self, frame_idx: usize) -> &[Option<AnimData>] {
    //     self.frames.get_anim_data(frame_idx)
    // }

    pub fn curr_frame_idx(&self) -> usize {
        self.curr_frame
    }

    // pub fn curr_frame(&self) -> &AnimFrame { &self.frames[self.curr_frame] }
    // pub fn next_frame(&self) -> Option<&AnimFrame> { self.frames.get(self.curr_frame + 1) }
}
