use std::collections::HashMap;
use bevy_ecs::prelude::*;
use crate::components::Vector2;
use crate::core::renderer::*;
use crate::components::sprite::*;
use crate::resources::chunk::*; 
use crate::resources::asset_manager::*;
use crate::resources::area_manager::*;
use crate::math_helper::*;

#[derive(Resource, Clone, Default)]
pub struct ChunkManager  {
    chunks_map: HashMap<Point, usize>,
    chunks_arr: Vec<Chunk>,
    new_chunk_points: Vec<Point>,
    render_dist: i32,
    world_pos: Vector2,
    sprite: Sprite
}

impl ChunkManager {
    pub fn new(world_pos: Vector2, asset_m: &AssetManager, h_render_dist: i32) -> ChunkManager {

        // init sprite here so chunks and tiles don't create one for themselves
        let mut sprite = Sprite::new(asset_m, TextureId::TileAtlas);
        sprite.set_sprite_sheet(4, 2);


        // notice mul by 2, the h_render_dist is only for one side
        // meaning you mul by 2 to give space for both right and top
        // this also disallows odd number rendering
        let render_dist: i32 = h_render_dist * 2; 
        let size = render_dist * render_dist;

        let mut new_chunk_points = Vec::new();
        new_chunk_points.reserve_exact(size as usize);

        // init chunks array with default value and size
        let chunks_arr = vec![Chunk::new(Vector2::new(0.0, 0.0)); size as usize];
        let chunks_map = HashMap::new();

        // init default values of chunk manager
        let cm = ChunkManager {
            render_dist: render_dist,
            chunks_map: chunks_map,
            // init values of chunks within array
            chunks_arr: chunks_arr,             
            world_pos: world_pos.clone(),
            new_chunk_points: new_chunk_points,
            sprite: sprite,
        };

        //cm.generate(world_pos.clone());

        cm
    }

    pub fn generate(&mut self, world_pos: &Vector2, area_m: &mut AreaManager) {

        self.world_pos = *world_pos;

        // new chunk points to save unrendered new chunk positions
        self.new_chunk_points.clear();

        let chunk_pos: Point = world_to_chunk(&self.world_pos);

        // default all to inactive
        for chunk_rc in self.chunks_arr.iter_mut() {
            chunk_rc.is_active = false;
        }

        for y in 0..self.render_dist {
            for x in 0..self.render_dist {
                // calc chunk position ---> key to chunks_map
                let n_chunk_pos = Point {
                    x: chunk_pos.x + x - self.render_dist / 2, 
                    y: chunk_pos.y + y - self.render_dist / 2 
                };

                // set active if on chunk_map
                if let Some(index) = self.chunks_map.get(&n_chunk_pos) {
                    let chunk = self.chunks_arr.get_mut(*index).expect("invalid index");
                    chunk.is_active = true;
                } 
                else {
                    self.new_chunk_points.push(n_chunk_pos);
                }
            }
        }

        for y in 0..self.render_dist {
            for x in 0..self.render_dist {
                let index = (y * self.render_dist + x) as usize;
                let chunk = self.chunks_arr.get_mut(index).expect("invalid index");

                if !chunk.is_active {
                    // refer to new_points for new points to generate
                    let c_chunk_pos = self.new_chunk_points.pop().expect("invalid index"); 
                                                                 // rendered chunks
                    // move chunk reference with the new point key
                    self.chunks_map.remove(&chunk.chunk_pos);
                    self.chunks_map.insert(c_chunk_pos.clone(), index);

                    /* set new value based on pos
                    *  note that once you set, the chunk automatically changes its tile frames based
                    *  on that pos, no need of new tile alloc RECYCLING */
                    let c_world_pos = chunk_to_world(&c_chunk_pos);
                    // println!("aream size: {}", area_m.tile_areas.len());
                    chunk.set(c_world_pos, area_m);
                }
            }
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        for chunk in self.chunks_arr.iter_mut() {
            chunk.draw(renderer, &self.sprite);
        }
    }
}
