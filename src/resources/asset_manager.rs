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

pub struct AssetManager {
    pub fonts_map: HashMap<FontId, Rc<Font<'static, 'static>>>,
    pub text_texture_set: SparseSet<TextId, Texture<'static>, VecStorage<TextId>>,
    pub texture_set: SparseSet<usize, Texture<'static>, VecStorage<usize>>,
}

impl AssetManager {

    pub fn new_texture(
        texture_set: &mut SparseSet<usize, Texture<'static>, VecStorage<usize>>,
        t_creator: &'static TextureCreator<WindowContext>,
        path: &str,
        id: TextureId
    ) {
        let player_t = t_creator.load_texture(path).unwrap();
        texture_set.insert(id.as_usize(), player_t );
    }
    
    pub fn new(t_creator: &'static TextureCreator<WindowContext>, ttf_ctx: &'static Sdl2TtfContext) -> Self {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

        let mut texture_set: SparseSet<usize, Texture<'static>, VecStorage<usize>> = SparseSet::default();

        Self::new_texture(&mut texture_set, t_creator, "assets/player.png", TextureId::Player);
        Self::new_texture(&mut texture_set, t_creator, "assets/zombie.png", TextureId::Zombie);
        Self::new_texture(&mut texture_set, t_creator, "assets/tile_atlas.png", TextureId::TileAtlas);
        Self::new_texture(&mut texture_set, t_creator, "assets/steel_sword.png", TextureId::SteelSword);
        Self::new_texture(&mut texture_set, t_creator, "assets/zombie_arm.png", TextureId::ZombieArm);
        Self::new_texture(&mut texture_set, t_creator, "assets/health_bar.png", TextureId::HealthBar);

        let mut fonts_map = HashMap::new();

        let open_sans_bold = ttf_ctx.load_font("assets/fonts/OpenSans-Bold.ttf", 18).unwrap();
        fonts_map.insert(FontId::OpenSansBold, Rc::new(open_sans_bold));

        Self {
            // open_sans_bold,
            fonts_map,
            text_texture_set: SparseSet::default(),
            texture_set,
        }
    }

    pub fn get_texture<'a>(&'a self, id: TextureId) -> &'a Texture<'a> {
        self.texture_set.get(id.as_usize()).unwrap()
    }
}

