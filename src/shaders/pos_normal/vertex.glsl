#version 430
layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_normal;

uniform mat4 projection;
uniform mat4 model_view;

out vec3 color;

void main() {
    gl_Position = projection * model_view * vec4(in_position, 1.0);
    color = in_normal * 0.5 + 0.5;
}