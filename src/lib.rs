pub mod engine;
pub mod entity;
pub mod scene;
pub mod sprite;
mod physics;
mod renderer;
mod shaders;
mod vertex;
mod input;

use engine::Engine;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn init(title: *const c_char, width: u32, height: u32) {
    let title = unsafe {
        CStr::from_ptr(title)
            .to_str()
            .expect("Failed to decode title")
    };

    let (engine, event_loop) = Engine::new(title, width, height);
    engine.run(event_loop);
}