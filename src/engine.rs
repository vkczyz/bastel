use crate::components;
use crate::entity::Entity;
use crate::global::Global;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::systems::audio::AudioSystem;
use crate::systems::render::RenderSystem;

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Engine {
    pub global: Arc<Mutex<Global>>,
    pub scene: Scene,
    pub fps: u64,
    renderer: Renderer,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let (renderer, event_loop) = Renderer::init(title, width, height);
        let fps = 60;

        let global = Global::new(
            title.to_string(),
            (width, height),
        );

        (Engine {
            global,
            scene: Scene::new(vec![]),
            fps,
            renderer,
        }, event_loop)
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        self.scene.add_entity(Entity::new(43, vec![
            components::audio::AudioComponent::new(),
        ]));
        self.scene.add_system(Box::new(RenderSystem::new(self.renderer, self.global.clone())));
        self.scene.add_system(Box::new(AudioSystem::new(self.global.clone())));

        let freq_millis = 1000 / self.fps;

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
                } => {
                    //input_handler.handle_input(input);
                }

                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        state: winit::event::ElementState::Released,
                        ..
                    },
                    ..
                } => {
                    /*
                    if input_handler.is_valid_cursor_position() {
                        println!("({}, {})", input_handler.cursor[0], input_handler.cursor[1]);
                    }
                    */
                },

                Event::WindowEvent {
                    event: WindowEvent::CursorMoved {
                        position,
                        ..
                    },
                    ..
                } => {
                    /*
                    let real_dims: [f32; 2] = self.renderer.viewport.dimensions.into();
                    let view_dims: [f32; 2] = [
                        real_dims[0] - 2.0 * self.renderer.viewport.origin[0],
                        real_dims[1] - 2.0 * self.renderer.viewport.origin[1],
                    ];

                    let mut pos: [f32; 2] = position.into();
                    pos = [
                        (2.0 * (pos[0] - real_dims[0] / 2.0) / real_dims[0]) as f32,
                        (2.0 * (pos[1] - real_dims[1] / 2.0) / real_dims[1]) as f32,
                    ];
                    pos[0] *= real_dims[0] / view_dims[0];
                    pos[1] *= real_dims[1] / view_dims[1];

                    input_handler.cursor = pos;
                    */
                }

                Event::RedrawEventsCleared => {
                    for system in self.scene.systems.iter_mut() {
                        system.run(&mut self.scene.entities);
                    }
                },
                _ => (),
            }
        });
    }

    /*
    pub get_entity(id: &str) -> &mut Entity
    pub get_entities_with_components() -> [&Entity]
    pub add_component(entity: &mut Entity, component: Component) -> &mut Entity
    pub add_system(scene: &mut Scene, system: System) -> &mut Entity
    */

    /*
    fn update_position(&mut self, input: &Component, entity_index: usize) {
        let units = (
            1.0 / self.view_size.0 as f32,
            1.0 / self.view_size.1 as f32,
        );

        // Apply scene forces
        let player = &mut self.scene.entities[entity_index];
        let global = self.scene.force;
        //player.physics.apply_force(global);
        for component in player.components {
            match component {
                PhysicsComponent(x) => x.apply_force(global),
            }
        }

        // Apply input forces
        input.handle_movement(
            player,
            (
                units.0,
                units.1,
            ),
        );

        // Apply jump (if requested)
        if input.up {
            let curve = 1.0;
            let force = (
                0.0,
                units.1 * -12.0 / (curve + player.airtime as f32),
            );
            if force.1 < 1.0 {
                player.physics.apply_force(force);
            }
        }

        player.airtime += 1;

        // Collision check
        let player = &self.scene.entities[entity_index];
        let mut collision = None;

        for entity in self.scene.entities.iter() {
            if !entity.collideable {
                continue;
            }
            if entity == player {
                continue;
            }

            if Entity::are_colliding(player, entity) {
                collision = Some((
                    entity.clone(),
                    Entity::get_collision_intersection(player, entity),
                ));
            }
        }

        // Collision handling
        let player = &mut self.scene.entities[entity_index];

        if let Some((e, d)) = collision {
            let x_dist = d[1] - d[0];
            let y_dist = d[3] - d[2];

            let collision_axis = if x_dist < y_dist { Axis::X } else { Axis::Y };
            let edge = match collision_axis {
                Axis::X => {
                    player.physics.bounce_x();
                    player.physics.friction_y();
                    if e.sprite.position.0 == d[0] { Edge::Left } else { Edge::Right }
                },
                Axis::Y => {
                    player.physics.bounce_y();
                    player.physics.friction_x();
                    if e.sprite.position.1 == d[2] { Edge::Top } else { Edge::Bottom }
                },
            };

            match edge {
                Edge::Left => {
                    player.sprite.position.0 -= x_dist;
                },
                Edge::Right => {
                    player.sprite.position.0 += x_dist;
                },
                Edge::Top => {
                    player.sprite.position.1 -= y_dist;
                    if player.physics.velocity.1.abs() < global.1.abs() {
                        player.airtime = 0;
                    }
                },
                Edge::Bottom => {
                    player.sprite.position.1 += y_dist;
                },
            }
        }

        player.physics.update();

        let displ = player.physics.get_displacement();
        let pos = (player.sprite.position.0 + displ.0, player.sprite.position.1 + displ.1);
        player.sprite.change_position(pos);

        player.physics.reset();
    }
    */
}