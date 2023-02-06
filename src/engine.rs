use crate::entity::{Entity, Axis, Edge};
use crate::components::Component;
use crate::components::Component::InputComponent;
use crate::components::Component::AudioComponent;
use crate::components::Component::PhysicsComponent;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::shaders::Shader;

use std::time::{Duration, Instant};

use vulkano::buffer::{TypedBufferAccess, CpuAccessibleBuffer, BufferUsage};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::pipeline::{Pipeline, PipelineBindPoint};
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Engine {
    pub title: String,
    pub window_size: (u32, u32),
    pub view_size: (u32, u32),
    pub fps: u64,
    pub scene: Scene,
    renderer: Renderer,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let (renderer, event_loop) = Renderer::init(title, width, height);
        let fps = 60;

        (Engine {
            title: String::from(title),
            window_size: (width, height),
            view_size: (width, height),
            fps,
            renderer,
            scene: Scene::new(
                vec![],
                1,
            ),
        }, event_loop)
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        // Convert FPS to redraw frequency
        let freq_millis = 1000 / self.fps;

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());

        let mut input_handler = InputComponent;

        /*
        match self.scene.bgm.as_ref() {
            Some(p) => self.audio.play_bgm(p),
            None => {},
        }
        */

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
                    self.window_size.0 = size.width;
                    self.window_size.1 = size.height;
                    
                    let x = self.window_size.0 as f32;
                    let y = self.window_size.1 as f32;

                    let res_ratio = self.view_size.0 as f32 / self.view_size.1 as f32;
                    let win_ratio = self.window_size.0 as f32 / self.window_size.1 as f32;

                    if win_ratio > res_ratio {
                        let vx = y * res_ratio;
                        let vy = y;

                        self.renderer.viewport.dimensions = [vx, vy];
                        self.renderer.viewport.origin = [
                            (x / 2.0) - (vx / 2.0),
                            0.0,
                        ];
                    } else {
                        let vx = x;
                        let vy = x / res_ratio;

                        self.renderer.viewport.dimensions = [vx, vy];
                        self.renderer.viewport.origin = [
                            0.0,
                            (y / 2.0) - (vy / 2.0),
                        ];
                    }

                    self.renderer.recreate_pipelines().unwrap();
                    recreate_swapchain = true
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

                    //input_handler.cursor = pos;
                }

                Event::RedrawEventsCleared => {
                    /*
                    let entity_index = self.scene.player_index;
                    self.update_position(&input_handler, entity_index);
                    */

                    previous_frame_end.as_mut().unwrap().cleanup_finished();

                    if recreate_swapchain {
                        if let Err(_) = self.renderer.recreate_swapchain() {
                            return;
                        }
                        recreate_swapchain = false;
                    }

                    let (image_num, suboptimal, acquire_future) =
                        match self.renderer.acquire_next_image() {
                            Ok(d) => d,
                            Err(_) => {
                                recreate_swapchain = true;
                                return;
                            }
                        };

                    if suboptimal {
                        recreate_swapchain = true;
                    }

                    let clear_values = vec![[0.0, 0.0, 0.0].into()];

                    let mut builder = AutoCommandBufferBuilder::primary(
                        self.renderer.device.clone(),
                        self.renderer.queue.family(),
                        CommandBufferUsage::OneTimeSubmit,
                    ).unwrap();

                    builder
                        .begin_render_pass(
                            self.renderer.framebuffers[image_num].clone(),
                            SubpassContents::Inline,
                            clear_values,
                        )
                        .unwrap()
                        .set_viewport(0, [self.renderer.viewport.clone()]);

                    /*
                    for entity in &self.scene.entities {
                        let sprite = &entity.sprite;
                        let vertices = Renderer::create_vertex_buffer(sprite.vertices.clone(), &self.renderer.device);
                        let indices = CpuAccessibleBuffer::from_iter(self.renderer.device.clone(), BufferUsage::all(), false, sprite.indices.clone())
                            .expect("Failed to create buffer");
                        let pipeline = self.renderer.pipelines[&sprite.shader].clone();

                        builder
                            .bind_pipeline_graphics(pipeline.clone());

                        if let (Shader::Texture, Some(s)) = (sprite.shader, &sprite.texture) {
                            let (texture, texture_future) = self.renderer.create_texture(s);
                            let layout = pipeline.layout().descriptor_set_layouts().get(0).unwrap();
                            let set = PersistentDescriptorSet::new(
                                layout.clone(),
                                [WriteDescriptorSet::image_view_sampler(
                                    0,
                                    texture,
                                    self.renderer.sampler.clone(),
                                )],
                            ).unwrap();

                            previous_frame_end = Some(texture_future.boxed());

                            builder.bind_descriptor_sets(
                                PipelineBindPoint::Graphics,
                                pipeline.layout().clone(),
                                0,
                                set.clone(),
                            );
                        }

                        builder
                            .bind_vertex_buffers(0, vertices.clone())
                            .bind_index_buffer(indices.clone())
                            .draw_indexed(indices.len() as u32, vertices.len() as u32, 0, 0, 0)
                            .unwrap();
                    }
                    */

                    builder
                        .end_render_pass()
                        .unwrap();

                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.renderer.queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(self.renderer.queue.clone(), self.renderer.swapchain.clone(), image_num)
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        },
                        Err(FlushError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());
                        },
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());
                        }
                    }
                }
                _ => (),
            }
        });
    }

    //fn update_position(&mut self, input: &InputComponent, entity_index: usize) {
    fn update_position(&mut self, input: &Component, entity_index: usize) {
        let units = (
            1.0 / self.view_size.0 as f32,
            1.0 / self.view_size.1 as f32,
        );

        /*
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
        */
    }
}
