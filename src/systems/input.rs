use crate::global::Global;
use crate::entity::Entity;
use crate::systems::System;

use std::sync::{Arc, Mutex};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyboardInput};

pub struct InputSystem {
    pub cursor: [f32; 2],
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    global: Arc<Mutex<Global>>,
}

impl InputSystem {
    pub fn new(global: Arc<Mutex<Global>>) -> Self {
        InputSystem {
            cursor: [0.0, 0.0],
            up: false,
            down: false,
            left: false,
            right: false,
            global,
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

        let global = self.global.clone();
        let mut global = global.lock().expect("Could not unlock global object");

        global.signals.insert("up_pressed".to_string(), self.up);
        global.signals.insert("down_pressed".to_string(), self.down);
        global.signals.insert("left_pressed".to_string(), self.left);
        global.signals.insert("right_pressed".to_string(), self.right);

    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        let dims: (f32, f32);
        let origin: (f32, f32);

        {
            let global = self.global.clone();
            let global = global.lock().expect("Could not unlock global object");
            dims = global.view_size;
            origin = global.view_origin;
        }

        let real_dims = [dims.0, dims.1];
        let view_dims = [
            real_dims[0] - 2.0 * origin.0,
            real_dims[1] - 2.0 * origin.1,
        ];

        let mut pos: [f32; 2] = position.into();
        pos = [
            (2.0 * (pos[0] - real_dims[0] / 2.0) / real_dims[0]),
            (2.0 * (pos[1] - real_dims[1] / 2.0) / real_dims[1]),
        ];
        pos[0] *= real_dims[0] / view_dims[0];
        pos[1] *= real_dims[1] / view_dims[1];

        self.cursor = pos;
    }

    pub fn click(&self) {
        if !self.is_valid_cursor_position() { return; }

        let global = self.global.clone();
        let mut global = global.lock().expect("Could not unlock global object");
        global.click = (self.cursor[0], self.cursor[1]);
        global.signals.insert("resize".to_string(), true);
    }

    fn is_valid_cursor_position(&self) -> bool {
        if self.cursor[0] < -1.0 || self.cursor[0] > 1.0 { return false; }
        if self.cursor[1] < -1.0 || self.cursor[1] > 1.0 { return false; }
        true
    }

}

impl System for InputSystem {
    fn run(&mut self, _entities: &mut [Arc<Mutex<Entity>>]) { }
}