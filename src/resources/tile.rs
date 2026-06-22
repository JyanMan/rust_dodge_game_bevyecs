use sdl2::render::*;

use crate::math_helper::*;
use crate::components::Vector2;
use crate::components::sprite::*;
use crate::resources::*;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Default)]
#[derive(PartialEq)]
pub enum TileType {
    #[default]
    Grass = 0,
    Dirt = 1,
    Stone = 2,
    Water = 3,
    Sand = 4,
    Air = 5,
}

#[derive(Copy, Clone, Default)]
pub struct Tile {
    tile_pos: Point,
    world_pos: Vector2,
    tile_type: TileType,

}

impl Tile {
    pub fn new() -> Tile {
        let world_pos = Vector2::new(0.0, 0.0);

        Tile {
            tile_pos: world_to_tile(&world_pos),
            world_pos,

            // sprite: sprite,
            tile_type: TileType::Grass,
        }
    }

    pub fn set(&mut self, new_tile_pos: Point, tile_type: TileType, area_m: &mut AreaManager) {
        if tile_type != TileType::Air {
            area_m.set_tile_area(&self.tile_pos, &new_tile_pos);
        }
        else {
            area_m.remove_tile(&self.tile_pos);
        }

        self.tile_pos = new_tile_pos.clone();
        self.world_pos = tile_to_world(&new_tile_pos);
        self.tile_type = tile_type;

        // self.area.x = self.world_pos.x;
        // self.area.y = self.world_pos.y;
    }

    pub fn draw(&self, draw_list: &mut DrawList, sprite: &Sprite) {
        draw_list.draw(DrawCommand::Sprite( SpriteParams {
            pos: self.world_pos, scale: Vector2::new(1.0, 1.0),
            angle: 0.0, relative_to_cam: true, pixel_perfect: true, frame: self.tile_type as u32,
            flip_x: sprite.flip_x, flip_y: sprite.flip_y, texture_id: sprite.texture_id,
            hor: sprite.hor, vert: sprite.vert,
            width: sprite.width, height: sprite.width
        } ), DrawLayer::Pixelated);
        // canvas, sprite, self.world_pos, Vector2::new(1.0, 1.0), self.tile_type as i32, 0.0
        // self.sprite.draw(canvas, &self.world_pos, self.tile_type as i32);
    }
}
