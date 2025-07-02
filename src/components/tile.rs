use std::rc::Rc;
use crate::core::renderer::*;
use crate::math_helper::*;
use crate::components::area::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::managers::area_manager::*;

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

#[derive(Clone, Default)]
pub struct Tile {
    tile_pos: Point,
    world_pos: Position,
    tile_type: TileType,

}

impl Tile {
    pub fn new() -> Tile {
        let world_pos = Position::new(0.0, 0.0);

        Tile {
            tile_pos: world_to_tile(&world_pos),
            world_pos: world_pos,

            // sprite: sprite,
            tile_type: TileType::Grass,
        }
    }

    pub fn set(&mut self, tile_pos: Point, tile_type: TileType, area_m: &mut AreaManager) {
        if tile_type != TileType::Air {
            area_m.set_tile_area(&self.tile_pos, &tile_pos);
        }

        self.tile_pos = tile_pos.clone();
        self.world_pos = tile_to_world(&tile_pos);
        self.tile_type = tile_type;

        // self.area.x = self.world_pos.x;
        // self.area.y = self.world_pos.y;
    }

    pub fn draw(&self, renderer: &mut Renderer, sprite: &Sprite) {
        renderer.draw_frame_to_cam(sprite, &self.world_pos, 1.0, self.tile_type as i32);
        // self.sprite.draw(canvas, &self.world_pos, self.tile_type as i32);
    }
}
