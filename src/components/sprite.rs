use sdl2::rect::*;
use sdl2::pixels::Color;
use bevy_ecs::prelude::*;
use sdl2::render::*;

use crate::core::renderer::*;
use crate::components::Vector2;
use crate::resources::asset_manager::*;

#[derive(Component, Clone)]
#[component(storage = "Table")]
pub struct Sprite {
    // texture: Option<Rc<Texture<'static>>>,
    texture_id: TextureId,
    pub visible: bool,
    pub scale: Vector2,
    hor: i32,
    vert: i32,
    pub angle: f64,
    pub px_h: i32,
    pub px_w: i32,
    pub width: f32,
    pub height: f32,
    pub frame: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    pub fn new(asset_m: &AssetManager, t_id: TextureId) -> Self {
        let texture = asset_m.get_texture(t_id);
        let width = texture.query().width;
        let height = texture.query().height;
        Self {
            texture_id: t_id,
            visible: true,
           // texture: Some(texture),
           scale: Vector2::new(1.0, 1.0),
           hor: 1,
           vert: 1,
           angle: 0.0,
           px_w: width as i32,
           px_h: height as i32,
           width: width as f32,
           height: height as f32,
           frame: 0,
           flip_x: false,
           flip_y: false,
        }
    }

    pub fn set_sprite_sheet(&mut self, hor: i32, vert: i32) {
        self.hor = hor;
        self.vert = vert;
        self.width = (self.px_w / self.hor) as f32;
        self.height = (self.px_h / self.vert) as f32;
    }

    // pub fn draw(&self, params: DrawParams) {
    //     self.draw_frame_angle(params);
    // }

    pub fn draw(&self, params: DrawParams, asset_m: &AssetManager) {

        let scale = params.scale;
        let pos = params.pos;
        // let asset_m = params.asset_m;
        let canvas = params.canvas;
        let angle = params.angle;
        let frame = if let Some(frame) = params.frame {
            frame
        } else { self.frame };

        let scale_x = scale.x.abs();
        let scale_y = scale.y.abs();

        let flip_x = if scale.x >= 0.0 {self.flip_x} else {!self.flip_x};
        let flip_y = if scale.y >= 0.0 {self.flip_y} else {!self.flip_y};

        let frame_x: i32 = self.width as i32 * (frame as i32 % self.hor);
        let frame_y: i32 = self.height as i32 * (frame as i32 / self.hor);
        let src_rect = Rect::new(
             frame_x, frame_y, self.width as u32, self.height as u32 
        );

        let dest_rect = Rect::new(
            pos.x.round() as i32,         
            pos.y.round() as i32,
            (self.width * scale_x).round() as u32, // scale
            (self.height * scale_y).round() as u32 // scale
        );

        let texture = asset_m.get_texture(self.texture_id);

        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.copy_ex(
            texture,
            src_rect,
            dest_rect,
            angle,
            None,
            flip_x,
            flip_y,
        );
    }
}

