use sdl2::render::*;
use bevy_ecs::prelude::*;
use sdl2::pixels::*;
use sdl2::rect::*;
use std::collections::VecDeque;
use bevy_ecs::storage::SparseSet;

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

    ) -> Self {
        Self {
            pos, scale: sprite.scale,
            angle: sprite.angle, frame: sprite.frame,
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

#[derive(Clone)]
#[repr(usize)]
pub enum DrawKind {
    Pixelated,
    RelativeToCam,
    Both,
    None,
    COUNT
}

pub struct Draw {
    pub cmd: DrawCommand,
    pub kind: DrawKind
}

pub enum DrawCommand {
    Sprite(SpriteParams),
    Geometry(GeometryParams),
    Text(TextObject),
}

impl Draw {
    fn draw_sprite_params(kind: &DrawKind, params: &mut SpriteParams, camera: &Camera) {
        let cam_scale = camera.scale;
        let cam_pos = camera.get_pos();
        match kind {
            DrawKind::Pixelated => {
                
            }            
            DrawKind::RelativeToCam => {
                let half_width = Vector2::new(params.width / 2.0, params.height / 2.0);
                let pos_centered = params.pos - half_width;
                params.scale *= cam_scale;

                let pos_cam_adjusted = (pos_centered - cam_pos) * cam_scale;

                params.pos = pos_cam_adjusted;
            }
            DrawKind::Both => {
                let half_width = Vector2::new(params.width / 2.0, params.height / 2.0);
                let pos_centered = params.pos - half_width;
                let dif_vec = Vector2::new(
                    ((SCREEN_WIDTH as f32 / cam_scale) - (RES_WIDTH as f32)) / 2.0,
                    ((SCREEN_HEIGHT as f32 / cam_scale) - (RES_HEIGHT as f32)) / 2.0,
                );
                params.pos = pos_centered - cam_pos - dif_vec;
            },
            _ => {
                
            }
        }
        // let cam_scale = camera.scale;
        // let cam_pos = camera.get_pos();
        // if params.relative_to_cam  {
        //     let half_width = Vector2::new(params.width / 2.0, params.height / 2.0);
        //     let pos_centered = params.pos - half_width;
        //     if params.pixel_perfect {

        //         let dif_vec = Vector2::new(
        //             ((SCREEN_WIDTH as f32 / cam_scale) - (RES_WIDTH as f32)) / 2.0,
        //             ((SCREEN_HEIGHT as f32 / cam_scale) - (RES_HEIGHT as f32)) / 2.0,
        //         );
        //         params.pos = pos_centered - cam_pos - dif_vec;
        //     }
        //     else {
        //         params.scale *= cam_scale;

        //         let pos_cam_adjusted = (pos_centered - cam_pos) * cam_scale;

        //         params.pos = pos_cam_adjusted;
        //     }
        // }
    }
    pub fn draw(self, canvas: &mut WindowCanvas, asset_m: &AssetManager, camera: &Camera) {
        match self.cmd {
            DrawCommand::Sprite(mut params) => {


                // let width = texture.query().width;
                // let height = texture.query().height;


                Self::draw_sprite_params(&self.kind, &mut params, camera);
                
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
            DrawCommand::Geometry(mut params) => {
                // TODO not None
                let texture = asset_m.get_texture(params.texture_id);
                let cam_scale = camera.scale;
                let cam_pos = camera.get_pos();
    
                let dif_vec = Vector2::new(
                    ((SCREEN_WIDTH as f32 / cam_scale) - (RES_WIDTH as f32)) / 2.0,
                    ((SCREEN_HEIGHT as f32 / cam_scale) - (RES_HEIGHT as f32)) / 2.0,
                );

                for v in params.vertices.iter_mut() {
                    match self.kind {
                        DrawKind::Pixelated => {
                            
                        },
                        DrawKind::RelativeToCam => {
                            v.position = FPoint::new(
                                (v.position.x - cam_pos.x) * cam_scale, 
                                (v.position.y - cam_pos.y) * cam_scale, 
                            );
                        },
                        DrawKind::Both => {
                            let adjustment = cam_pos + dif_vec;
                            v.position -= FPoint::new(adjustment.x.floor(), adjustment.y.floor());
                        },
                        _ => {}
                    }
                    // if params.relative_to_cam {
                    //     if params.pixel_perfect {
                    //         let adjustment = cam_pos + dif_vec;
                    //         v.position -= FPoint::new(adjustment.x.floor(), adjustment.y.floor());
                    //     }
                    //     else {
                    //         v.position = FPoint::new(
                    //             (v.position.x - cam_pos.x) * cam_scale, 
                    //             (v.position.y - cam_pos.y) * cam_scale, 
                    //         );
                    //     }
                    // }
                }
                canvas.render_geometry(&params.vertices, None, VertexIndices::Sequential).unwrap();
            }
            DrawCommand::Text(text) => {
                
                // if text.new {
                //     text.set_id(asset_m.text_texture_set.len()); 
                // }

                // let id = text.id();
        
                // if text.changed() {
                //     text.mark_unchanged();
                //     let part_render = asset_m.fonts_map.get(&FontId::OpenSansBold)
                //         .unwrap().render(text.content()); 
                //     let surface = part_render.solid(Color::RGB(255, 255, 255)).unwrap();
                //     let t_creator: &'static TextureCreator<WindowContext> = unsafe {
                //         &*(t_creator as *const TextureCreator<WindowContext>)
                //     }; 
                //     let new_texture = t_creator.create_texture_from_surface(surface).unwrap(); 
                //     asset_m.text_texture_set.insert(id, new_texture);
                // }
                // if let Some(texture) = asset_m.text_texture_set.get(id) {
            

                //     let cam_scale = camera.scale;
                //     let x_len = text.content().len() as f32 * text.size as f32;
                //     let y_len = text.size as f32 * 2.0;
                //     // let x_len = (text.content().len() as f32 * text.size as f32 * cam_scale).floor() as u32;
                //     // let y_len = (text.size as f32 * 2.0 * cam_scale).floor() as u32;
                //     let dest_rect = if text.is_relative_to_camera() {
                //         let pos_cam_adjusted = (text.pos() - camera.get_pos()) * camera.scale;
                //         Rect::new(
                //              pos_cam_adjusted.x.floor() as i32,
                //              pos_cam_adjusted.y.floor() as i32,
                //              (x_len * cam_scale).floor() as u32,
                //              (y_len * cam_scale).floor() as u32
                //          )
                //         // let _ = canvas.copy_ex( texture, None, dest_rect, 0.0, None, false, false, );
                //         // return;
                //     } else {
                //         let pos_cam_adjusted = text.pos();
                //         Rect::new(
                //              pos_cam_adjusted.x.floor() as i32,
                //              pos_cam_adjusted.y.floor() as i32,
                //              x_len.floor() as u32,
                //              y_len.floor() as u32
                //         )
                //     };

                //     canvas.set_draw_color(Color::WHITE);
                //     let _ = canvas.copy_ex( texture, None, dest_rect, 0.0, None, false, false, );
                // }
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


pub struct GeometryParams {
    pub vertices: Vec<Vertex>, 
    pub texture_id: TextureId,
}

impl GeometryParams {
    pub fn new(vertices: Vec<Vertex>, texture_id: TextureId) -> Self {
        Self {
            vertices, texture_id
        }
    }
}


pub struct DrawDrain<'a> {
    curr_kind: usize,
    next: bool,
    sorted_indices: &'a mut Vec<usize>,
    draw_set: &'a mut SparseSet<usize, Vec<Draw>>
}
impl <'a> DrawDrain <'a> {
    pub fn drain(draw_set: &'a mut SparseSet<usize, Vec<Draw>>, indices: &'a mut Vec<usize>) -> Self {
        indices.sort_by(|a, b| b.cmp(a));
        Self {
            draw_set,
            sorted_indices: indices,
            curr_kind: 0,
            next: true,
        }
    }
}
impl <'a>Iterator for DrawDrain<'a> {
    type Item = Draw;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next {
                self.next = false;
                self.curr_kind  = self.sorted_indices.pop()?;
            }
            let queue = self.draw_set.get_mut(self.curr_kind).expect("should have the next z_index");
            if let Some(draw) = queue.pop() {
                return Some(draw);
            }
            else {
                self.next = true;
                continue;            
            }
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Resource)]
pub struct DrawList {
    list: Vec<(SparseSet<usize, Vec<Draw>>, Vec<usize>)>
    // pixelated: Vec<VecDeque<Draw>>,
    // cam_relative: Vec<VecDeque<Draw>>,
    // both: Vec<VecDeque<Draw>>,
    // none: Vec<VecDeque<Draw>>,
}
impl Default for DrawList {
    fn default() -> Self {
        let mut list = Vec::new();
        for _ in 0..DrawKind::COUNT as usize {
            list.push((SparseSet::new(), Vec::new()));
        }
        Self {
            list 
        }
    }
}

impl DrawList {
    pub fn draw(&mut self, draw: Draw, z_index: usize) {
        let idx = draw.kind.clone() as usize;
        let (set, indices) = self.list.get_mut(idx).unwrap(); 
        set.get_or_insert_with(z_index, Vec::new).push(draw);
        indices.push(z_index);
    }
    pub fn drain<'a>(&'a mut self, kind: DrawKind) -> DrawDrain<'a> {
        let idx = kind as usize;
        let (set, indices) = self.list.get_mut(idx).unwrap(); 
        DrawDrain::drain(set, indices)
    }
}

