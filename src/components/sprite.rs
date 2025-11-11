use sdl2::render::*;
use sdl2::rect::*;
use sdl2::pixels::Color;
use std::rc::Rc;
use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::Vector2;
use crate::resources::asset_manager::*;

#[derive(Component, Default, Clone)]
#[component(storage = "Table")]
pub struct Sprite {
    // texture: Option<Rc<Texture<'static>>>,
    texture_id: TextureId,
    pub visible: bool,
    pub scale: f32,
    hor: i32,
    vert: i32,
    pub angle: f64,
    pub px_h: i32,
    pub px_w: i32,
    pub width: f32,
    pub height: f32,
    pub frame: i32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    pub fn new(asset_m: &AssetManager, t_id: TextureId) -> Self {
        let texture = asset_m.get_texture(t_id.clone());
        let width = texture.query().width;
        let height = texture.query().height;
        Self {
            texture_id: t_id,
            visible: true,
           // texture: Some(texture),
           scale: 1.0,
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

    pub fn init(&mut self, asset_m: &AssetManager, t_id: TextureId) {
        let texture = asset_m.get_texture(t_id.clone());
        let width = texture.query().width;
        let height = texture.query().height;
        self.px_w = width as i32;
        self.px_h = height as i32;
        self.texture_id = t_id;
    }

    pub fn set_sprite_sheet(&mut self, hor: i32, vert: i32) {
        self.hor = hor;
        self.vert = vert;
        self.width = (self.px_w / self.hor) as f32;
        self.height = (self.px_h / self.vert) as f32;
    }
    pub fn draw(&self, renderer: &mut Renderer, pos: &Vector2, scale: f32) {
        self.draw_frame_angle(renderer, pos, scale, self.frame, self.angle);
    }

    pub fn draw_frame_angle(&self, renderer: &mut Renderer, pos: &Vector2, scale: f32, frame: i32, angle: f64) {

        let frame_x: i32 = self.width as i32 * (frame % self.hor);
        let frame_y: i32 = self.height as i32 * (frame / self.hor);
        let src_rect = Rect::new(
             frame_x, frame_y, self.width as u32, self.height as u32 
        );

        let dest_rect = Rect::new(
            pos.x.round() as i32,         
            pos.y.round() as i32,
            (self.width as f32 * scale).round() as u32, // scale
            (self.height as f32 * scale).round() as u32 // scale
        );

        let texture = renderer.asset_m.get_texture(self.texture_id.clone());

        renderer.canvas.set_draw_color(Color::WHITE);
        let _ = renderer.canvas.copy_ex(
            &*texture,
            src_rect,
            dest_rect,
            angle,
            None,
            self.flip_x,
            self.flip_y,
        );
    }
}

