use crate::renderer::Renderer;
use crate::systems::System;
use crate::shaders::Shader;

use vulkano::buffer::{TypedBufferAccess, CpuAccessibleBuffer, BufferUsage};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::pipeline::PipelineBindPoint;
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};

pub struct RenderSystem {
    pub renderer: Renderer,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl RenderSystem {
    pub fn new(renderer: Renderer) -> Self {
        RenderSystem {
            renderer,
            recreate_swapchain: false,
            previous_frame_end: Some(sync::now(renderer.device.clone()).boxed()),
        }
    }

    /*
    pub fn resize() {
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
    }
    */
}

impl System for RenderSystem {
    fn run(&mut self) {
        // Convert FPS to redraw frequency

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(sync::now(self.renderer.device.clone()).boxed());

        //self.update_position(&input_handler, entity_index);

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
}