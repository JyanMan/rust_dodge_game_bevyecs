use std::any::*;

pub enum AnimType {
    Integer(i32),
    Float(f32),
    Bool(bool),
}

pub struct AnimData {
    anim_data: AnimType,
}

pub struct AnimFrame {
    data: Vec<AnimData>,
    // frame: i32,
}

pub struct Animation {
    frames: Vec<AnimFrame>,
    frame_num: usize,
}

pub struct AnimationPlayer {
    anims: Vec<Animation>
}

impl Animation {
    pub fn new(frame_num: usize) -> Self {
        let mut new_frames_vec = Vec::new();
        new_frames_vec.reserve_exact(frame_num);
        Self {
            frame_num: frame_num,
            frames: new_frames_vec,
        }
    }

    pub fn insert_frame(&mut self, frame: usize, anim_data: AnimType) {
        let new_data = AnimData { anim_data: anim_data };

        if let Some(anim_frame) = self.frames.get_mut(frame) {
            anim_frame.data.push(new_data)
        }
        else {
            self.frames[frame] = AnimFrame {
                data: Vec::from([new_data])
            }
        }
    }
}
