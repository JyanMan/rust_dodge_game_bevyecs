use std::collections::HashMap;
use crate::components::area::*;
use crate::math_helper::*;
use crate::config::*;

#[derive(Default, Clone)]
pub struct AreaManager {
    tile_areas: HashMap<Point, Area>,    
}

impl AreaManager {
    pub fn new() -> Self {
        Self {
            tile_areas: HashMap::new(),
        }
    }

    pub fn set_tile_area(&mut self, prev_pos: &Point, new_pos: &Point) {
        let new_world_pos = tile_to_world(&new_pos);
        if let Some(mut old_area) = self.tile_areas.remove(prev_pos) {
            old_area.x = new_world_pos.x;
            old_area.y = new_world_pos.y;
            self.tile_areas.insert(new_pos.clone(), old_area);
        }
        else {
            self.tile_areas.insert(new_pos.clone(), Area::new(
                new_world_pos.x, new_world_pos.y, TILE_SIZE as f32, TILE_SIZE as f32 
            ));
        }
    }

    // pub fn insert_tile_area(&mut self, tile_pos: &Point, area: Area) {
    //     self.tile_areas.insert(tile_pos.clone(), area);
    // }

    // pub fn remove_tile_area(&mut self, tile_pos: &Point) {
    //     self.tile_areas.remove(tile_pos);
    // }
}
