use sdl2::render::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::managers::chunk::*; 
use crate::math_helper::*;
use crate::components::sprite::*;
use crate::config::*;

pub struct ChunkManager  {
    chunks_map: HashMap<Point, Rc<RefCell<Chunk>>>,
    chunks_arr: Vec<Rc<RefCell<Chunk>>>,
    new_chunk_points: Vec<Point>,
    render_dist: i32,
    world_pos: Vector2,
    sprite: Rc<Sprite>
}

impl  ChunkManager  {
    pub fn new(world_pos: Vector2, tile_atlas_t: Rc<Texture<'static>>, render_dist: i32) -> ChunkManager {
        // init sprite here, all reused by chunks
        let mut sprite = Sprite::new(tile_atlas_t.clone());
        sprite.set_sprite_sheet(4, 2);
        let sprite_rc = Rc::new(sprite);

        // notice mul by 2, you see, the render dist is only for one side
        // meaning you mul by 2 to give space for both right, left, bottom, and top
        let size = render_dist * 2 * render_dist * 2;

        let mut new_chunk_points = Vec::new();
        new_chunk_points.reserve_exact(size as usize);

        // reserve chunks with default value
        let chunks_arr = std::iter::repeat_with(|| {
                Rc::new(
                    RefCell::new(
                        Chunk::new(world_pos.clone(), sprite_rc.clone()) 
                    )
                ) 
            })
            .take(size as usize)
            .collect();


        // init default values of chunk manager
        let mut cm = ChunkManager {
            render_dist: render_dist,
            chunks_map:  HashMap::new(),
            // init values of chunks within array
            chunks_arr: chunks_arr,             world_pos: world_pos.clone(),
            new_chunk_points: new_chunk_points,
            sprite: sprite_rc,
        };

        // store pointer of chunks to hashmap
        for chunk_rc in cm.chunks_arr.iter_mut() {
            let chunk_pos = chunk_rc.borrow().chunk_pos();
            chunk_rc.borrow_mut().is_active = true;
            cm.chunks_map.insert(chunk_pos, chunk_rc.clone());
        }

        cm.generate(world_pos.clone());

        cm
    }

    pub fn generate(&mut self, world_pos: Vector2) {
        // let mut new_points: Vec<Point> = vec![];
        self.world_pos = world_pos;

        let chunk_pos: Point = world_to_chunk(&self.world_pos);

        // default all to inactive
        for chunk_rc in self.chunks_arr.iter_mut() {
            chunk_rc.borrow_mut().is_active = false;
        }

        for y in -self.render_dist..self.render_dist {
            for x in -self.render_dist..self.render_dist {
                let n_chunk_pos = Point { x: chunk_pos.x + x, y: chunk_pos.y + y };
                // set active if on chunk_pos
                if let Some(chunk_rc) = self.chunks_map.get(&n_chunk_pos) {
                    chunk_rc.borrow_mut().is_active = true;
                } 
                else {
                    self.new_chunk_points.push(n_chunk_pos);
                }
            }
        }

        // println!("size of arr: {}, size of map: {}", self.chunks_arr.len(), self.chunks_map.len());
        for chunk_rc in self.chunks_arr.iter_mut() {
            let mut chunk = chunk_rc.borrow_mut();
            if !chunk.is_active {
                let c_chunk_pos = self.new_chunk_points.pop().unwrap(); // refer to new_points for new
                                                             // rendered chunks
                // move chunk reference with the new point key
                self.chunks_map.remove(&chunk.chunk_pos);
                self.chunks_map.insert(c_chunk_pos.clone(), chunk_rc.clone());

                // set new value based on pos
                // note that once you set, the chunk automatically changes its tile frames based
                // on that pos, draw function will adjust it, no need of new tile alloc
                let c_world_pos = chunk_to_world(&c_chunk_pos);
                chunk.set(c_world_pos);
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        for chunk_rc in self.chunks_arr.iter_mut() {
            chunk_rc.borrow_mut().draw(canvas);
        }
    }
}
