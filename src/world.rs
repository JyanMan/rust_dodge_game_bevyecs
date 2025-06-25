use sdl2::render::*;
use crate::systems::asset_manager::*;
use sdl2::video::WindowContext;
use crate::managers::chunk_manager::*;
use crate::math_helper::*;

pub struct World <'a> {
    pub am: AssetManager <'a>,
    pub cm: ChunkManager <'a>,
    // pub cm: ChunkManager,
}

impl <'a> World <'a> {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>) -> Self {
        let am = AssetManager::new(t_creator);
        let cm = ChunkManager::new(
            Vector2::new(0.0, 0.0),
            am.get_tile_atlas_t(),
            2,
        );
        Self {
            am: am,
            cm: cm,
        }
    }
}
