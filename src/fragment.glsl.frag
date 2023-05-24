#version 450

layout(location = 0) in vec3 v_normal;

layout(location = 0) out vec4 f_color;

void main() {
    vec3 normal = normalize(v_normal);
    f_color = vec4(normal*0.5+0.5, 1.0);
    // f_color = vec4(gl_FragCoord.zzz, 1.0);
}
