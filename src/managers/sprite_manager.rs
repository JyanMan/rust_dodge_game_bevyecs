use sdl2::render::*;
use std::rc::Rc;
use crate::components::camera::*;
use crate::components::sprite::*;
use crate::math_helper::*;

pub struct SpriteManager <'a> {
    pub camera: Camera,
    sprites_vec: Vec<Sprite<'a>>
}

impl <'a> SpriteManager <'a> {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            sprites_vec: vec![],
        }
    }
    pub fn create_sprite(texture: Rc<Texture<'a>>) {
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas, sprite: Sprite, pos: &Vector2, frame: i32)  {
        let adjusted_pos = pos - self.camera.get_pos();
        sprite.draw(canvas, &adjusted_pos, frame);
    }
}
