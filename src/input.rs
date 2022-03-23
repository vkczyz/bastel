use crate::Engine;
use crate::Vertex;
use winit::event::KeyboardInput;

pub fn handle_input(engine: &mut Engine, input: KeyboardInput) {
    if input.state != winit::event::ElementState::Pressed {
        return;
    }

    let units: [f32; 2] = [
        1.0 / engine.surface.window().inner_size().width as f32,
        1.0 / engine.surface.window().inner_size().height as f32,
        ];
    let speed: f32 = 10.0;
    let newnits = units.map(|u| u * speed);

    match input.scancode {
        // Clockwise arrow keys
        103 | 17 => {
            handle_movement(engine, 0.0, -newnits[1]);
        }
        106 | 32 => {
            handle_movement(engine, newnits[0], 0.0);
        },
        108 | 31 => {
            handle_movement(engine, 0.0, newnits[1]);
        },
        105 | 30 => {
            handle_movement(engine, -newnits[0], 0.0);
        },
        _ => {},
    }
}

pub fn handle_movement(engine: &mut Engine, x: f32, y: f32) {
    let old_vertices = match engine.pop_polygon() {
        Some(p) => p,
        None => { return; }
    };
    let old_vertices = old_vertices.read().unwrap();

    let new_vertices = old_vertices
        .iter()
        .map(|v| Vertex{ position: [v.position[0] + x, v.position[1] + y] })
        .collect();

    let vertex_buffer = Engine::create_polygon(new_vertices, &engine.device);
    engine.vertex_buffers.push(vertex_buffer);
}