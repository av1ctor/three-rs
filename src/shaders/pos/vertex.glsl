#version 430
layout (location = 0) in vec3 in_position;

uniform mat4 projection;
uniform mat4 model_view;

void main() {
    gl_Position = projection * model_view * vec4(in_position, 1.0);
}