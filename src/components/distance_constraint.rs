use bevy_ecs::prelude::*;
use std::vec::Vec;
use crate::components::*;

#[derive(Component, Clone)]
pub struct ForwardConstraint {
    pub target: Option<Entity>,
    pub target_offset: Vector2,
    pub distance: f32,
    pub stiffness: f32
}
impl DistanceConstraint for ForwardConstraint {
    fn distance(&self) -> f32 { self.distance }
    fn stiffness(&self) -> f32 { self.stiffness }
    fn target(&self) -> Option<Entity> { 
        self.target
    }
    fn target_offset(&self) -> Vector2 {
        self.target_offset
    }
}

#[derive(Component, Clone)]
pub struct BackwardConstraint {
    pub target: Option<Entity>,
    pub distance: f32,
    pub target_offset: Vector2,
    pub stiffness: f32
}
impl DistanceConstraint for BackwardConstraint {
    fn distance(&self) -> f32 { self.distance }
    fn stiffness(&self) -> f32 { self.stiffness }
    fn target(&self) -> Option<Entity> { 
        self.target
    }
    fn target_offset(&self) -> Vector2 {
        self.target_offset
    }
}

pub trait DistanceConstraint: Component {
    fn distance(&self) -> f32;
    fn stiffness(&self) -> f32;
    fn target(&self) -> Option<Entity>;
    fn target_offset(&self) -> Vector2;
}

#[derive(Component)]
pub struct PolygonId {
    pub list: Vec<Entity>
}

#[derive(Component, Clone)]
#[require(LocalTransform)]
pub struct Anchor(pub Entity);
