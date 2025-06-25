use sdl2::render::*;
use sdl2::rect::*;
use sdl2::pixels::Color;
use std::rc::Rc;
use crate::math_helper::*;

pub struct Sprite <'a> {
    texture: Rc<Texture<'a>>,
    pub frame: i32,
    pub scale: f32,
    hor: i32,
    vert: i32,
}

impl <'a> Sprite <'a> {
    pub fn new(texture: Rc<Texture <'a>>) -> Self {
        Self {
           texture: texture,
           frame: 0,
           scale: 1.0,
           hor: 1,
           vert: 1,
        }
    }

    pub fn set_sprite_sheet(&mut self, hor: i32, vert: i32) {
        self.hor = hor;
        self.vert = vert;
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, pos: &Vector2) {
            // set the width of each frame
    // int cell_width = s->px_w / s->hor;
    // int cell_height = s->px_h / s->vert;

    // // determine which cell a frame corresponds to
    // int frame_x = (s->is_spritesheet) ? cell_width * (frame % s->hor) : 0;
    // int frame_y = (s->is_spritesheet) ? cell_height * SDL_floorf((float)frame / s->hor) : 0;

    // const SDL_Rect src_rect = (SDL_Rect) { frame_x, frame_y, cell_width, cell_height };
    // const SDL_Rect dest_rect = (SDL_Rect) { 
    //     f_roundtoint(pos.x), 
    //     f_roundtoint(pos.y), 
    //     s->width * scale,
    //     s->height * scale
    // };

        let px_w = self.texture.query().width as i32;
        let px_h = self.texture.query().height as i32;

        let cell_w = px_w / self.hor;
        let cell_h = px_h / self.vert;

        let frame_x: i32 = cell_w * (self.frame % self.hor);
        let frame_y: i32 = cell_h * (self.frame / self.hor);
        let src_rect = Rect::new(
             frame_x, frame_y, cell_w as u32, cell_h as u32 
        );

        let dest_rect = Rect::new(
            pos.x.round() as i32,         
            pos.y.round() as i32,
            cell_w as u32 * 1, // scale
            cell_h as u32 * 1 // scale
        );
        // const SDL_Rect dest_rect = (SDL_Rect) { 
        //     f_roundtoint(pos.x), 
        //     f_roundtoint(pos.y), 
        //     s->width * scale,
        //     s->height * scale
        // };

        canvas.set_draw_color(Color::WHITE);
        canvas.copy_ex(
            &*self.texture,
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

