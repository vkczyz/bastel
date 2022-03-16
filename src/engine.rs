use crate::shaders;

use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::device::{
    Device,
    Features,
};
use vulkano::device::physical::{
    PhysicalDevice,
    PhysicalDeviceType,
};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::sync;
use vulkano::sync::GpuFuture;
use vulkano::image::ImageUsage;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;
use vulkano::swapchain::{Surface, Swapchain};
use vulkano::device::Queue;
use vulkano::Version;

use vulkano_win::create_vk_surface_from_handle;

use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use vulkano::image::{ImageAccess, SwapchainImage};
use vulkano::image::view::ImageView;
use vulkano::render_pass::{Framebuffer, RenderPass};
use winit::window::Window;

#[derive(Default, Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

pub struct Engine {
    pub event_loop: EventLoop<()>,
    pub surface: Arc<Surface<Window>>,
    pub swapchain: Arc<Swapchain<Window>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub viewport: Viewport,
    pub render_pass: Arc<RenderPass>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

impl Engine {
    pub fn init(title: &str, width: u32, height: u32) -> Self {
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

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        window.set_title(title);
        let surface = create_vk_surface_from_handle(window, instance.clone()).unwrap();

        let (physical, queue_family) = PhysicalDevice::enumerate(&instance)
            .filter(|&p| p.supported_extensions().is_superset_of(&device_ext))
            .filter_map(|p| {
                p.queue_families()
                    .find(|&q| q.supports_graphics())
                .map(|q| (p, q))
            })
            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                }
            })
            .expect("No devices available");

        let (device, mut queues) =
            Device::new(
                physical,
                &Features::none(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned()).expect("Failed to create device");

        let queue = queues.next()
            .expect("Could not select queue");

        let caps = surface.capabilities(physical)
            .expect("Failed to get surface capabilities");

        let dims = caps.current_extent.unwrap_or([width, height]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        let (swapchain, images) = Swapchain::start(device.clone(), surface.clone())
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
            dimensions: [width as f32, height as f32],
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
        let framebuffers = Engine::window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);
        let recreate_swapchain = false;
        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        return Engine {
            event_loop,
            surface,
            swapchain,
            framebuffers,
            recreate_swapchain,
            previous_frame_end,
            viewport,
            render_pass,
            device,
            queue,
            pipeline,
            vertex_buffer,
        }
    }

    pub fn window_size_dependent_setup(
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
}