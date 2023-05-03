#version 450

layout(location = 0) in vec3 position;

layout(location = 0) out uint vertex_id;

layout(push_constant) uniform PushConstantData {
    mat4 view;
} pc;

void main() {
    vertex_id = gl_VertexIndex;
    gl_Position = vec4(position, 1.0);
}
