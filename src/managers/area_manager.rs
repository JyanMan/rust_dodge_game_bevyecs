use std::collections::HashMap;
use std::rc::Rc;
use crate::components::area::*;
use crate::math_helper::*;

#[derive(Default, Clone)]
pub struct AreaManager {
    tile_areas: HashMap<Point, Rc<Area>>,    
}

impl AreaManager {
    pub fn new() -> Self {
        Self {
            tile_areas: HashMap::new(),
        }
    }

    pub fn insert_tile_area(&mut self, tile_pos: Point, area_rc: Rc<Area>) {
        self.tile_areas.insert(tile_pos, area_rc);
    }

    pub fn remove_tile_area(&mut self, tile_pos: Point) {
        self.tile_areas.remove(&tile_pos);
    }
}
