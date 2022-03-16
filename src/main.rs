mod engine;
pub mod shaders;

use engine::Engine;

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::swapchain::{self, SwapchainCreationError, AcquireError};
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::buffer::TypedBufferAccess;
use vulkano::command_buffer::SubpassContents;

use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

fn main() {
    const TITLE: &str = "BASTEL";
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    let engine = Engine::init(TITLE, WIDTH, HEIGHT);
    begin_loop(engine);
}

fn begin_loop(mut engine: Engine) {
    engine.event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => { *control_flow = ControlFlow::Exit; },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => { engine.recreate_swapchain = true },
            Event::RedrawEventsCleared => {
                engine.previous_frame_end.as_mut().unwrap().cleanup_finished();

                if engine.recreate_swapchain {
                    let dims: [u32; 2] = engine.surface.window().inner_size().into();
                    let (new_swapchain, new_images) =
                        match engine.swapchain.recreate().dimensions(dims).build() {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => return,
                            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                        };

                        engine.swapchain = new_swapchain;
                        engine.framebuffers = Engine::window_size_dependent_setup(
                            &new_images,
                            engine.render_pass.clone(),
                            &mut engine.viewport,
                        );
                        engine.recreate_swapchain = false;
                }

                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(engine.swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            engine.recreate_swapchain = true;
                            return;
                        },
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                if suboptimal {
                    engine.recreate_swapchain = true;
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

                let future = engine.previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(engine.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(engine.queue.clone(), engine.swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        engine.previous_frame_end = Some(future.boxed());
                    },
                    Err(FlushError::OutOfDate) => {
                        engine.recreate_swapchain = true;
                        engine.previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                    },
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        engine.previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                    }
                }
            }
            _ => (),
        }
    });
}
