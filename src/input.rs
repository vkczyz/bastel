use crate::entity::Entity;

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

    pub fn handle_movement(&self, player: &mut Entity, strength: (f32, f32)) {
        let force = get_vector_normalised((
            0.0 + (self.right as i32 as f32) - (self.left as i32 as f32),
            0.0 + (self.down as i32 as f32),
        ));
        let force = (force.0 * strength.0, force.1 * strength.1);
        player.physics.apply_force(force);

        if self.up && player.contact {
            player.physics.velocity.1 -= strength.1 * 16.0;
        }
    }

    pub fn is_valid_cursor_position(&self) -> bool {
        if self.cursor[0] < -1.0 || self.cursor[0] > 1.0 { return false; }
        if self.cursor[1] < -1.0 || self.cursor[1] > 1.0 { return false; }
        true
    }
}

fn get_vector_magnitude(v: (f32, f32)) -> f32 {
    ((v.0 * v.0) + (v.1 * v.1)).sqrt()
}

fn get_vector_normalised(v: (f32, f32)) -> (f32, f32) {
    let mag = get_vector_magnitude(v);
    match mag == 0.0 {
        true => (0.0, 0.0),
        false => (
            v.0 / mag,
            v.1 / mag,
        ),
    }
}