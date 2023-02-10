pub mod engine;
pub mod global;
pub mod scene;
pub mod entity;
pub mod components;
pub mod systems;
mod renderer;
mod shaders;
mod vertex;

use engine::Engine;
use scene::Scene;

use std::ffi::CStr;
use std::fs;
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

#[no_mangle]
pub extern "C" fn init_with_scene(title: *const c_char, width: u32, height: u32, scene: *const c_char) {
    let title = unsafe {
        CStr::from_ptr(title)
            .to_str()
            .expect("Failed to decode title")
    };

    let scene = unsafe {
        CStr::from_ptr(scene)
            .to_str()
            .expect("Failed to decode title")
    };
    let scene = fs::read_to_string(scene).expect("Failed to find scene file");
    let scene = Scene::from_xml(&scene);

    let (mut engine, event_loop) = Engine::new(title, width, height);
    engine.scene = scene;

    engine.run(event_loop);
}