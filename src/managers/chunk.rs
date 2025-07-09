use crate::config::*;
use crate::components::position::*;
use crate::components::Vector2;
use crate::components::sprite::*;
use crate::components::tile::*;
use crate::core::renderer::*;
use crate::math_helper::*;
use crate::managers::area_manager::*;
use fastnoise_lite::*;

#[derive(Default)]
pub struct Chunk {
    world_pos: Vector2,
    pub chunk_pos: Point,
    tiles_arr: Vec<Tile>,
    max_height: i32,
    min_y: i32,
    pub is_active: bool,
    noise: FastNoiseLite,
}

fn get_perlin_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_seed(Some(1337));
    noise.set_frequency(Some(0.005));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(6));
    noise.set_fractal_gain(Some(0.4));
    noise
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Self {
            world_pos: self.world_pos.clone(),
            chunk_pos: self.chunk_pos.clone(),
            tiles_arr: self.tiles_arr.clone(),
            max_height: self.max_height.clone(),
            min_y: self.min_y.clone(),
            is_active: self.is_active.clone(),
            noise: get_perlin_noise(),
        }
    }
}

impl Chunk {
    pub fn new(world_pos: Vector2) -> Chunk {
        // init tiles_array all empty
        let tiles_arr: Vec<Tile> = std::iter::repeat_with(|| Tile::new()).
            take((CHUNK_SIZE * CHUNK_SIZE) as usize).
            collect();

        let max_height = 200;
        let min_y = -(max_height / 2);

        let chunk = Chunk {
            chunk_pos: world_to_chunk(&world_pos),
            world_pos: world_pos,
            tiles_arr: tiles_arr,
            min_y: min_y,
            max_height: max_height,
            is_active: true,
            noise: get_perlin_noise(),
        };

        // chunk.set(chunk.world_pos.clone());

        chunk
    }

    pub fn set(&mut self, world_pos: Vector2, area_m: &mut AreaManager) {

        self.chunk_pos = world_to_chunk(&world_pos);
        self.world_pos = world_pos;

        let base_x = self.world_pos.x.floor() / TILE_SIZE as f32;
        let base_y = self.world_pos.y.floor() / TILE_SIZE as f32;

        for y in 0..(CHUNK_SIZE) {
            for x in 0..(CHUNK_SIZE) {
                let tile_x = base_x + x as f32;
                let tile_y = base_y + y as f32;

                let noise_val = self.noise.get_noise_2d(tile_x, 0.0);
                let surface_y = -(noise_val * self.max_height as f32) + self.min_y as f32;

                let tile_type: TileType = {
                    if tile_y < surface_y {
                        TileType::Air
                    }
                    else if tile_y > surface_y + 8.0 {
                        TileType::Stone
                    }
                    else if tile_y > surface_y + 1.0 {
                        TileType::Dirt
                    }
                    else {
                        TileType::Grass
                    }
                }; 

                let index = (y * CHUNK_SIZE + x) as usize;
                if let Some(tile) = self.tiles_arr.get_mut(index) {
                    tile.set(Point { x: tile_x as i32, y: tile_y as i32 }, tile_type, area_m);
                }
            }
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, sprite: &Sprite) {
        for y in 0..(CHUNK_SIZE) {
            for x in 0..(CHUNK_SIZE) {
                let index = (y * CHUNK_SIZE + x) as usize;
                if let Some(tile) = self.tiles_arr.get_mut(index) {
                    tile.draw(renderer, sprite);
                }
            }
        }
    }
}
