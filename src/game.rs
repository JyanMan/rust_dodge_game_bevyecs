use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::render::*;
use std::rc::Rc;
use std::cell::RefCell;
// use crate::structs::player::*;
use crate::managers::asset_manager::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::systems::sprite_system::*;
use crate::systems::player_system::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;
use crate::ecs::entity::*;

pub struct Game {
    ecs: ECS,
    sprite_system: Rc<RefCell<SpriteSystem>>,
    player_system: Rc<RefCell<PlayerSystem>>,
}

impl <'a> Game {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>, asset_m: &mut AssetManager) -> Self {
        // let camera = Camera2D {
        //     offset: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        //     target: Vector2::new(0.0, 0.0),
        //     rotation: 0.0,
        //     zoom: 2.0,
        // };
        //

        // let world = World::new(t_creator);
        // let player = Player::new(world.am.get_player_t());

        // let tile_atlas_t = world.am.get_tile_atlas_t(); 
        // world.cm.generate(player.get_pos().clone(), 
        //     tile_atlas_t,
        //     4
        // );

        let mut ecs = ECS::new();

        ecs.register_component::<Position>();
        ecs.register_component::<Sprite>();
        ecs.register_component::<PlayerTag>();

        let sprite_system = ecs.register_system::<SpriteSystem>();
        let player_system = ecs.register_system::<PlayerSystem>();


        let mut sprite_sign: Signature = Signature::default();
        sprite_sign |= 1 << ecs.get_component_type::<Position>().unwrap();
        sprite_sign |= 1 << ecs.get_component_type::<Sprite>().unwrap();
        ecs.set_system_signature::<SpriteSystem>(sprite_sign);

        let mut player_sign: Signature = Signature::default();
        player_sign |= 1 << ecs.get_component_type::<Position>().unwrap();
        player_sign |= 1 << ecs.get_component_type::<Sprite>().unwrap();
        player_sign |= 1 << ecs.get_component_type::<PlayerTag>().unwrap();
        ecs.set_system_signature::<PlayerSystem>(player_sign);

        // sprite_system.borrow_mut().init(&mut ecs, asset_m);
        player_system.borrow_mut().init(&mut ecs, asset_m);

        // let player = ecs.create_entity();
        // let mut sprite = Sprite::new(
        //     asset_m.get_texture(TextureId::Player),
        //     TextureId::Player
        // );
        // sprite.set_sprite_sheet(6, 6);

        // ecs.add_component::<Position>(player, Position { x: 10.0, y: 10.0 });
        // ecs.add_component::<Sprite>(player, sprite);
        // ecs.add_component::<PlayerTag>(player, PlayerTag{});

        Self {
            ecs: ecs,
            sprite_system: sprite_system.clone(),
            player_system: player_system.clone(),
        }

    }   

    pub fn update(&mut self, delta_time: f32) {
    }

    pub fn fixed_update(&mut self, _time_step: f32) {
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.sprite_system.borrow_mut().draw(&mut self.ecs, renderer);
        // self.player_system.borrow_mut().draw(&mut self.ecs, renderer);
    }

    pub fn input(&mut self, event: &Event) {
    }
}
