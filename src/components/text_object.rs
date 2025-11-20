use bevy_ecs::prelude::*;
use crate::resources::TextId;
use crate::core::Renderer;
use crate::components::*;

#[derive(Component)]
pub struct TextObject {
    id: TextId,
    content: String,
    prev_content: String,
    pos: Vector2,
    pub size: i32,
}

impl TextObject {
    pub fn new(renderer: &mut Renderer, str: &str, size: i32, pos: Vector2) -> Self {
        Self {
            id: renderer.asset_m.text_set.len(),
            content: String::from(str),
            prev_content: String::from(""),
            pos,
            size,
        }
    }

    pub fn pos(&self) -> Vector2 { self.pos }

    pub fn content(&self) -> &str { self.content.as_str() }

    pub fn id(&self) -> TextId { self.id }

    pub fn set_content(&mut self, str: &str) {
        self.prev_content = self.content.clone();
        self.content = String::from(str);
    }

    pub fn changed(&self) -> bool { self.prev_content != self.content }

    pub fn mark_unchanged(&mut self) { self.prev_content = self.content.clone();  }
}

