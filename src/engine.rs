use crate::entity::Entity;
use crate::input::Input;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::shaders::Shader;
use crate::sprite::Sprite;

use std::path::Path;
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
    pub width: u32,
    pub height: u32,
    pub resolution: (u32, u32),
    pub fps: u64,
    pub renderer: Renderer,
    pub input: Input,
    pub scene: Scene,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let (renderer, event_loop) = Renderer::init(title, width, height);
        let input = Input::new();
        let fps = 60;

        let entities = vec![
            Entity::new(
                Sprite::new(
                    (-1.0, -1.0),
                    (2.0, 2.0),
                    None,
                )
            ),
            Entity::new(
                Sprite::new(
                    (-0.5, -0.5),
                    (0.1, 0.1),
                    Some(Shader::Rainbow),
                )
            ),
        ];

        (Engine {
            title: String::from(title),
            width,
            height,
            resolution: (width, height),
            fps,
            renderer,
            input,
            scene: Scene::new(
                entities,
                1,
            ),
        }, event_loop)
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        // Convert FPS to redraw frequency
        let freq_millis = 1000 / self.fps;

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());

        let ratio = self.width / self.height;

        let mut input_handler = Input::new();

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
                    self.width = size.width;
                    self.height = size.height;
                    
                    let x = self.width;
                    let y = self.height;

                    if x > y {
                        let vx = y*ratio;
                        self.renderer.viewport.dimensions = [vx as f32, y as f32];
                        self.renderer.viewport.origin = [((x / 2) - (vx / 2)) as f32, 0.0];
                    } else {
                        let vy = x/ratio;
                        self.renderer.viewport.dimensions = [x as f32, vy as f32];
                        self.renderer.viewport.origin = [0.0, ((y / 2) - (vy / 2)) as f32];
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
                    input_handler.handle_input(input);
                }

                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        state: winit::event::ElementState::Released,
                        ..
                    },
                    ..
                } => {
                    if !input_handler.is_valid_cursor_position() {
                        return;
                    }

                    let mut sprite = Sprite::new(
                        (input_handler.cursor[0], input_handler.cursor[1]),
                        (0.1, 0.1),
                        Some(Shader::Texture),
                    );
                    sprite.add_texture(Path::new("data/textures/test.png")).unwrap();
                    self.scene.entities.insert(self.scene.player_index, Entity::new(sprite));
                    self.scene.player_index += 1;
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

                    input_handler.cursor = pos;
                }

                Event::RedrawEventsCleared => {
                    self.update_position(&input_handler);

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

    fn update_position(&mut self, input: &Input) {
        let units = [
            1.0 / self.resolution.0 as f32,
            1.0 / self.resolution.1 as f32,
        ];

        let player = &mut self.scene.entities[self.scene.player_index];

        input.handle_movement(
            player,
            &self.scene.physics,
            &[
                units[0] / 2.0,
                units[1] / 2.0,
                ],
        );
    }

}