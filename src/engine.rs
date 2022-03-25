use crate::renderer::Renderer;
use crate::input::Input;

use winit::event_loop::EventLoop;

pub struct Engine {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fps: u64,
    pub renderer: Renderer,
    pub event_loop: EventLoop<()>,
    pub input: Input,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let (renderer, event_loop) = Renderer::init(title, width, height);
        let input = Input::new();
        let fps = 60;

        Engine {
            title: String::from(title),
            width,
            height,
            fps,
            renderer,
            event_loop,
            input,
        }
    }
}