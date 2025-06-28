use sdl2::render::*;
use crate::core::renderer::*;
use crate::math_helper::*;
use crate::components::position::*;
use crate::components::sprite::*;
use std::rc::Rc;

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Grass = 0,
    Dirt = 1,
    Stone = 2,
    Water = 3,
    Sand = 4,
    Air = 5,
}

pub struct Tile {
    tile_pos: Point,
    world_pos: Position,
    // sprite: Rc<Sprite> ,
    tile_type: TileType,
}

impl Tile {
    pub fn new() -> Tile {
        // let mut sprite = Sprite::new(tile_atlas_t);
        // sprite.set_sprite_sheet(4, 2);
        let world_pos = Position::new(0.0, 0.0);

        Tile {
            tile_pos: world_to_tile(&world_pos),
            world_pos: world_pos,
            // sprite: sprite,
            tile_type: TileType::Grass,
        }
    }

    pub fn set(&mut self, tile_pos: Point, tile_type: TileType) {
        self.tile_pos = tile_pos.clone();
        self.world_pos = tile_to_world(tile_pos);
        self.tile_type = tile_type;
        // self.sprite.borrow_mut().frame = tile_type as i32;
        // println!("x: {}, y: {}", self.tile_pos.x, self.tile_pos.y);
    }

    pub fn draw(&self, renderer: &mut Renderer, sprite: &Sprite) {
        renderer.draw_frame_to_cam(sprite, &self.world_pos, 1.0, self.tile_type as i32);
        // self.sprite.draw(canvas, &self.world_pos, self.tile_type as i32);
    }
}
