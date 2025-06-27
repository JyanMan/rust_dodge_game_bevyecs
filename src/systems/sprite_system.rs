use std::collections::HashSet;
use std::any::Any;
use crate::components::position::*;
use crate::systems::asset_manager::*;
use crate::managers::renderer::*;
use crate::math_helper::*;
use crate::components::sprite::*;
use crate::ecs::system::*;
use crate::ecs::entity::*;
use crate::ecs::ecs::*;

#[derive(Default)]
pub struct SpriteSystem {
    entities: HashSet<Entity>,
}

impl System for SpriteSystem {
    fn entities(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities 
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl SpriteSystem {
    pub fn init(&self, ecs: &mut ECS, asset_m: &AssetManager) {
        for entity in self.entities.iter() {
            let sprite = ecs.get_component_mut::<Sprite>(*entity).unwrap();
            sprite.init(asset_m.get_texture(TextureId::Player), TextureId::Player);
        }
    }

    pub fn draw(&self, ecs: &mut ECS, renderer: &mut Renderer) {
        for entity in self.entities.iter() {
            if let (Some(position), Some(sprite)) = (
                ecs.get_component::<Position>(*entity),
                ecs.get_component::<Sprite>(*entity),
            ) {
                sprite.draw(
                    renderer, 
                    &Vector2::new(position.x, position.y),
                    0
                );
            }
        }
    }
}
