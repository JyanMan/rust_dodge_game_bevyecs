use sdl2::render::*;
use sdl2::image::*;
use sdl2::video::WindowContext;
use std::rc::Rc;

#[derive(Clone, Default)]
pub enum TextureId {
    #[default]
    Player,
    TileAtlas,
    Zombie,
    SteelSword,
    HealthBar
}

pub struct AssetManager <'a> {
    player_t: Rc<Texture<'a>>,
    zombie_t: Rc<Texture<'a>>,
    tile_atlas_t: Rc<Texture<'a>>,
    steel_sword_t: Rc<Texture<'a>>,
    health_bar_clear_t: Rc<Texture<'a>>,
}

impl <'a> AssetManager <'a> {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>) -> Self {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
        // let t_creator = canvas.texture_creator();

        let player_t = Rc::new(t_creator.load_texture("assets/player.png").unwrap());
        let zombie_t = Rc::new(t_creator.load_texture("assets/zombie.png").unwrap());
        let tile_atlas_t = Rc::new(t_creator.load_texture("assets/tile_atlas.png").unwrap());
        let steel_sword_t = Rc::new(t_creator.load_texture("assets/steel_sword.png").unwrap());
        let health_bar_clear_t= Rc::new(t_creator.load_texture("assets/health_bar.png").unwrap());

        Self {
            player_t,
            tile_atlas_t,
            zombie_t,
            steel_sword_t,
            health_bar_clear_t
        }
    }

    pub fn get_texture(&self, t_id: TextureId) -> Rc<Texture<'a>> {
        match t_id {
            TextureId::Player => self.player_t.clone(),
            TextureId::TileAtlas => self.tile_atlas_t.clone(),
            TextureId::Zombie => self.zombie_t.clone(),
            TextureId::SteelSword => self.steel_sword_t.clone(),
            TextureId::HealthBar=> self.health_bar_clear_t.clone(),
        }
    }
}

