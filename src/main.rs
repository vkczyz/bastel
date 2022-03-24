use bastel;
use bastel::engine::Engine;

fn main() {
    const TITLE: &str = "BASTEL";
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const FPS: u64 = 60;

    let (engine, event_loop) = Engine::init(TITLE, WIDTH, HEIGHT);
    bastel::begin_loop(engine, event_loop, FPS);
}