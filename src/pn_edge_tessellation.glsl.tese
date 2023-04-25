#version 450

layout(triangles, equal_spacing, cw) in;

layout(location = 0) out vec3 v_normal;

layout(push_constant) uniform PushConstantData {
    mat4 view;
} pc;

void main(void) {
    // Retrieve the vertex positions set by the TCS.
    vec4 vert_x = gl_in[0].gl_Position;
    vec4 vert_y = gl_in[1].gl_Position;
    vec4 vert_z = gl_in[2].gl_Position;

    // Convert `gl_TessCoord` from Barycentric coordinates to Cartesian coordinates.
    vec4 position = pc.view * vec4(
        gl_TessCoord.x * vert_x.x + gl_TessCoord.y * vert_y.x + gl_TessCoord.z * vert_z.x,
        gl_TessCoord.x * vert_x.y + gl_TessCoord.y * vert_y.y + gl_TessCoord.z * vert_z.y,
        gl_TessCoord.x * vert_x.z + gl_TessCoord.y * vert_y.z + gl_TessCoord.z * vert_z.z,
        1.0
    );
    gl_Position = position / position.w;

    v_normal = mat3(pc.view) * normalize(
        cross(
            gl_in[2].gl_Position.xyz - gl_in[0].gl_Position.xyz,
            gl_in[1].gl_Position.xyz - gl_in[0].gl_Position.xyz
        )
    );
}
