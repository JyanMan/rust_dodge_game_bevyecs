use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;
use std::vec::*;
use xsparseset::*;

use crate::components::*;
use crate::config::*;
use crate::core::*;
use crate::math_helper::*;

#[derive(Clone)]
pub struct Cell {
    pos: Point, // chunk size
    entities: HashSet<Entity>,
    is_active: bool,
}

impl Cell {
    pub fn contains_pos(&self, p: &Vector2) -> bool {
        let world_pos = cell_to_world(&self.pos);

        let min_x = world_pos.x;
        let min_y = world_pos.y;

        let max_x = world_pos.x + CELL_SIZE as f32;
        let max_y = world_pos.y + CELL_SIZE as f32;

        p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y
    }
}

type ENeighbors = SparseSet<usize, Entity, VecStorage<usize>>;

#[derive(Resource, Default)]
pub struct EntityQuadMap {
    render_dist: i32,
    cells_map: HashMap<Point, usize>,
    cells_arr: Vec<Cell>,
    new_cell_points: Vec<Point>,
    entity_neighbors: SparseSet<usize, ENeighbors, VecStorage<usize>>,
    // obb_map: HashMap<Point, Cell>,
    center: Point,
}

#[allow(dead_code)]
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
        let cells_arr = vec![
            Cell {
                pos: Point::zero(),
                entities: HashSet::new(),
                is_active: false
            };
            size as usize
        ];
        let cells_map = HashMap::new();

        // init default values of
        Self {
            render_dist,
            cells_map,
            // init values of cells within array
            cells_arr,
            center: world_to_cell(&world_pos),
            entity_neighbors: SparseSet::default(),
            new_cell_points,
        }
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
                    y: self.center.y + y - self.render_dist / 2,
                };

                // set active if on chunk_map
                if let Some(index) = self.cells_map.get(&new_cell_pos) {
                    let chunk = self.cells_arr.get_mut(*index).expect("invalid index");
                    chunk.is_active = true;
                } else {
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
                    self.cells_map.insert(c_chunk_pos, index);
                    // println!("aream size: {}", area_m.tile_areas.len());
                    // chunk.set(c_world_pos, area_m);
                    chunk.pos = c_chunk_pos;
                    chunk.entities.clear();
                }
            }
        }
    }

    // pub fn entity_in_cells(&self, e: Entity) -> &[Entity] {
    //     if let Some(neighbors) = self.entity_neighbors.get(e.index() as usize) {
    //         return neighbors.data();            
    //     }
    //     else {
    //         return &[];
    //     }
    // }

    pub fn entity_in_cells(&self, cells: &CellPos) -> Result<Vec<&Entity>> {
        let mut result = Vec::new();

        for cell_pos in &cells.0 {
            if let Some(index) = self.cells_map.get(cell_pos) {
                for e in &self.cells_arr[*index].entities {
                    result.push(e);
                }
            }
        }

        Ok(result)
    }

    pub fn obb_overlap_edge_of_cell(cell: &Cell, obb: &OBB) -> bool {
        for v in obb.get_vertices().iter() {
            if !cell.contains_pos(v) {
                return true;
            }
        }
        false
    }

    pub fn obb_get_overlapping_cells(&self, obb: &OBB) -> IntoIter<Point> {
        let mut overlapping_points: Vec<Point> = Vec::new();

        for vert in obb.get_vertices().iter() {
            let vert_cell_pos = world_to_cell(vert);
            if !overlapping_points.contains(&vert_cell_pos) {
                overlapping_points.push(vert_cell_pos);
            }
        }

        overlapping_points.into_iter()
    }

    pub fn update_entity_cell(&mut self, e: Entity, prev_cells: &mut CellPos, obb: &OBB) {
        // remove entity from previous cells
        for _ in 0..prev_cells.0.len() {
            let cell_pos = prev_cells.0.pop().unwrap();
            if let Some(index) = self.cells_map.get_mut(&cell_pos) {
                let prev_cell = self.cells_arr.get_mut(*index).unwrap();
                prev_cell.entities.remove(&e);
            }
        }

        // add the cells it currently overlaps into
        for cell_pos in self.obb_get_overlapping_cells(obb) {
            prev_cells.0.push(cell_pos);
        }

        // insert the entity within those cells
        for cell_pos in prev_cells.0.iter() {
            if let Some(index) = self.cells_map.get_mut(cell_pos) {
                let new_cell = self.cells_arr.get_mut(*index).unwrap();
                new_cell.entities.insert(e);
            }
        }

        // let neighbors = if let Some(neighbors) = self.entity_neighbors.get_mut(e.index() as usize) { neighbors }
        // else {
        //     let new_set: SparseSet<usize, Entity, VecStorage<usize>> = SparseSet::default();
        //     self.entity_neighbors.insert(e.index() as usize, new_set);
        //     self.entity_neighbors.get_mut(e.index() as usize).unwrap()
        // };
        // neighbors.clear();
        // for cell_pos in prev_cells.0.iter() {
        //     if let Some(index) = self.cells_map.get(cell_pos) {
        //         let cell = &self.cells_arr[*index];
        //         for &e in cell.entities.iter() {
        //             neighbors.insert(e.index() as usize, e);
        //         }

        //     }
        // }
    }

    /* debug purposes */
    pub fn draw_occupied_cells(&self, renderer: &mut Renderer) {
        use sdl2::pixels::Color;
        use sdl2::rect::*;

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
