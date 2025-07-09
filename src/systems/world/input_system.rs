use sdl2::EventPump;
use crate::ecs::ecs::*;
use crate::components::Vector2;
use crate::resources::MouseInput;

pub fn mouse_input_system(ecs: &mut ECS, event: &mut EventPump) {
    let mut mouse_input = ecs.get_resource_mut::<MouseInput>();
    let mouse_state = event.mouse_state();
    mouse_input.pos = Vector2::new(
        mouse_state.x() as f32, mouse_state.y() as f32
    );
}
