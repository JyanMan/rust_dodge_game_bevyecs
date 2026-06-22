use sdl2::render::*;
use bevy_ecs::prelude::*;
use sdl2::pixels::*;
use sdl2::rect::*;
use std::collections::VecDeque;

use crate::resources::*;
use crate::components::*;
use crate::config::*;

pub struct SpriteParams {
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
    pub scale: Vector2,
    pub angle: f64,
    pub frame: u32,
    pub relative_to_cam: bool,
    pub pixel_perfect: bool, 
    pub flip_x: bool,
    pub flip_y: bool,
    pub texture_id: TextureId,
    pub hor: i32,
    pub vert: i32
}

impl SpriteParams {
    pub fn new(
        sprite: &Sprite,
        pos: Vector2,
        relative_to_cam: bool,
        pixel_perfect: bool,

    ) -> Self {
        Self {
            pos, scale: sprite.scale,
            angle: sprite.angle, relative_to_cam, pixel_perfect, frame: sprite.frame,
            flip_x: sprite.flip_x, flip_y: sprite.flip_y, texture_id: sprite.texture_id,
            hor: sprite.hor, vert: sprite.vert,
            width: sprite.width, height: sprite.width
        }       
    }
    pub fn angle(mut self, angle: f64) -> Self {
        self.angle = angle;
        self
    }
    pub fn frame(mut self, frame: u32) -> Self {
        self.frame = frame;
        self
    }
    pub fn scale(mut self, scale: Vector2) -> Self {
        self.scale = scale;
        self
    }
}

#[repr(usize)]
pub enum DrawLayer {
    UI,
    Normal,
    Pixelated,
    COUNT
}

pub enum DrawCommand {
    Sprite(SpriteParams),
}

impl DrawCommand {
    fn draw_sprite_params(params: &mut SpriteParams, camera: &Camera) {
        let cam_scale = camera.scale;
        let cam_pos = camera.get_pos();
        if params.relative_to_cam  {
            let half_width = Vector2::new(params.width / 2.0, params.height / 2.0);
            let pos_centered = params.pos - half_width;
            if params.pixel_perfect {

                let dif_vec = Vector2::new(
                    ((SCREEN_WIDTH as f32 / cam_scale) - (RES_WIDTH as f32)) / 2.0,
                    ((SCREEN_HEIGHT as f32 / cam_scale) - (RES_HEIGHT as f32)) / 2.0,
                );
                params.pos = pos_centered - cam_pos - dif_vec;
            }
            else {
                params.scale *= cam_scale;

                let pos_cam_adjusted = (pos_centered - cam_pos) * cam_scale;

                params.pos = pos_cam_adjusted;
            }
        }
    }
    pub fn draw(self, canvas: &mut WindowCanvas, asset_m: &AssetManager, camera: &Camera) {
        match self {
            DrawCommand::Sprite(mut params) => {


                // let width = texture.query().width;
                // let height = texture.query().height;


                Self::draw_sprite_params(&mut params, camera);
                
                let width = params.width;
                let height = params.height;
                let scale = params.scale;
                let pos = params.pos;
                let angle = params.angle;
                let frame = params.frame;

                let scale_x = scale.x.abs();
                let scale_y = scale.y.abs();

                let flip_x = if scale.x >= 0.0 {params.flip_x} else {!params.flip_x};
                let flip_y = if scale.y >= 0.0 {params.flip_y} else {!params.flip_y};

                let frame_x: i32 = width as i32 * (frame as i32 % params.hor);
                let frame_y: i32 = height as i32 * (frame as i32 / params.hor);
                let src_rect = Rect::new(
                     frame_x, frame_y, width.floor() as u32, height.floor() as u32 
                );

                let dest_rect = Rect::new(
                    pos.x.floor() as i32,         
                    pos.y.floor() as i32,
                    (width * scale_x).floor() as u32, // scale
                    (height * scale_y).floor() as u32 // scale
                );

                let texture = asset_m.get_texture(params.texture_id);

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
    }
}

// pub struct DrawParams<'a> {
//     pub canvas: &'a mut WindowCanvas,
//     // pub asset_m: &'a AssetManager,
//     pub pos: Vector2,
//     pub scale: Vector2,
//     pub angle: f64,
//     pub frame: Option<u32>,
//     pub relative_to_cam: bool,
//     pub pixel_perfect: bool, 
// }

pub struct GeometryParams<'a> {
    pub canvas: &'a mut WindowCanvas,
    pub relative_to_cam: bool,
    pub pixel_perfect: bool, 
}

#[derive(Resource)]
pub struct DrawList(Vec<VecDeque<DrawCommand>>);
impl Default for DrawList {
    fn default() -> Self {
        let mut list = Vec::new();
        for _ in 0..DrawLayer::COUNT as usize {
            list.push(VecDeque::new());
        }
        Self(list)
    }
}
impl DrawList {
    pub fn draw(&mut self, cmd: DrawCommand, layer: DrawLayer) {
        self.0[layer as usize].push_back(cmd);
    }
    pub fn get_list(&mut self, layer: DrawLayer) -> Option<&mut VecDeque<DrawCommand>> {
        self.0.get_mut(layer as usize)
    }
}

