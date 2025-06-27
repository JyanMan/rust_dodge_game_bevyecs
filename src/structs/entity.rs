use crate::structs::player::*;

pub enum Entity <'a> {
    Player(Player <'a> ),
}

impl <'a> Entity <'a> {
    pub fn update(&mut self, delta_time: f32) {
        match self {
            Entity::Player(p) => p.update(delta_time),
        } 
    }

    pub fn fixed_update(&mut self, time_step: f32) {
        match self {
            Entity::Player(p) => p.fixed_update(time_step),
        } 
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        match self {
            Entity::Player(p) => p.draw(d),
        } 
    }
}
