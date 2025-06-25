use sdl2::render::*;
use sdl2::event::*;
use std::rc::Rc;
use sdl2::keyboard::Keycode;
use crate::components::physics::Physics;
use crate::components::area::Area;
use crate::components::sprite::Sprite;
use crate::math_helper::*;

pub struct Player <'a> {
    physics: Physics,
    area: Area,
    sprite: Sprite <'a>,
    running: bool,
}

impl <'a> Player <'a> {
    pub fn new(texture: Rc<Texture<'a>>) -> Self {
        // let texture: Texture2D = rl.load_texture(thread, "assets/player.png").unwrap();

        let mut sprite = Sprite::new(texture);
        sprite.set_sprite_sheet(6, 4);

        Self {
            sprite: sprite,
            physics: Physics {
                pos: Vector2::new(100.0, 100.0),
                vel: Vector2::new(0.0, 0.0),
                mass: 1.0,
            },
            area: Area {
                pos: Vector2::new(100.0, 100.0),
                width: 10.0,
                height: 10.0,
            },
            running: false,
        }
    }    

    pub fn update(&mut self, delta_time: f32) {

    }

    pub fn fixed_update(&mut self, time_step: f32) {
        
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.sprite.draw(canvas, &self.physics.pos, 0);
    }

    pub fn input(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                self.physics.pos.x -= 10.0;
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                self.physics.pos.y += 10.0;
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                self.physics.pos.y -= 10.0;
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                self.physics.pos.x += 10.0;
            },
            _ => {}
        }
    }

    pub fn get_pos(&mut self) -> &Vector2 {
        return &self.physics.pos;
    }
}


