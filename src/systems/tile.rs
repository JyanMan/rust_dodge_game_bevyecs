use sdl2::render::*;
use crate::math_helper::*;
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
    world_pos: Vector2,
    sprite: Rc<Sprite> ,
    tile_type: TileType,
}

impl Tile {
    pub fn new(sprite: Rc<Sprite>) -> Tile {
        // let mut sprite = Sprite::new(tile_atlas_t);
        // sprite.set_sprite_sheet(4, 2);
        let world_pos = Vector2::new(0.0, 0.0);

        Tile {
            tile_pos: world_to_tile(&world_pos),
            world_pos: world_pos,
            sprite: sprite,
            tile_type: TileType::Grass,
        }
    }

    pub fn set(&mut self, tile_pos: Point, tile_type: TileType) {
        self.tile_pos = tile_pos.clone();
        self.world_pos = tile_to_world(tile_pos);
        // self.sprite.borrow_mut().frame = tile_type as i32;
        // println!("x: {}, y: {}", self.tile_pos.x, self.tile_pos.y);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.sprite.draw(canvas, &self.world_pos, self.tile_type as i32);
    }
}
