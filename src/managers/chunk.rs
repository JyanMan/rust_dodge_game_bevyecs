use sdl2::render::*;
use std::rc::Rc;
use crate::config::*;
use crate::systems::tile::*;
use crate::math_helper::*;
use crate::components::sprite::*;
use fastnoise_lite::*;

pub struct Chunk {
    world_pos: Vector2,
    pub chunk_pos: Point,
    tiles_arr: Vec<Tile>,
    max_height: i32,
    base_y: i32,
    pub is_active: bool,
    noise: FastNoiseLite,
}

impl Chunk {
    pub fn new(world_pos: Vector2, sprite: Rc<Sprite>) -> Chunk {
        // init tiles_array all empty
        let tiles_arr: Vec<Tile> = std::iter::repeat_with(|| {Tile::new(sprite.clone())}).
            take((CHUNK_SIZE * CHUNK_SIZE) as usize).
            collect();

        let max_height = 200;
        let base_y = -(max_height / 2);

        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::Perlin));
        noise.set_seed(Some(1337));
        noise.set_frequency(Some(0.005));
        noise.set_fractal_type(Some(FractalType::FBm));
        noise.set_fractal_octaves(Some(6));
        noise.set_fractal_gain(Some(0.4));

        let mut chunk = Chunk {
            chunk_pos: world_to_chunk(&world_pos),
            world_pos: world_pos,
            tiles_arr: tiles_arr,
            base_y: base_y,
            max_height: max_height,
            is_active: true,
            noise: noise,
        };

        chunk.set(chunk.world_pos.clone());

        chunk
    }

    pub fn set(&mut self, world_pos: Vector2) {

        self.chunk_pos = world_to_chunk(&world_pos);
        self.world_pos = world_pos;

        // set values for tile_array
        // noise.octaves = 6;
        // noise.lacunarity = 2.0f;
        // noise.gain = 0.4f;

        // fnl_state cave_noise = fnlCreateState();
        // noise.seed = 9007;
        // cave_noise.noise_type = FNL_NOISE_PERLIN;
        // cave_noise.frequency = 0.02f;
        // cave_noise.octaves = 6;
        // cave_noise.lacunarity = 2.5f;
        // cave_noise.gain = 0.1f;
        // float cave_threshold = 0.2f;
        for y in 0..(CHUNK_SIZE) {
            for x in 0..(CHUNK_SIZE) {
                let global_x = (self.world_pos.x.floor() / TILE_SIZE as f32) + x as f32;
                let global_y = (self.world_pos.y.floor() / TILE_SIZE as f32) + y as f32;

                let noise_val = self.noise.get_noise_2d(global_x, 0.0);
                let surface_y = -(noise_val * self.max_height as f32) + self.base_y as f32;

                let mut tile_type = TileType::Grass;

                if global_y < surface_y {
                    tile_type = TileType::Air;
                }
                else if global_y > surface_y + 8.0 {
                    tile_type = TileType::Stone;
                }
                else if global_y > surface_y + 1.0 {
                    tile_type = TileType::Dirt;
                }
                else if global_y >= surface_y {
                    tile_type = TileType::Grass;
                }

                if let Some(tile) = self.tiles_arr.get_mut((y * CHUNK_SIZE + x) as usize) {
                    tile.set(Point { x: global_x as i32, y: global_y as i32 }, tile_type);
                }
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        for y in 0..(CHUNK_SIZE) {
            for x in 0..(CHUNK_SIZE) {
                if let Some(tile) = self.tiles_arr.get_mut((y * CHUNK_SIZE + x) as usize) {
                    tile.draw(canvas);
                }
            }
        }
    }

    pub fn chunk_pos(&self) -> Point {
        self.chunk_pos.clone()
    } 
}
