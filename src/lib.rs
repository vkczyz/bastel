pub mod engine;
mod shaders;
mod vertex;

use engine::Engine;
use std::time::{Duration, Instant};

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::swapchain::{self, AcquireError};
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::buffer::TypedBufferAccess;
use vulkano::command_buffer::SubpassContents;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn begin_loop(mut engine: Engine, event_loop: EventLoop<()>, fps: u64) {
    // Convert FPS to redraw frequency
    let freq_millis = 1000 / fps;

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(engine.device.clone()).boxed());

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
                event: WindowEvent::Resized(_),
                ..
            } => { recreate_swapchain = true },
            Event::RedrawEventsCleared => {
                previous_frame_end.as_mut().unwrap().cleanup_finished();

                if recreate_swapchain {
                    if let Err(_) = engine.recreate_swapchain() {
                        return;
                    }
                    recreate_swapchain = false;
                }

                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(engine.swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        },
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                if suboptimal {
                    recreate_swapchain = true;
                }

                let clear_values = vec![[0.1, 0.1, 0.1].into()];

                let mut builder = AutoCommandBufferBuilder::primary(
                    engine.device.clone(),
                    engine.queue.family(),
                    CommandBufferUsage::OneTimeSubmit,
                ).unwrap();

                builder
                    .begin_render_pass(
                        engine.framebuffers[image_num].clone(),
                        SubpassContents::Inline,
                        clear_values,
                    )
                    .unwrap()
                    .set_viewport(0, [engine.viewport.clone()])
                    .bind_pipeline_graphics(engine.pipeline.clone())
                    .bind_vertex_buffers(0, engine.vertex_buffer.clone())
                    .draw(engine.vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass()
                    .unwrap();

                let command_buffer = builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(engine.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(engine.queue.clone(), engine.swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    },
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                    },
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                    }
                }
            }
            _ => (),
        }
    });
}