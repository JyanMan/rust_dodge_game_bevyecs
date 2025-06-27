use sdl2::render::*;
use crate::managers::asset_manager::*;

pub struct Renderer <'a> {
    pub canvas: WindowCanvas,
    pub asset_m: AssetManager <'a>,
}

