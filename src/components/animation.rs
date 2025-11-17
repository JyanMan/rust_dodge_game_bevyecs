use bevy_ecs::prelude::*;
use std::ptr::*;
use std::any::*;

use crate::components::{ Sprite, Vector2, OBB };
use crate::systems::*;

pub trait AnimationSet {
    fn get(&mut self, name: &str) -> Option<&mut Animation>;
}

#[derive(Clone)]
pub enum AnimData {
    //Integer { value: i32, target: *mut i32 },
    SpriteFrame { value: i32, target: Entity },
    OBBOffset { offset: Vector2, target: Entity },
    Debug { msg: String },
    OBBUpdate { target: Entity }
    // Float { value: f32, target: *mut f32 },
    // Bool { value: bool, target: *mut bool },
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

    pub fn play_timer(&self) -> f32 { self.play_timer }

    pub fn curr_frame(&self) -> &AnimFrame { &self.frames[self.curr_frame] }

    pub fn curr_frame_index(&self) -> usize { self.curr_frame }

    pub fn update_frame(world: &mut World, anim_frame: &AnimFrame) {
        for anim_data in anim_frame.data.iter() {
            match anim_data {
                AnimData::SpriteFrame { value, target } => {
                    if let Ok(mut e) = world.get_entity_mut(*target) {
                        let mut sprite = e.get_mut::<Sprite>().expect("entity does not have sprite component");
                        sprite.frame = *value;
                    }
                },
                AnimData::OBBOffset { offset, target } => {
                    if let Ok(mut e) = world.get_entity_mut(*target) {
                        let mut obb = e.get_mut::<OBB>().expect("entity does not have sprite component");
                        obb.offset = *offset;
                    }
                    // obb.compute_vertices();
                },
                AnimData::OBBUpdate { target } => {
                    // steel_sword_per_frame_update(world, *target);
                },
                AnimData::Debug{ msg } => {
                    // println!("ANIM_DEBUG: {}", msg);
                },
            }
        }
    }
}
