use std::any::*;

pub enum AnimData {
    Integer,
    Float,
    Bool,
}

pub struct Animation {
    data: Box<dyn Any>,
    new_data: Box<dyn Any>

}

pub struct AnimationPlayer {

}
