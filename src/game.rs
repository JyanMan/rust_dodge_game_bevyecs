use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::render::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::systems::sprite_system::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;

pub struct Game {
    ecs: ECS,
}

impl <'a> Game {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>, renderer: &mut Renderer) -> Self {
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
        // ecs.register_component::<PlayerTag>();

        ecs.register_system_startup(sprite_startup_system());
        ecs.register_system_draw(sprite_draw_system());
        
        ecs.call_startup_systems(renderer);

        Self {
            ecs: ecs,
        }

    }   

    pub fn update(&mut self, delta_time: f32) {
        self.ecs.call_update_systems(delta_time);
    }

    pub fn fixed_update(&mut self, time_step: f32) {
        self.ecs.call_update_systems(time_step);
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        // self.player_system.borrow_mut().draw(&mut self.ecs, renderer);
        self.ecs.call_draw_systems(renderer);
    }

    pub fn input(&mut self, event: &Event) {
    }
}
