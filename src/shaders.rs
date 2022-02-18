pub mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
            #version 450

            layout(location = 0) in vec2 position;
            layout(location = 1) out vec4 vertex_color;

            vec4 colors[3] = vec4[3](
                vec4(1.0, 0.0, 0.0, 1.0),
                vec4(0.0, 1.0, 0.0, 1.0),
                vec4(0.0, 0.0, 1.0, 1.0)
            );

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                vertex_color = colors[gl_VertexIndex];
            }"
    }
}

pub mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
            #version 450

            layout(location = 0) out vec4 f_color;
            layout(location = 1) in vec4 vertex_color;

            void main() {
                f_color = vertex_color;
            }"
    }
}
