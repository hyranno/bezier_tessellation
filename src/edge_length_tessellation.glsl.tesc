#version 450

layout(vertices = 3) out; 

layout(location = 0) in uint vertices[];

layout(location = 0) patch out vec4 edge_control_points[3][2];

struct Edge {
    uint vertices[2];
    vec4 control_points[2];
};
struct Face {
    uint edges[3];
};

layout(std430, binding = 0) buffer Edges {
    Edge edges[];
} ssbo_edges;
layout(std430, binding = 1) buffer Faces {
    Face faces[];
} ssbo_faces;

/*
    Do some edge based calculation.
    Should use LOD, screen space length, curviness or something.
    We here use just the length for simple.
*/
int edge_length_based_tess_level(vec3 position0, vec3 position1, vec3 control_point0, vec3 control_point1) {
    return int(max(1.0, round(distance(position0, position1) * 8)));
}

void main(void) {
    if (gl_InvocationID == 0) {
        vec3 positions[3] = vec3[](
            gl_in[0].gl_Position.xyz,
            gl_in[1].gl_Position.xyz,
            gl_in[2].gl_Position.xyz
        );
        Face face = ssbo_faces.faces[gl_PrimitiveID];
        Edge edges[3] = Edge[](
            ssbo_edges.edges[face.edges[0]],
            ssbo_edges.edges[face.edges[1]],
            ssbo_edges.edges[face.edges[2]]
        );

        int tess_level_outer[3] = int[](0, 0, 0);
        for (int i=0; i<3; i++) {
            int o = int(mix(1, 0, edges[i].vertices[0] == vertices[i]));
            vec4 control_point0 = edges[i].control_points[o];
            vec4 control_point1 = edges[i].control_points[1-o];
            tess_level_outer[i] = edge_length_based_tess_level(positions[i], positions[(i+1)%3], control_point0.xyz, control_point1.xyz);
            edge_control_points[i][0] = control_point0;
            edge_control_points[i][1] = control_point1;
            gl_TessLevelOuter[(i+2)%3] = tess_level_outer[i];
        }
        gl_TessLevelInner[0] = round((tess_level_outer[0] + tess_level_outer[1] + tess_level_outer[2]) / 3.0);
    }

    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
}
