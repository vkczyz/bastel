pub mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "shaders/rainbow.vert",
    }
}

pub mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "shaders/rainbow.frag",
    }
}
