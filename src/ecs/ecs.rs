use crate::components::sprite::*;
use crate::ecs::component::*;

pub struct ECS<'a> {
    sprites: ComponentArray<Sprite<'a>>,
}

