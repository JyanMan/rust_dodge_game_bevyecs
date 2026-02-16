use sdl2::render::*;
use bevy_ecs::prelude::*;
use sdl2::pixels::*;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::rect::*;
use std::rc::Rc;
use std::cell::RefCell;
use static_cell::StaticCell;

use crate::resources::*;
use crate::components::{ Vector2, Sprite, TextObject };

#[derive(Resource)]
pub struct Renderer <'a> {
    pub canvas: &'static mut WindowCanvas,
    pub asset_m: AssetManager <'a>,
    pub camera: Camera,
    pub alpha: f32,
    pub t_creator: &'static TextureCreator<WindowContext>,
}

static T_CREATOR: StaticCell<TextureCreator<WindowContext>> = StaticCell::new();

impl <'a> Renderer <'a> {
    
    pub fn new(
        canvas: &'static mut WindowCanvas,
        ttf_ctx: &'static Sdl2TtfContext,
        camera: Camera
    ) -> Self {
        let t_creator: &'static TextureCreator<WindowContext> = T_CREATOR.init(canvas.texture_creator());
        Self {
            canvas, 
            t_creator,
            asset_m: AssetManager::new(
                t_creator, ttf_ctx
            ),
            camera,
            alpha: 0.0,
        }
    }

    // pub fn init_textures<'a>(mut renderer: ResMut<Renderer<'static>>) {
    //     renderer.asset_m.init_textures();
    // }

    pub fn get_camera_adjusted_pos(&self, pos: Vector2) -> Vector2 {
        let cam_scale = self.camera.scale;
        (pos - self.camera.get_pos()) * cam_scale
    }

    pub fn draw(&mut self, sprite: &Sprite, pos: Vector2, scale: f32) {
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
    
    pub fn render_text(&mut self, text: &mut TextObject) {

        if text.new {
            text.set_id(self.asset_m.text_texture_set.len()); 
        }

        let id = text.id();
        
        if text.changed() {
            text.mark_unchanged();
            let part_render = self.asset_m.fonts_map.get(&FontId::OpenSansBold)
                .unwrap().render(text.content()); 
            let surface = part_render.solid(Color::RGB(255, 255, 255)).unwrap();
            let t_creator: &'static TextureCreator<WindowContext> = unsafe {
                &*(self.t_creator as *const TextureCreator<WindowContext>)
            }; 
            let new_texture = t_creator.create_texture_from_surface(surface).unwrap(); 
            self.asset_m.text_texture_set.insert(id, new_texture);
        }
        if let Some(texture) = self.asset_m.text_texture_set.get(id) {
            
            let cam_scale = self.camera.scale;
            let x_len = (text.content().len() as f32 * text.size as f32 * cam_scale).round() as u32;
            let y_len = (text.size as f32 * 2.0 * cam_scale).round() as u32;

            if text.is_relative_to_camera() {
                self.canvas.set_draw_color(Color::WHITE);
                let pos_cam_adjusted = (text.pos() - self.camera.get_pos()) * self.camera.scale;
                let dest_rect = Rect::new(
                     pos_cam_adjusted.x.round() as i32,
                     pos_cam_adjusted.y.round() as i32,
                     x_len,
                     y_len
                 );
                let _ = self.canvas.copy_ex( &texture, None, dest_rect, 0.0, None, false, false, );
                return;
            }

            let pos_cam_adjusted = text.pos() * self.camera.scale;

            self.canvas.set_draw_color(Color::WHITE);
            let dest_rect = Rect::new(
                 pos_cam_adjusted.x.round() as i32,
                 pos_cam_adjusted.y.round() as i32,
                 x_len,
                 y_len
            );
            let _ = self.canvas.copy_ex( &texture, None, dest_rect, 0.0, None, false, false, );
        }
    }

    pub fn delete_text(&mut self, text: &TextObject) {
        self.asset_m.text_texture_set.swap_remove_by_id(text.id());
    }


    // pub fn render_text(&mut self, id: FontId, str: &str, size: i32)  {
    //     let string = String::from(str);
    //     match id {
    //         FontId::OpenSansBold => {
    //             if let Some(text) = self.asset_m.fonts_map.get(&string) {
    //                 self.canvas.set_draw_color(Color::WHITE);

    //                 let x_len = str.len() as i32 * size;
    //                 let y_len = size * 2;
    //                 let dest_rect = Rect::new( 0, 20, x_len as u32, y_len as u32 );
                    
    //                 let _ = self.canvas.copy_ex( text, None, dest_rect, 0.0, None, false, false, );
    //             }
    //             else {
    //                 let part_render = self.asset_m.open_sans_bold.render(str); 
    //                 let surface = part_render.solid(Color::RGB(255, 255, 255)).unwrap();
    //                 let text = self.asset_m.t_creator.create_texture_from_surface(surface).unwrap(); 
    //                 self.asset_m.fonts_map.insert(string, text);
    //             }
    //         }
    //     }
    // }
}

