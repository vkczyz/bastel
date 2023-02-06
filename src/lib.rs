pub mod engine;
pub mod entity;
pub mod scene;
pub mod components;
pub mod systems;
mod renderer;
mod shaders;
mod vertex;

use engine::Engine;
use scene::Scene;

use std::fs;
use std::ffi::CStr;
use std::os::raw::c_char;

use miniserde;
use miniserde::json;

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
#[cfg(feature = "json")]
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

    let scene: json::Value = json::from_str(
        fs::read_to_string(scene)
        .unwrap()
        .as_str()
    ).unwrap();

    let scene = Scene::from_json(&scene).unwrap();
    let (mut engine, event_loop) = Engine::new(title, width, height);
    engine.scene = scene;

    engine.run(event_loop);
}