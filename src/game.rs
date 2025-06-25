use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::render::*;
use crate::structs::player::*;
use crate::managers::chunk::*;
use crate::math_helper::*;
// use crate::structs::entity::*;
// use crate::config::SCREEN_WIDTH;
// use crate::config::SCREEN_HEIGHT;
use crate::world::*;

pub struct Game <'a> {
    player: Player <'a>, 
    world: World<'a>,
    test_chunk: Chunk <'a>,
    // entities: Vec<Entity>,
}

impl <'a> Game <'a> {
    pub fn new(t_creator: &'a TextureCreator<WindowContext>) -> Self {
        // let camera = Camera2D {
        //     offset: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        //     target: Vector2::new(0.0, 0.0),
        //     rotation: 0.0,
        //     zoom: 2.0,
        // };

        let world = World::new(t_creator);
        let player = Player::new(world.am.get_player_t());

        Self {
            test_chunk: Chunk::new(Vector2::new(0.0,0.0), world.am.get_tile_atlas_t()),
            world: world,
            player: player,
            // entities: Vec::new(),
        }
    }   

    pub fn update(&mut self, delta_time: f32) {
        self.player.update(delta_time);
        // let tile_atlas_t = self.world.am.get_tile_atlas_t(); 
        // self.world.cm.generate(self.player.get_pos().clone(), 
        //     tile_atlas_t,
        //     4
        // );
    }

    pub fn fixed_update(&mut self, _time_step: f32) {
        // self.player.fixed_update(_time_step);
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        // drawn relative to camera
        // let mut d2 = d.begin_mode2D(self.camera);
        // self.world.cm.draw(canvas);
        self.player.draw(canvas);
        self.test_chunk.draw(canvas);
    }

    pub fn input(&mut self, event: &Event) {
        self.player.input(event);
    }
}
