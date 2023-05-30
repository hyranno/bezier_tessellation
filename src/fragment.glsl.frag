#version 450

layout(location = 0) in vec3 v_normal;

layout(location = 0) out vec4 f_color;

const float almost_zero = 1e-8;
vec3 normalize_or_zero(vec3 v) {
    if (length(v) <= almost_zero) {
        return vec3(0);
    }
    return normalize(v);
}

void main() {
    vec3 normal = normalize_or_zero(v_normal);
    f_color = vec4(normal*0.5+0.5, 1.0);
    // f_color = vec4(gl_FragCoord.zzz, 1.0);
}
