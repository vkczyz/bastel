mod shaders;

use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::device::{
    physical::PhysicalDevice,
    Device,
    Features,
};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::image::{ImageAccess, ImageUsage, SwapchainImage};
use vulkano::command_buffer::SubpassContents;
use vulkano::image::view::ImageView;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::swapchain::{self, Swapchain, SwapchainCreationError, AcquireError};
use vulkano::Version;

use vulkano_win::create_vk_surface_from_handle;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

#[derive(Default, Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Framebuffer::start(render_pass.clone())
                .add(view)
                .unwrap()
                .build()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn main() {
    const TITLE: &str = "BASTEL";
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    let instance = Instance::new(
        None,
        Version::V1_1,
        &vulkano_win::required_extensions(),
        None,
    ).expect("Failed to create instance");

    let device_ext = vulkano::device::DeviceExtensions {
        khr_swapchain: true,
        .. vulkano::device::DeviceExtensions::none()
    };

    let physical = PhysicalDevice::enumerate(&instance)
        .filter(|&p| p.supported_extensions().is_superset_of(&device_ext))
        .next()
        .expect("No devices available");

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Couldn't find a graphical queue family");

    let (device, mut queues) =
        Device::new(
            physical,
            &Features::none(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned()).expect("Failed to create device");

    let queue = queues.next()
        .expect("Could not select queue");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(TITLE);
    let surface = create_vk_surface_from_handle(window, instance.clone()).unwrap();

    let caps = surface.capabilities(physical)
        .expect("Failed to get surface capabilities");

    let dims = caps.current_extent.unwrap_or([WIDTH, HEIGHT]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (mut swapchain, images) = Swapchain::start(device.clone(), surface.clone())
        .num_images(caps.min_image_count)
        .format(format)
        .dimensions(dims)
        .usage(ImageUsage::color_attachment())
        .sharing_mode(&queue)
        .composite_alpha(alpha)
        .build()
        .expect("Failed to create swapchain");

    vulkano::impl_vertex!(Vertex, position);

    let vertices = vec!(
        Vertex{ position: [-0.5, -0.5] },
        Vertex{ position: [0.5, -0.5] },
        Vertex{ position: [0.0, 0.5] },
    );

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        vertices.into_iter(),
    ).unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap();

    let vs = shaders::vs::load(device.clone())
        .expect("Failed to create shader module");
    let fs = shaders::fs::load(device.clone())
        .expect("Failed to create shader module");

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [WIDTH as f32, HEIGHT as f32],
        depth_range: 0.0..1.0,
    };

    let pipeline = GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };
    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);
    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    event_loop.run(move |event, _, control_flow| {
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
                    let dims: [u32; 2] = surface.window().inner_size().into();
                    let (new_swapchain, new_images) =
                        match swapchain.recreate().dimensions(dims).build() {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => return,
                            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                        };

                        swapchain = new_swapchain;
                        framebuffers = window_size_dependent_setup(
                            &new_images,
                            render_pass.clone(),
                            &mut viewport,
                        );
                        recreate_swapchain = false;
                }

                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None) {
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
                    device.clone(),
                    queue.family(),
                    CommandBufferUsage::OneTimeSubmit,
                ).unwrap();

                builder
                    .begin_render_pass(
                        framebuffers[image_num].clone(),
                        SubpassContents::Inline,
                        clear_values,
                    )
                    .unwrap()
                    .set_viewport(0, [viewport.clone()])
                    .bind_pipeline_graphics(pipeline.clone())
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass()
                    .unwrap();

                let command_buffer = builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    },
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    },
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                }
            }
            _ => (),
        }
    });
}
