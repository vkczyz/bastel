use crate::components::Component;
use crate::global::Global;
use crate::entity::Entity;
use crate::renderer::Renderer;
use crate::shaders::Shader;
use crate::systems::System;

use std::sync::{Arc, Mutex};
use vulkano::buffer::{TypedBufferAccess, CpuAccessibleBuffer, BufferUsage};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::pipeline::{Pipeline, PipelineBindPoint};
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};

pub struct RenderSystem {
    renderer: Renderer,
    global: Arc<Mutex<Global>>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    recreate_swapchain: bool,
}

impl RenderSystem {
    pub fn new(renderer: Renderer, global: Arc<Mutex<Global>>) -> Self {
        RenderSystem {
            previous_frame_end: Some(sync::now(renderer.device.clone()).boxed()),
            recreate_swapchain: false,
            renderer,
            global,
        }
    }

    fn resize(&mut self) {
        let global = self.global.clone();
        let global = global.lock().expect("Could not unlock global object");
        let window_size = global.window_size;
        let view_size = global.view_size;

        let x = window_size.0 as f32;
        let y = window_size.1 as f32;

        let res_ratio = view_size.0 as f32 / view_size.1 as f32;
        let win_ratio = window_size.0 as f32 / window_size.1 as f32;

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
        self.recreate_swapchain = true
    }
}

impl System for RenderSystem {
    fn run(&mut self, entities: &mut [Arc<Mutex<Entity>>]) {
        self.recreate_swapchain = false;
        self.previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());

        let mut resize = false;
        {
            let global = self.global.clone();
            let mut global = global.lock().expect("Could not unlock global object");
            if let Some(true) = global.signals.get("resize") {
                resize = true;
                global.signals.insert("resize".to_string(), false);
            }
        }

        if resize { self.resize() }
        //self.update_position(&input_handler, entity_index);

        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain {
            if let Err(_) = self.renderer.recreate_swapchain() {
                return;
            }
            self.recreate_swapchain = false;
        }

        let (image_num, suboptimal, acquire_future) =
            match self.renderer.acquire_next_image() {
                Ok(d) => d,
                Err(_) => {
                    self.recreate_swapchain = true;
                    return;
                }
            };

        if suboptimal {
            self.recreate_swapchain = true;
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


        for entity in entities {
            let unlocked_entity = entity.clone();
            let unlocked_entity = unlocked_entity.lock().expect("Could not acquire entity");
            let components = &unlocked_entity.components;

            // Entities need a SpriteComponent and a PositionComponent in order to be drawn
            if let Some(Component::Sprite(sprite)) = components.iter().find(|c| if let Component::Sprite(_) = c { true } else { false }) {
                if let Some(Component::Position(position)) = components.iter().find(|c| if let Component::Position(_) = c { true } else { false }) {
                    let vertices = Renderer::create_vertex_buffer(position.vertices.clone(), &self.renderer.device);
                    let indices = CpuAccessibleBuffer::from_iter(self.renderer.device.clone(), BufferUsage::all(), false, position.indices.clone())
                        .expect("Failed to create buffer");
                    let pipeline = self.renderer.pipelines[&sprite.shader].clone();

                    builder
                        .bind_pipeline_graphics(pipeline.clone());

                    if let (Shader::Texture, Some(s)) = (&sprite.shader, &sprite.texture) {
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

                        self.previous_frame_end = Some(texture_future.boxed());

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
            }
        }

        builder
            .end_render_pass()
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = self.previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.renderer.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.renderer.queue.clone(), self.renderer.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            },
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());
            },
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());
            }
        }
    }
}