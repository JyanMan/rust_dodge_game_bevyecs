use crate::config::*;
use std::ops;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32, pub y: i32
}

#[derive(Clone)]
pub struct Vector2 {
    pub x: f32, 
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y:f32) -> Self {
        Self {
            x: x, y: y
        } 
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scale: f32) -> Point {
        Point {
            x: self.x * (scale as i32),
            y: self.y * (scale as i32)
        }
    }
}

#[inline(always)]
pub fn world_to_chunk(world_pos: &Vector2) -> Point {
    return Point {
        x: (world_pos.x / ((CHUNK_SIZE * TILE_SIZE) as f32)).floor() as i32,
        y: (world_pos.y / ((CHUNK_SIZE * TILE_SIZE) as f32)).floor() as i32
    };
}

#[inline(always)]
pub fn chunk_to_world(chunk_pos: &Point) -> Vector2 {
    return Vector2 {
        x: (chunk_pos.x * CHUNK_SIZE * TILE_SIZE) as f32,
        y: (chunk_pos.y * CHUNK_SIZE * TILE_SIZE) as f32,
    };
}

#[inline(always)]
pub fn world_to_tile(world_pos: &Vector2) -> Point {
    return Point {
        x: (world_pos.x / ((TILE_SIZE) as f32)).floor() as i32,
        y: (world_pos.y / ((TILE_SIZE) as f32)).floor() as i32
    };
}

#[inline(always)]
pub fn tile_to_world(tile_pos: Point) -> Vector2 {
    return Vector2 {
        x: (tile_pos.x * TILE_SIZE) as f32,
        y: (tile_pos.y * TILE_SIZE) as f32,
    };
}

pub fn vec_to_point(vec: Vector2) -> Point {
    return Point { x: vec.x as i32, y: vec.y as i32 };
}
