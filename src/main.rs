use bastel;
use bastel::renderer::Renderer;

fn main() {
    const TITLE: &str = "BASTEL";
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const FPS: u64 = 60;

    let (renderer, event_loop) = Renderer::init(TITLE, WIDTH, HEIGHT);
    bastel::begin_loop(renderer, event_loop, FPS);
}