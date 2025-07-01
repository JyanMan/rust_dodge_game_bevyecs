use sdl2::video::WindowContext;
use sdl2::render::*;
use crate::managers::chunk_manager::*;
use crate::managers::asset_manager::*;
use crate::components::position::*;

pub struct World {
    pub chunk_m: ChunkManager,
}

impl <'a> World <'a> {
    pub fn new(context: &'a TextureCreator<WindowContext>) -> Self {
        let mut asset_m = AssetManager::new(context);
        let chunk_m = ChunkManager::new(
            Position::new(0.0, 0.0),
            &mut asset_m,
            3
        );
        Self {
            chunk_m: chunk_m
        }
    }
}
