use crate::config::*;
use crate::components::position::*;
use crate::components::Vector2;
use std::ops;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn lerp_pos(a: &Vector2, b: &Vector2, t: f32) -> Vector2 {
    Vector2 {
        x: lerp(a.x, b.x, t),
        y: lerp(a.y, b.y, t),
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32, pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x, y: y
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
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
pub fn tile_to_world(tile_pos: &Point) -> Vector2 {
    return Vector2 {
        x: (tile_pos.x * TILE_SIZE) as f32,
        y: (tile_pos.y * TILE_SIZE) as f32,
    };
}

pub fn pos_to_point(vec: Vector2) -> Point {
    return Point { x: vec.x as i32, y: vec.y as i32 };
}
