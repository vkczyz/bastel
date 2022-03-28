use std::sync::Arc;
use vulkano::device::Device;
use vulkano::shader::ShaderModule;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Shader {
    Solid,
    Rainbow,
    Texture,
}

pub fn get_shaders(shader: &Shader, device: &Arc<Device>) -> Vec<Arc<ShaderModule>> {
    match *shader {
        Shader::Solid => vec!(
            vs_solid::load(device.clone()).expect("Failed to create shader module"),
            fs_solid::load(device.clone()).expect("Failed to create shader module"),
        ),
        Shader::Rainbow => vec!(
            vs_rainbow::load(device.clone()).expect("Failed to create shader module"),
            fs_rainbow::load(device.clone()).expect("Failed to create shader module"),
        ),
        Shader::Texture => vec!(
            vs_texture::load(device.clone()).expect("Failed to create shader module"),
            fs_texture::load(device.clone()).expect("Failed to create shader module"),
        ),
    }
}

pub mod vs_solid {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "shaders/solid.vert",
    }
}

pub mod fs_solid {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "shaders/solid.frag",
    }
}

pub mod vs_rainbow {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "shaders/rainbow.vert",
    }
}

pub mod fs_rainbow {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "shaders/rainbow.frag",
    }
}

pub mod vs_texture {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "shaders/texture.vert",
    }
}

pub mod fs_texture {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "shaders/texture.frag",
    }
}