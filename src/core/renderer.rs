use sdl2::render::*;
use bevy_ecs::prelude::*;

use crate::resources::AssetManager;
use crate::resources::Camera;
use crate::components::{ Vector2, Sprite };

#[derive(Resource)]
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

    pub fn get_camera_adjusted_pos(&self, pos: Vector2) -> Vector2 {
        let cam_scale = self.camera.scale;
        (pos - self.camera.get_pos()) * cam_scale
    }

    pub fn draw(&mut self, sprite: &Sprite, pos: &Vector2, scale: f32) {
        let cam_scale = self.camera.scale;
        sprite.draw(self, &pos, scale * cam_scale);
    }

    pub fn draw_frame_to_cam(&mut self, sprite: &Sprite, pos: Vector2, scale: f32, frame: i32, angle: f64) {
        let cam_scale = self.camera.scale;

        let half_width = Vector2::new(sprite.width / 2.0, sprite.height / 2.0);
        let pos_centered = pos - half_width;

        let pos_cam_adjusted = (pos_centered - self.camera.get_pos()) * cam_scale;

        sprite.draw_frame_angle(self, &pos_cam_adjusted, scale * cam_scale, frame, angle);
    }

    pub fn draw_to_cam(&mut self, sprite: &Sprite, pos: Vector2, scale: f32) {
        self.draw_frame_to_cam(sprite, pos, scale, sprite.frame, sprite.angle);
        // let cam_scale = self.camera.scale;
        // let adjusted_pos = (pos - self.camera.get_pos()) * cam_scale;
        // sprite.draw(self, &adjusted_pos, scale * cam_scale);
    }
}

