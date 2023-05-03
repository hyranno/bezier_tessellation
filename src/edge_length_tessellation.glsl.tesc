#version 450

layout(vertices = 3) out; 

layout(location = 0) in uint vertices[];

layout(location = 0) patch out vec3 edge_normals[3][2];

struct Edge {
    uint vertices[2];
    uint normals[2];
};
struct Face {
    uint edges[3];
};

layout(std430, binding = 0) buffer Normals {
    vec3 normals[];
} ssbo_normals;
layout(std430, binding = 1) buffer Edges {
    Edge edges[];
} ssbo_edges;
layout(std430, binding = 2) buffer Faces {
    Face faces[];
} ssbo_faces;

void main(void) {
    Face face = ssbo_faces.faces[gl_PrimitiveID];
    Edge edges[3] = Edge[](
        ssbo_edges.edges[face.edges[0]],
        ssbo_edges.edges[face.edges[1]],
        ssbo_edges.edges[face.edges[2]]
    );
    for (int i=0; i<3; i++) {
        int o = int(mix(1, 0, edges[i].vertices[0] == vertices[i]));
        edge_normals[i][0] = ssbo_normals.normals[edges[i].normals[o]];
        edge_normals[i][1] = ssbo_normals.normals[edges[i].normals[1-o]];
    }

    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

    /* TODO */
    gl_TessLevelInner[0] = 4;
    gl_TessLevelOuter[0] = 4;
    gl_TessLevelOuter[1] = 4;
    gl_TessLevelOuter[2] = 4;
}
