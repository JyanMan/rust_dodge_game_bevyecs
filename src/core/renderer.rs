use sdl2::render::*;
use crate::resources::asset_manager::*;
use crate::components::camera::*;
use crate::components::{ Vector2, Sprite };

pub struct Renderer <'a> {
    pub canvas: WindowCanvas,
    pub asset_m: AssetManager <'a>,
    pub camera: Camera,
    pub alpha: f32,
}

impl <'a> Renderer <'a> {
    pub fn new(canvas: WindowCanvas, asset_m: AssetManager <'a>, camera: Camera) -> Self {
        Self {
            canvas: canvas,
            asset_m: asset_m,
            camera: camera,
            alpha: 0.0,
        }
    }

    pub fn draw(&mut self, sprite: &Sprite, pos: &Vector2, scale: f32) {
        let cam_scale = self.camera.scale;
        sprite.draw(self, &pos, scale * cam_scale);
    }

    pub fn draw_frame_to_cam(&mut self, sprite: &Sprite, pos: &Vector2, scale: f32, frame: i32) {
        let cam_scale = self.camera.scale;
        let adjusted_pos = (*pos - self.camera.get_pos()) * cam_scale;
        sprite.draw_frame(self, &adjusted_pos, scale * cam_scale, frame);
    }

    pub fn draw_to_cam(&mut self, sprite: &Sprite, pos: &Vector2, scale: f32) {
        let cam_scale = self.camera.scale;
        let adjusted_pos = (*pos - self.camera.get_pos()) * cam_scale;
        sprite.draw(self, &adjusted_pos, scale * cam_scale);
    }
}

