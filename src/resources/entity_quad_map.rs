use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;
use std::vec::*;

use crate::components::*;
use crate::core::*;
use crate::config::*;
use crate::math_helper::*;

#[derive(Clone)]
pub struct Cell {
    pos: Point, // chunk size
    entities: HashSet<Entity>,
    is_active: bool,
}

#[derive(Resource, Default)]
pub struct EntityQuadMap {
    render_dist: i32,
    cells_map: HashMap<Point, usize>,
    cells_arr: Vec<Cell>,
    new_cell_points: Vec<Point>,
    // obb_map: HashMap<Point, Cell>,
    center: Point,
}

impl EntityQuadMap {
    /* COPY PASTAD FROM CHUNKMANAGER */
    pub fn new(world_pos: Vector2, h_render_dist: i32) -> Self {

        // notice mul by 2, the h_render_dist is only for one side
        // meaning you mul by 2 to give space for both right and top
        // this also disallows odd number rendering
        let render_dist: i32 = h_render_dist * 2; 
        let size = render_dist * render_dist;

        let mut new_cell_points = Vec::new();
        new_cell_points.reserve_exact(size as usize);

        // init cells array with default value and size
        let cells_arr = vec![Cell {pos: Point::zero(), entities: HashSet::new(), is_active: false}; size as usize];
        let cells_map = HashMap::new();

        // init default values of
        let eqm = Self {
            render_dist: render_dist,
            cells_map: cells_map,
            // init values of cells within array
            cells_arr: cells_arr,             
            center: world_to_cell(&world_pos),
            new_cell_points: new_cell_points,
        };

        //cm.generate(world_pos.clone());

        eqm
    }

    /* COPY PASTAD FROM CHUNKMANAGER */
    pub fn generate(&mut self, world_pos: &Vector2) {

        self.center = world_to_cell(world_pos);

        // new chunk points to save unrendered new chunk positions
        self.new_cell_points.clear();

        // default all to inactive
        for chunk_rc in self.cells_arr.iter_mut() {
            chunk_rc.is_active = false;
        }

        for y in 0..self.render_dist {
            for x in 0..self.render_dist {
                // calc chunk position ---> key to chunks_map
                let new_cell_pos = Point {
                    x: self.center.x + x - self.render_dist / 2, 
                    y: self.center.y + y - self.render_dist / 2 
                };

                // set active if on chunk_map
                if let Some(index) = self.cells_map.get(&new_cell_pos) {
                    let chunk = self.cells_arr.get_mut(*index).expect("invalid index");
                    chunk.is_active = true;
                } 
                else {
                    self.new_cell_points.push(new_cell_pos);
                }
            }
        }

        for y in 0..self.render_dist {
            for x in 0..self.render_dist {
                let index = (y * self.render_dist + x) as usize;
                let chunk = self.cells_arr.get_mut(index).expect("invalid index");

                if !chunk.is_active {
                    // refer to new_points for new points to generate
                    let c_chunk_pos = self.new_cell_points.pop().expect("invalid index"); 
                                                                 // rendered chunks
                    // move chunk reference with the new point key
                    self.cells_map.remove(&chunk.pos);
                    self.cells_map.insert(c_chunk_pos.clone(), index);

                    let c_world_pos = cell_to_world(&c_chunk_pos);
                    // println!("aream size: {}", area_m.tile_areas.len());
                    // chunk.set(c_world_pos, area_m);
                    chunk.pos = c_chunk_pos;
                }
            }
        }
    }
    pub fn obb_entities_at(&self, cell_pos: Point) -> Option<impl Iterator<Item = &Entity>> {
        if let Some(index) = self.cells_map.get(&cell_pos) {
            Some(self.cells_arr[*index].entities.iter())
        }
        else { None }
    }

    pub fn update_cells() {
        
    }

    pub fn update_entity_cell(&mut self, e: Entity, trans: Transform, prev_cell_pos: &mut CellPos) {
        let curr_cell_pos = world_to_cell(&trans.global); 

        if prev_cell_pos.0 != curr_cell_pos {
            if let Some(index) = self.cells_map.get_mut(&prev_cell_pos.0) {
                let cell = self.cells_arr.get_mut(*index).unwrap();
                cell.entities.remove(&e);
            }

            if let Some(index) = self.cells_map.get_mut(&curr_cell_pos) {
                let cell = self.cells_arr.get_mut(*index).unwrap();
                cell.entities.insert(e);
            }

            prev_cell_pos.0 = curr_cell_pos;
        }
    }

    /* debug purposes */
    pub fn draw_occupied_cells(&self, renderer: &mut Renderer) {
        use sdl2::rect::*;
        use sdl2::pixels::Color;

        for (point, index) in self.cells_map.iter() {
            let cell = self.cells_arr.get(*index).unwrap();
            if cell.entities.len() == 0 {
                continue;
            }
            let world_pos = cell_to_world(&point);
            let cam_adjusted_pos = renderer.get_camera_adjusted_pos(world_pos);
            let cam_scale = renderer.camera.scale.round() as i32;
            let cell_rect = Rect::new(
                cam_adjusted_pos.x.round() as i32, 
                cam_adjusted_pos.y.round() as i32, 
                (CELL_SIZE * cam_scale) as u32, 
                (CELL_SIZE * cam_scale) as u32, 
            );
            renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
            let _ = renderer.canvas.draw_rect(cell_rect);
            // println!("entity_quad_map.rs: drawn at point: {} {}", point.x, point.y);
        }
    }
}
