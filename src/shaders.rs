pub mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "shaders/vertex.glsl",
    }
}

pub mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "shaders/fragment.glsl",
    }
}
