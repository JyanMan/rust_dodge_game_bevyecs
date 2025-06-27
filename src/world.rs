use sdl2::render::*;
use crate::systems::asset_manager::*;
use sdl2::video::WindowContext;
use crate::components::camera::*;
// use crate::managers::chunk_manager::*;
// use crate::managers::sprite_manager::*;
use crate::math_helper::*;

pub struct World {
    // pub am: AssetManager,
    // pub cm: ChunkManager,
    // pub sm: SpriteManager,
    // pub cm: ChunkManager,
}

impl <'a> World {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>) -> Self {
        // let am = AssetManager::new(t_creator);
        // let cm = ChunkManager::new(
        //     Vector2::new(0.0, 0.0),
        //     am.get_tile_atlas_t(),
        //     4,
        // );
        // let sm = SpriteManager::new();
        Self {
            // am: am,
            // cm: cm,
            // sm: sm,
        }
    }
}
