#version 450

layout(location = 0) in vec3 position;

layout(push_constant) uniform PushConstantData {
    mat4 view;
} pc;

void main() {
    gl_Position = vec4(position, 1.0);
}
