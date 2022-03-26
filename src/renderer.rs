use crate::shaders;
use crate::vertex::Vertex;

use std::sync::Arc;
use std::collections::HashMap;

use vulkano::instance::Instance;
use vulkano::device::{
    Device,
    Features,
    QueuesIter,
};
use vulkano::device::physical::{
    PhysicalDevice,
    PhysicalDeviceType,
};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::image::ImageUsage;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{self, AcquireError, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreationError};
use vulkano::device::Queue;
use vulkano::Version;

use vulkano_win::create_vk_surface_from_handle;

use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use vulkano::image::{ImageAccess, SwapchainImage};
use vulkano::image::view::ImageView;
use vulkano::render_pass::{Framebuffer, RenderPass};
use winit::window::Window;

pub struct Renderer {
    pub surface: Arc<Surface<Window>>,
    pub swapchain: Arc<Swapchain<Window>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub viewport: Viewport,
    pub render_pass: Arc<RenderPass>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub shaders: HashMap<String, Arc<ShaderModule>>,
}

impl Renderer {
    fn get_instance() -> Arc<Instance> {
        let instance = Instance::new(
            None,
            Version::V1_1,
            &vulkano_win::required_extensions(),
            None,
        ).expect("Failed to create instance");

        instance
    }

    fn get_surface(event_loop: &EventLoop<()>, instance: &Arc<Instance>, title: &str) -> Arc<Surface<Window>> {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        window.set_title(title);
        let surface = create_vk_surface_from_handle(window, instance.clone()).unwrap();

        surface
    }

    fn get_device_and_queues(instance: &Arc<Instance>) -> (Arc<Device>, QueuesIter) {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        let (physical, queue_family) = PhysicalDevice::enumerate(instance)
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

        let (device, queues) =
            Device::new(
                physical,
                &Features::none(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned()).expect("Failed to create device");

        (device, queues)
    }

    fn get_swapchain(surface: &Arc<Surface<Window>>, device: &Arc<Device>, queue: &Arc<Queue>, width: &u32, height: &u32) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
        let caps = surface.capabilities(device.physical_device())
            .expect("Failed to get surface capabilities");

        let dims = caps.current_extent.unwrap_or([*width, *height]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        let (swapchain, images) = Swapchain::start(device.clone(), surface.clone())
            .num_images(caps.min_image_count)
            .format(format)
            .dimensions(dims)
            .usage(ImageUsage::color_attachment())
            .sharing_mode(queue)
            .composite_alpha(alpha)
            .build()
            .expect("Failed to create swapchain");

        (swapchain, images)
    }

    pub fn create_vertex_buffer(vertices: Vec<Vertex>, device: &Arc<Device>) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            vertices.into_iter(),
        ).expect("Failed to create buffer");

        vertex_buffer
    }

    pub fn recreate_swapchain(&mut self) -> Result<(), ()> {
        let dims: [u32; 2] = self.surface.window().inner_size().into();
        let (swapchain, images) =
            match self.swapchain.recreate().dimensions(dims).build() {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => return Err(()),
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
            };

        self.swapchain = swapchain;
        self.framebuffers = Renderer::window_size_dependent_setup(
            &images,
            self.render_pass.clone(),
            &mut self.viewport,
        );

        Ok(())
    }

    pub fn recreate_pipeline(&mut self) -> Result<(), ()> {
        let viewport = self.viewport.clone();
        let vs = self.shaders.get("vs").unwrap();
        let fs = self.shaders.get("fs").unwrap();

        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .render_pass(Subpass::from(self.render_pass.clone(), 0).unwrap())
            .build(self.device.clone())
            .unwrap();
        self.pipeline = pipeline;

        Ok(())
    }

    pub fn acquire_next_image(&self) -> Result<(usize, bool, SwapchainAcquireFuture<Window>), ()> {
        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    return Err(());
                },
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        Ok((image_num, suboptimal, acquire_future))
    }

    pub fn init(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let instance = Renderer::get_instance();
        let event_loop = EventLoop::new();
        let surface = Renderer::get_surface(&event_loop, &instance, title);

        let (device, mut queues) = Renderer::get_device_and_queues(&instance);
        let queue = queues.next()
            .expect("Could not select queue");

        let (swapchain, images) = Renderer::get_swapchain(&surface, &device, &queue, &width, &height);

        vulkano::impl_vertex!(Vertex, position);

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
        ).expect("Failed to create render pass");

        let shaders = HashMap::from([
        (String::from("vs"), shaders::vs::load(device.clone()).expect("Failed to create shader module")),
        (String::from("fs"), shaders::fs::load(device.clone()).expect("Failed to create shader module")),
        ]);

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [width as f32, height as f32],
            depth_range: 0.0..1.0,
        };

        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .vertex_shader(shaders.get("vs").unwrap().entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
            .fragment_shader(shaders.get("fs").unwrap().entry_point("main").unwrap(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap();

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };
        let framebuffers = Renderer::window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

        (Renderer {
            surface,
            swapchain,
            framebuffers,
            viewport,
            render_pass,
            device,
            queue,
            pipeline,
            shaders,
        }, event_loop)
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
