use std::ptr::*;
use std::any::*;

pub trait AnimationSet {
    fn get(&mut self, name: &str) -> Option<&mut Animation>;
}

#[derive(Clone)]
pub enum AnimData {
    Integer { value: i32, target: *mut i32 },
    Float { value: f32, target: *mut f32 },
    Bool { value: bool, target: *mut bool },
}

#[derive(Clone, Default)]
pub struct AnimFrame {
    data: Vec<AnimData>,
    // frame: i32,
}

#[derive(Clone, Default)]
pub struct Animation {
    frames: Vec<AnimFrame>,
    frame_num: usize,
    // playing: bool,
    f_per_sec: f32,
    curr_frame: usize,
    play_timer: f32,
}

impl Animation {
    pub fn new(frame_num: usize, f_per_sec: f32) -> Self {
        let mut new_frames_vec = Vec::new();

        for _ in 0..frame_num {
            new_frames_vec.push(AnimFrame { data: Vec::new() });
        }

        Self {
            frame_num: frame_num,
            frames: new_frames_vec,
            // playing: false,
            f_per_sec: f_per_sec,
            curr_frame: 0,
            play_timer: 0.0,
        }
    }

    pub fn set_frame(&mut self, frame: usize, anim_data: AnimData) {
        // if at index frame there's anim_frame, insert animdata there
        if let Some(anim_frame) = self.frames.get_mut(frame) {
            anim_frame.data.push(anim_data);
        }
    }

    pub fn stop(&mut self) {
        self.play_timer = 0.0;
        self.curr_frame = 0;
    }

    pub fn play(&mut self, delta_time: f32) {
        if self.play_timer == 0.0 {
            self.update_frame();
        }

        self.play_timer += delta_time;

        if self.play_timer >= self.f_per_sec {
            self.play_timer = 0.0;

            self.curr_frame += 1;

            if self.curr_frame >= self.frame_num {
                self.curr_frame = 0;
            }
        }
    }

    fn update_frame(&mut self) {
        let anim_frame = &self.frames[self.curr_frame];
        for anim_data in anim_frame.data.iter() {
            match *anim_data {
                AnimData::Integer { value, target } => unsafe { *target = value; },
                AnimData::Float { value, target } => unsafe { *target = value; },
                AnimData::Bool { value, target } => unsafe { *target = value; },
            }
        }
    }
}
