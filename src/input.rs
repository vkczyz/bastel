use crate::Engine;
use crate::Sprite;

use winit::event::{ElementState, KeyboardInput};

pub struct Input {
    pub cursor: [f32; 2],
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            cursor: [0.0, 0.0],
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn handle_input(&mut self, engine: &mut Engine, input: KeyboardInput) {
        let units: [f32; 2] = [
            1.0 / engine.width as f32,
            1.0 / engine.height as f32,
            ];
        let speed: f32 = 10.0;
        let factor = units.map(|u| u * speed);

        match input.scancode {
            // Clockwise arrow keys
            103 | 17 => {
                self.up = match input.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
            }
            106 | 32 => {
                self.right = match input.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
            },
            108 | 31 => {
                self.down = match input.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
            },
            105 | 30 => {
                self.left = match input.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
            },
            _ => {},
        }

        self.handle_movement(
            engine,
            factor[0] * (0.0 + (self.right as i32 as f32) - (self.left as i32 as f32)),
            factor[1] * (0.0 + (self.down as i32 as f32) - (self.up as i32 as f32)),
        );
    }

    fn handle_movement(&self, engine: &mut Engine, x: f32, y: f32) {
        let old_sprite = match engine.sprites.pop() {
            Some(p) => p,
            None => { return; }
        };

        let new_sprite = Sprite::new(
            (old_sprite.position.0 + x, old_sprite.position.1 + y),
            old_sprite.size,
        );

        engine.sprites.push(new_sprite);
    }

    pub fn is_valid_cursor_position(&self) -> bool {
        if self.cursor[0] < -1.0 || self.cursor[0] > 1.0 { return false; }
        if self.cursor[1] < -1.0 || self.cursor[1] > 1.0 { return false; }
        true
    }
}