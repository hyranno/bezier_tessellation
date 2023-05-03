#version 450

layout(triangles, equal_spacing, cw) in;

layout(location = 0) patch in vec3 edge_normals[3][2];

layout(location = 0) out vec3 v_normal;

layout(push_constant) uniform PushConstantData {
    mat4 view;
} pc;



vec3 quartic_bezier_triangle(
    vec3 c400, vec3 c040, vec3 c004,
    vec3 c310, vec3 c220, vec3 c130,
    vec3 c031, vec3 c022, vec3 c013,
    vec3 c103, vec3 c202, vec3 c301,
    vec3 c211, vec3 c121, vec3 c112,
    float t0, float t1, float t2
) {
    return(
        +c400 *pow(t0, 4)
        +c040 *pow(t1, 4)
        +c004 *pow(t2, 4)
        +4*c310 *pow(t0, 3)* pow(t1, 1)
        +6*c220 *pow(t0, 2) *pow(t1, 2)
        +4*c130 *pow(t0, 1) *pow(t1, 3)
        +4*c031 *pow(t1, 3)* pow(t2, 1)
        +6*c022 *pow(t1, 2) *pow(t2, 2)
        +4*c013 *pow(t1, 1) *pow(t2, 3)
        +4*c103 *pow(t2, 3)* pow(t0, 1)
        +6*c202 *pow(t2, 2) *pow(t0, 2)
        +4*c301 *pow(t2, 1) *pow(t0, 3)
        +12*c211 *pow(t0, 2) * pow(t1, 1) * pow(t2, 1)
        +12*c121 *pow(t0, 1) * pow(t1, 2) * pow(t2, 1)
        +12*c112 *pow(t0, 1) * pow(t1, 1) * pow(t2, 2)
    );
}

void calc_curve_control_points(
    vec3 v0, vec3 v1,
    vec3 n0, vec3 n1,
    out vec3 c31, out vec3 c22, out vec3 c13
) {
    float K = 1.0;
    float Kc = 1.0;
    c31 = 1.0/4.0 * (4*v0 + K*((v1-v0)-n0*dot(v1-v0, n0)));
    c13 = 1.0/4.0 * (4*v1 + K*((v0-v1)-n1*dot(v0-v1, n1)));
    c22 = (v0+v1)/2 + Kc*((c31-v0) + (c13-v1));
}

vec3 pn_triangle(
    vec3 v0, vec3 v1, vec3 v2,
    vec3 n01, vec3 n10, vec3 n12, vec3 n21, vec3 n20, vec3 n02,
    float t0, float t1, float t2
) {
    vec3 c310, c220, c130;
    vec3 c031, c022, c013;
    vec3 c103, c202, c301;
    calc_curve_control_points(v0, v1, n01, n10, c310, c220, c130);
    calc_curve_control_points(v1, v2, n12, n21, c031, c022, c013);
    calc_curve_control_points(v2, v0, n20, n02, c103, c202, c301);
    return quartic_bezier_triangle(
        v0, v1, v2,
        c310, c220, c130,
        c031, c022, c013,
        c103, c202, c301,
        v0, v1, v2,
        t0, t1, t2
    );
}


void main(void) {
    vec3 v0 = gl_in[0].gl_Position.xyz;
    vec3 v1 = gl_in[1].gl_Position.xyz;
    vec3 v2 = gl_in[2].gl_Position.xyz;

    float r0 = gl_TessCoord.x;
    float r1 = gl_TessCoord.y;
    float r2 = gl_TessCoord.z;
    float rsum = r0 + r1 + r2;

    vec4 position = pc.view * vec4(
        pn_triangle(
            v0, v1, v2,
            edge_normals[0][0], edge_normals[0][1],
            edge_normals[1][0], edge_normals[1][1],
            edge_normals[2][0], edge_normals[2][1],
            r0/rsum, r1/rsum, r2/rsum
        ),
        1.0
    );
    gl_Position = position / position.w;

    /* TODO */
    v_normal = mat3(pc.view) * normalize(
        cross(
            gl_in[2].gl_Position.xyz - gl_in[0].gl_Position.xyz,
            gl_in[1].gl_Position.xyz - gl_in[0].gl_Position.xyz
        )
    );
}
