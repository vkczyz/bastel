use crate::global::Global;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::systems::System;
use crate::systems::input::InputSystem;
use crate::systems::render::RenderSystem;
use crate::systems::audio::AudioSystem;
use crate::systems::physics::PhysicsSystem;
use crate::systems::movement::MovementSystem;
use crate::systems::collision::CollisionSystem;

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Engine {
    pub global: Arc<Mutex<Global>>,
    pub scene: Scene,
    pub fps: u64,
    input: InputSystem,
    renderer: Renderer,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let global = Global::new(
            title.to_string(),
            (width, height),
        );

        let (renderer, event_loop) = Renderer::init(title, width, height);
        let input = InputSystem::new(global.clone());
        let fps = 60;

        (Engine {
            global,
            scene: Scene::new(vec![]),
            fps,
            input,
            renderer,
        }, event_loop)
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        let freq_millis = 1000 / self.fps;

        // Initialize systems
        let systems: Vec<Box<dyn System>> = vec![
            Box::new(RenderSystem::new(self.renderer, self.global.clone())),
            Box::new(AudioSystem::new(self.global.clone())),
            Box::new(PhysicsSystem::new(self.global.clone())),
            Box::new(MovementSystem::new(self.global.clone())),
            Box::new(CollisionSystem::new()),
        ];
        for system in systems { self.scene.add_system(system) }

        // Event loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(freq_millis)
            );

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => { *control_flow = ControlFlow::Exit; },

                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let global = self.global.clone();
                    let mut global = global.lock().expect("Could not unlock global object");
                    global.window_size = (size.width, size.height);
                    global.signals.insert("resize".to_string(), true);
                },

                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input,
                        ..
                    },
                    ..
                } => { self.input.handle_input(input); }

                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        state: winit::event::ElementState::Released,
                        ..
                    },
                    ..
                } => { self.input.click(); },

                Event::WindowEvent {
                    event: WindowEvent::CursorMoved {
                        position,
                        ..
                    },
                    ..
                } => { self.input.cursor_moved(position); }

                Event::RedrawEventsCleared => {
                    for system in self.scene.systems.iter_mut() {
                        system.run(&mut self.scene.entities);
                    }
                },
                _ => (),
            }
        });
    }
}