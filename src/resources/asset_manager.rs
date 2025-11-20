use sdl2::render::*;
use sdl2::image::*;
use sdl2::video::WindowContext;
use std::rc::Rc;
use sdl2::ttf::*;
use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::*;
use xsparseset::*;

use crate::core::*;

pub type TextId = usize;

#[derive(Clone, Default)]
pub enum TextureId {
    #[default]
    Player,
    TileAtlas,
    Zombie,
    SteelSword,
    HealthBar
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub enum FontId {
    #[default]
    OpenSansBold
}

pub struct AssetManager <'a> {
    player_t: Rc<Texture<'a>>,
    zombie_t: Rc<Texture<'a>>,
    tile_atlas_t: Rc<Texture<'a>>,
    steel_sword_t: Rc<Texture<'a>>,
    pub health_bar_clear_t: Rc<Texture<'a>>,
    pub open_sans_bold: Rc<Font<'a, 'a>>,
    pub fonts_map: HashMap<String, Texture<'a>>,
    pub text_set: SparseSet<TextId, Texture<'a>, VecStorage<TextId>>,
    pub ttf_ctx: &'a Sdl2TtfContext,
    pub t_creator: &'a TextureCreator<WindowContext>
}

impl <'a> AssetManager <'a> {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>, ttf_ctx: &'a Sdl2TtfContext) -> Self {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
        // let t_creator = canvas.texture_creator();

        let player_t = Rc::new(t_creator.load_texture("assets/player.png").unwrap());
        let zombie_t = Rc::new(t_creator.load_texture("assets/zombie.png").unwrap());
        let tile_atlas_t = Rc::new(t_creator.load_texture("assets/tile_atlas.png").unwrap());
        let steel_sword_t = Rc::new(t_creator.load_texture("assets/steel_sword.png").unwrap());
        let health_bar_clear_t= Rc::new(t_creator.load_texture("assets/health_bar.png").unwrap());
        
        let open_sans_bold = Rc::new(ttf_ctx.load_font("assets/fonts/OpenSans-Bold.ttf", 18).unwrap());

        Self {
            player_t,
            tile_atlas_t,
            zombie_t,
            steel_sword_t,
            health_bar_clear_t,
            open_sans_bold,
            fonts_map: HashMap::new(),
            text_set: SparseSet::default(),
            ttf_ctx,
            t_creator
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

