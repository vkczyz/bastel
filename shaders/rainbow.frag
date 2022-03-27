#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec4 vertex_color;

void main() {
    f_color = vertex_color;
}