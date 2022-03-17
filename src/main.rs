use bastel;
use bastel::engine::Engine;


fn main() {
    const TITLE: &str = "BASTEL";
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const FPS: u64 = 60;

    let engine = Engine::init(TITLE, WIDTH, HEIGHT);

    bastel::begin_loop(engine, FPS);
}