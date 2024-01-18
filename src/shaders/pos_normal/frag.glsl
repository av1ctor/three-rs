#version 430
in vec3 color;

out vec4 out_color;

void main() {
    float c = color.x * 0.25 + color.y * 0.50 + color.z * 0.25;
    out_color = vec4(c, c, c, 1.0);
}