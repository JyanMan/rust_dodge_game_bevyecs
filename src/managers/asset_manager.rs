use sdl2::render::*;
use sdl2::image::*;
use sdl2::video::WindowContext;
use std::rc::Rc;

#[derive(Clone, Default)]
pub enum TextureId {
    #[default]
    Player
}

pub struct AssetManager <'a> {
    player_t: Rc<Texture<'a>>,
    tile_atlas_t: Rc<Texture<'a>>,
}

impl <'a> AssetManager <'a> {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>) -> Self {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
        // let t_creator = canvas.texture_creator();

        let player_t = Rc::new(t_creator.load_texture("assets/player.png").unwrap());
        let tile_atlas_t = Rc::new(t_creator.load_texture("assets/tile_atlas.png").unwrap());
        Self {
            player_t: player_t,
            tile_atlas_t: tile_atlas_t,
        }
    }

    pub fn get_texture(&self, t_id: TextureId) -> Rc<Texture<'a>> {
        match t_id {
            TextureId::Player => self.player_t.clone() 
        }
    }

    pub fn get_player_t(&self) -> Rc<Texture<'a>> {
        self.player_t.clone() 
    }

    pub fn get_tile_atlas_t(&self) -> Rc<Texture<'a>> {
        self.tile_atlas_t.clone() 
    }
}

