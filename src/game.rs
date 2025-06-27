use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::render::*;
use std::rc::Rc;
use std::cell::RefCell;
// use crate::structs::player::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::systems::sprite_system::*;
use crate::managers::renderer::*;
use crate::systems::asset_manager::*;
use crate::components::camera::*;
use crate::ecs::ecs::*;
use crate::ecs::entity::*;
use crate::world::*;

pub struct Game {
    // player: Player , 
    // world: World,
    ecs: ECS,
    sprite_system: Rc<RefCell<SpriteSystem>>,
    player: Entity,
    // test_chunk: Chunk ,
    // entities: Vec<Entity>,
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

        let sprite_system = ecs.register_system::<SpriteSystem>();
        let mut signature: Signature = Signature::default();
        signature |= 1 << ecs.get_component_type::<Position>().unwrap();
        signature |= 1 << ecs.get_component_type::<Sprite>().unwrap();
        ecs.set_system_signature::<SpriteSystem>(signature);

        let player = ecs.create_entity();
        ecs.add_component::<Position>(player, Position { x: 10.0, y: 10.0 });
        ecs.add_component::<Sprite>(player, 
            Sprite::new(
                asset_m.get_texture(TextureId::Player),
                TextureId::Player
            )
        );

        sprite_system.borrow_mut().init(&mut ecs, asset_m);

        Self {
            player: player,
            // test_chunk: Chunk::new(Vector2::new(0.0,0.0), world.am.get_tile_atlas_t()),
            // world: world,
            // player: player,
            ecs: ecs,
            sprite_system: sprite_system.clone(),
            // entities: Vec::new(),
        }

    }   

    pub fn update(&mut self, delta_time: f32) {
        // self.player.update(delta_time);
        // self.world.cm.generate(self.player.get_pos().clone());
        // self.world.sm.camera.set_target(self.player.get_pos());
    }

    pub fn fixed_update(&mut self, _time_step: f32) {
        // self.player.fixed_update(_time_step);
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        // drawn relative to camera
        // let mut d2 = d.begin_mode2D(self.camera);
        // self.world.cm.draw(canvas);
        // self.player.draw(canvas);
        // self.test_chunk.draw(canvas);
        self.sprite_system.borrow_mut().draw(&mut self.ecs, renderer);
    }

    pub fn input(&mut self, event: &Event) {
        // self.player.input(event);
    }
}
