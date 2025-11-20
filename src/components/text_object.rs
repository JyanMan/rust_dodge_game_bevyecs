use bevy_ecs::prelude::*;
use crate::resources::TextId;
use crate::core::Renderer;
use crate::components::*;

#[derive(Component)]
pub struct TextObject {
    pub new: bool,
    relative_to_camera: bool,
    id: TextId,
    content: String,
    prev_content: String,
    pos: Vector2,
    pub size: i32,
}

impl TextObject {
    pub fn new(str: &str, size: i32, pos: Vector2, rel: bool) -> Self {
        Self {
            new: true,
            id: 0,
            relative_to_camera: rel,
            content: String::from(str),
            prev_content: String::from(""),
            pos,
            size,
        }
    }

    pub fn is_relative_to_camera(&self) -> bool { self.relative_to_camera }
    
    pub fn set_relative_to_camera(&mut self) { self.relative_to_camera = true; }

    pub fn pos(&self) -> Vector2 { self.pos }

    pub fn set_pos(&mut self, pos: Vector2) { self.pos = pos }

    pub fn content(&self) -> &str { self.content.as_str() }

    pub fn id(&self) -> TextId { self.id }

    pub fn set_content(&mut self, str: &str) {
        self.prev_content = self.content.clone();
        self.content = String::from(str);
    }

    pub fn set_id(&mut self, id: TextId) { self.id = id }
    
    pub fn changed(&self) -> bool { self.prev_content != self.content }

    pub fn mark_unchanged(&mut self) {
        self.prev_content = self.content.clone(); self.new = false;
    }
}

