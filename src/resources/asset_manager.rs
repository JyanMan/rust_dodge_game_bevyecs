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
// pub type TextureId = usize;

#[derive(Clone, Copy, Default)]
#[repr(usize)]
pub enum TextureId {
    #[default]
    Player,
    TileAtlas,
    Zombie,
    SteelSword,
    ZombieArm,
    HealthBar
}
impl TextureId {
    pub fn as_usize(self) -> usize {
        self as usize
    } 
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub enum FontId {
    #[default]
    OpenSansBold
}

pub struct AssetManager <'a> {
    pub open_sans_bold: Rc<Font<'a, 'a>>,
    pub fonts_map: HashMap<String, Texture<'a>>,
    pub text_texture_set: SparseSet<TextId, Texture<'a>, VecStorage<TextId>>,
    pub texture_set: SparseSet<usize, Texture<'a>, VecStorage<usize>>,
    pub ttf_ctx: &'a Sdl2TtfContext,
    pub t_creator: &'a TextureCreator<WindowContext>
}

impl <'a> AssetManager <'a> {

    pub fn new_texture(
        texture_set: &mut SparseSet<usize, Texture<'a>, VecStorage<usize>>,
        t_creator: &'a TextureCreator<WindowContext>,
        path: &str,
        id: TextureId
    ) {
        let player_t = t_creator.load_texture(path).unwrap();
        texture_set.insert(id.as_usize(), player_t );
    }
    
    pub fn new(t_creator: &'a TextureCreator<WindowContext>, ttf_ctx: &'a Sdl2TtfContext) -> Self {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
        // let t_creator = canvas.texture_creator();

        let mut texture_set: SparseSet<usize, Texture<'a>, VecStorage<usize>> = SparseSet::default();
        
        Self::new_texture(&mut texture_set, t_creator, "assets/player.png", TextureId::Player);
        Self::new_texture(&mut texture_set, t_creator, "assets/zombie.png", TextureId::Zombie);
        Self::new_texture(&mut texture_set, t_creator, "assets/tile_atlas.png", TextureId::TileAtlas);
        Self::new_texture(&mut texture_set, t_creator, "assets/steel_sword.png", TextureId::SteelSword);
        Self::new_texture(&mut texture_set, t_creator, "assets/zombie_arm.png", TextureId::ZombieArm);
        Self::new_texture(&mut texture_set, t_creator, "assets/health_bar.png", TextureId::HealthBar);
        
        let open_sans_bold = Rc::new(ttf_ctx.load_font("assets/fonts/OpenSans-Bold.ttf", 18).unwrap());

        Self {
            open_sans_bold,
            fonts_map: HashMap::new(),
            text_texture_set: SparseSet::default(),
            texture_set,
            ttf_ctx,
            t_creator
        }
    }

    pub fn get_texture(&'a self, id: TextureId) -> &'a Texture<'a> {
        self.texture_set.get(id.as_usize()).unwrap()
    }
}

