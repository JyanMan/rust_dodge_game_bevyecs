use sdl2::render::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::managers::chunk::*; 
use crate::math_helper::*;
use crate::components::sprite::*;
use crate::config::*;

pub struct ChunkManager <'a> {
    chunks_map: HashMap<Point, Rc<RefCell<Chunk<'a>>>>,
    chunks_arr: Vec<Rc<RefCell<Chunk<'a>>>>,
    world_pos: Vector2,
    sprite: Rc<Sprite<'a>>
}

impl <'a> ChunkManager <'a> {
    pub fn new(world_pos: Vector2, tile_atlas_t: Rc<Texture>, render_dist: i32) -> ChunkManager {
        let mut sprite = Sprite::new(tile_atlas_t.clone());
        sprite.set_sprite_sheet(4, 2);
        let sprite_rc = Rc::new(sprite);

        // init default values of chunk manager
        let mut cm = ChunkManager {
            chunks_map:  HashMap::new(),
            // init values of chunks within array
            chunks_arr: std::iter::repeat_with(|| {
                Rc::new(
                    RefCell::new(
                        Chunk::new(world_pos.clone(), sprite_rc.clone()) 
                    )
                ) 
            })
            // notice mul by 2, you see, the render dist is only for one side
            // meaning you mul by 2 to give space for both right, left, bottom, and top
            .take(((render_dist * 2) * (render_dist * 2)) as usize)
            .collect(),
            world_pos: world_pos.clone(),
            sprite: sprite_rc,
        };

        // init values of hashmap chunks
        for chunk_rc in cm.chunks_arr.iter_mut() {
            let chunk_pos = chunk_rc.borrow().chunk_pos();
            chunk_rc.borrow_mut().is_active = true;
            cm.chunks_map.insert(chunk_pos, chunk_rc.clone());
        }

        cm.generate(world_pos.clone(), tile_atlas_t, render_dist);

        cm
    }

    pub fn generate(&mut self, world_pos: Vector2, tile_atlas_t: Rc<Texture>, render_dist: i32) {
        let mut new_points: Vec<Point> = vec![];
        self.world_pos = world_pos;

        let chunk_pos: Point = world_to_chunk(&self.world_pos);

        // default all to inactive
        for chunk_rc in self.chunks_arr.iter_mut() {
            chunk_rc.borrow_mut().is_active = false;
        }

        for y in -render_dist..render_dist {
            for x in -render_dist..render_dist {
                let n_chunk_pos = Point { x: chunk_pos.x + x, y: chunk_pos.y + y };

                if let Some(chunk_rc) = self.chunks_map.get(&n_chunk_pos) {
                    chunk_rc.borrow_mut().is_active = true;
                } 
                else {
                    new_points.push(n_chunk_pos);
                }

                // // activate within render chunks
                // if self.chunks_map.contains_key(&n_chunk_pos) {
                //     self.chunks_map[&n_chunk_pos].borrow_mut().is_active = true;
                // }
                // else {
                //     // push new chunk points to new_points
                //     // this makes sure that new chunks are not created
                //     new_points.push(n_chunk_pos);
                // }
            }
        }

        // println!("size of arr: {}, size of map: {}", self.chunks_arr.len(), self.chunks_map.len());
        for chunk_rc in self.chunks_arr.iter_mut() {
            let mut chunk = chunk_rc.borrow_mut();
            if !chunk.is_active {
                let c_chunk_pos = new_points.pop().unwrap(); // refer to new_points for new
                                                             // rendered chunks
                // move chunk reference with the new point key
                self.chunks_map.remove(&chunk.chunk_pos);
                self.chunks_map.insert(c_chunk_pos.clone(), chunk_rc.clone());

                // set new value based on pos
                // note that once you set, the chunk automatically changes its tile frames based
                // on that pos
                let c_world_pos = chunk_to_world(&c_chunk_pos);
                chunk.set(c_world_pos);
            }
        }
        // println!("chunks len: {}, chunks arr len: {}", self.chunks_map.len(), self.chunks_arr.len());
        // assert!(self.chunks_map.len() == self.chunks_arr.len());
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        for chunk_rc in self.chunks_arr.iter_mut() {
            chunk_rc.borrow_mut().draw(canvas);
        }
    }
}
