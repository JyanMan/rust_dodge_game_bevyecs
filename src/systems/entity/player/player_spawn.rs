use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::*;
use crate::resources::asset_manager::*;
use crate::components::entity::*;

pub fn player_spawn(world: &mut World, renderer: &mut Renderer) {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);

    world.spawn((
         sprite,
         Transform::new(10.0, -1000.0),
         Velocity::new(0.0, 0.0),
         area,
         PlayerData::default(),
         PlayerTag {},
         PlayerInput::default(),
         WalkerData {
             run_speed: 200.0,
             accel: 50.0,
             jump_force: 300.0,
             grounded: false,
             state: WalkerState::default()
         },
         // AnimationPlayer::new(WalkerAnim::COUNT),
         Combat::default(),
    ));
}
