use crate::entity::Entity;
use crate::sprite::Sprite;

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

    pub fn handle_input(&mut self, input: KeyboardInput) {
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
    }

    pub fn handle_movement(&self, entities: &mut Vec<Entity>, factor: &[f32]) {
        let x = factor[0] * (0.0 + (self.right as i32 as f32) - (self.left as i32 as f32));
        let y = factor[1] * (0.0 + (self.down as i32 as f32) - (self.up as i32 as f32));

        if entities.len() <= 1 {
            return;
        }

        let old_sprite = match entities.pop() {
            Some(p) => p.sprite,
            None => { return; }
        };

        let mut new_sprite = Sprite::new(
            Input::normalise_position(old_sprite.position.0 + x, old_sprite.position.1 + y, old_sprite.size),
            old_sprite.size,
            Some(old_sprite.shader),
        );
        new_sprite.texture = old_sprite.texture;

        entities.push(Entity::new(new_sprite));
    }

    pub fn is_valid_cursor_position(&self) -> bool {
        if self.cursor[0] < -1.0 || self.cursor[0] > 1.0 { return false; }
        if self.cursor[1] < -1.0 || self.cursor[1] > 1.0 { return false; }
        true
    }

    fn normalise_position(x: f32, y: f32, size: (f32, f32)) -> (f32, f32) {
        let bounds: ((f32, f32), (f32, f32)) = (
            (-1.0, 1.0 - size.0),
            (-1.0, 1.0 - size.1),
        );

        (
            match x {
                p if p < bounds.0.0 => bounds.0.0,
                p if p > bounds.0.1 => bounds.0.1,
                p => p,
            },
            match y {
                p if p < bounds.1.0 => bounds.1.0,
                p if p > bounds.1.1 => bounds.1.1,
                p => p,
            },
        )
    }
}