use sdl2::render::*;
use sdl2::rect::*;
use sdl2::pixels::Color;
use std::rc::Rc;
use crate::core::renderer::*;
use crate::managers::asset_manager::*;
use crate::math_helper::*;

#[derive(Default, Clone)]
pub struct Sprite {
    // texture: Option<Rc<Texture<'static>>>,
    texture_id: TextureId,
    pub scale: f32,
    hor: i32,
    vert: i32,
    height: i32,
    width: i32,
    frame: i32,
}

impl <'a> Sprite {
    pub fn new(texture: Rc<Texture<'a>>, t_id: TextureId) -> Self {
        let width = texture.query().width;
        let height = texture.query().height;
        Self {
            texture_id: t_id,
           // texture: Some(texture),
           scale: 1.0,
           hor: 1,
           vert: 1,
           width: width as i32,
           height: height as i32,
           frame: 0,
        }
    }

    pub fn init(&mut self, texture: Rc<Texture <'a>>, t_id: TextureId) {
        let width = texture.query().width;
        let height = texture.query().height;
        self.width = width as i32;
        self.height = height as i32;
        self.texture_id = t_id;
        // self.texture = Some(texture);
    }

    pub fn set_sprite_sheet(&mut self, hor: i32, vert: i32) {
        self.hor = hor;
        self.vert = vert;
    }
    pub fn draw(&self, renderer: &mut Renderer, pos: &Vector2) {
        self.draw_frame(renderer, pos, self.frame);
    }

    pub fn draw_frame(&self, renderer: &mut Renderer, pos: &Vector2, frame: i32) {

        let px_w = self.width as i32;
        let px_h = self.height as i32;

        let cell_w = px_w / self.hor;
        let cell_h = px_h / self.vert;

        let frame_x: i32 = cell_w * (frame % self.hor);
        let frame_y: i32 = cell_h * (frame / self.hor);
        let src_rect = Rect::new(
             frame_x, frame_y, cell_w as u32, cell_h as u32 
        );

        let dest_rect = Rect::new(
            pos.x.round() as i32,         
            pos.y.round() as i32,
            cell_w as u32 * 1, // scale
            cell_h as u32 * 1 // scale
        );

        renderer.canvas.set_draw_color(Color::WHITE);
        let texture = renderer.asset_m.get_texture(self.texture_id.clone());
        // let texture = self.texture.clone().unwrap();
        let _ = renderer.canvas.copy_ex(
            &*texture,
            src_rect,
            dest_rect,
            0.0,
            None,
            false,
            false,
        );
        // d.draw_texture_rec(&*self.texture, src_rect, pos, Color::WHITE);
    }
}

