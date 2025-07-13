use sdl2::rect::*;
use sdl2::pixels::Color;
use crate::core::renderer::*;
use crate::components::position::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Area {
    pub x: f32, pub y: f32,
    pub w: f32, pub h: f32,
    pub offset: Position,
}

#[allow(unused)]
impl Area {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            // center pos sub half bounds
            x: x - w / 2.0, 
            y: y - h / 2.0, 
            w: w, 
            h: h,
            offset: Position::new(0.0, 0.0)
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let cam_pos = renderer.camera.get_pos();
        let cam_scale = renderer.camera.scale;
        renderer.canvas.set_draw_color(Color::RED);
        let _ = renderer.canvas.draw_rect(Rect::new(
            (self.x - cam_pos.x) as i32 * cam_scale as i32, 
            (self.y - cam_pos.y) as i32 * cam_scale as i32, 
            self.w as u32 * cam_scale as u32, 
            self.h as u32 * cam_scale as u32,
        ));
    }

    pub fn update_pos(&mut self, x: f32, y: f32) {
        // center pos by subtracting half bounds
        self.x = x + self.offset.vec.x - self.w / 2.0;
        self.y = y + self.offset.vec.y - self.h / 2.0;
    }
}
